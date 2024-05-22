use std::ops::{Index, IndexMut};

pub use advice_entry::AdviceEntry;
use TraceItem::*;

use crate::{
    constants::INITIAL_HASH_WORDS,
    table::{indices::*, ADVICE_COLUMNS, NUM_ROWS, ROWS_PER_ROUND},
    trace::{Trace, TraceItem},
    types::{decompose, Bitem},
};

mod advice_entry;

type AdviceArea = [[AdviceEntry; NUM_ROWS]; ADVICE_COLUMNS];

#[derive(Clone)]
pub struct Advice {
    columns: AdviceArea,
}

impl Index<usize> for Advice {
    type Output = [AdviceEntry; NUM_ROWS];

    fn index(&self, index: usize) -> &Self::Output {
        &self.columns[index]
    }
}

impl IndexMut<usize> for Advice {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.columns[index]
    }
}

impl Advice {
    pub fn from_trace(trace: &Trace) -> Self {
        Self {
            columns: [[AdviceEntry::Mpty; NUM_ROWS]; ADVICE_COLUMNS],
        }
        .fill_auxiliary_rows(&trace)
        .fill_round_rows(&trace)
    }

    fn fill_auxiliary_rows(mut self, trace: &Trace) -> Self {
        for round in 0..16 {
            let row = (round) * ROWS_PER_ROUND; // penultimate row of each round
            self[W][row] = trace[w][round].word.into();

            if round == 0 {
                [self[AX][row], self[AY][row], self[AZ][row]] =
                    decompose(&INITIAL_HASH_WORDS[0]).map(Into::into);
                [self[EX][row], self[EY][row], self[EZ][row]] =
                    decompose(&INITIAL_HASH_WORDS[4]).map(Into::into);
            } else {
                [self[AX][row], self[AY][row], self[AZ][row]] =
                    trace[a][round - 1].limbs.map(Into::into);
                [self[EX][row], self[EY][row], self[EZ][row]] =
                    trace[e][round - 1].limbs.map(Into::into);
            }
        }
        self
    }

    fn fill_round_rows(mut self, trace: &Trace) -> Self {
        const OFFSET: usize = 16 * ROWS_PER_ROUND;

        self.fill_first_round_input(OFFSET);

        for round in 0..64 {
            let row = OFFSET + round * ROWS_PER_ROUND;
            let trace_access = |col| trace[col][round];

            self.fill_round_lookups(row + 1, trace_access);
            self.fill_round_compositions(row + 2, trace_access);
            self.fill_round_additions(row + 3, trace_access);
            self.fill_round_decompositions(row + 4, trace_access);
            self.fill_witness_computation(row, trace_access);
        }

        self
    }

    fn fill(&mut self, row: usize, assignment: impl IntoIterator<Item = (usize, AdviceEntry)>) {
        for (col, value) in assignment {
            self[col][row] = value;
        }
    }

    fn fill_first_round_input(&mut self, row: usize) {
        let a_limbs = decompose(&INITIAL_HASH_WORDS[0]);
        let e_limbs = decompose(&INITIAL_HASH_WORDS[4]);
        self.fill(
            row,
            [
                (AX, a_limbs[0].into()),
                (AY, a_limbs[1].into()),
                (AZ, a_limbs[2].into()),
                (EX, e_limbs[0].into()),
                (EY, e_limbs[1].into()),
                (EZ, e_limbs[2].into()),
            ],
        );
    }

    fn fill_round_lookups(&mut self, row: usize, trace: impl Fn(TraceItem) -> Bitem) {
        self.fill(
            row,
            [
                (ROT0, trace(rot0).word.into()),
                (ROT1, trace(rot1).word.into()),
                (MAJ_X, trace(maj).limbs[0].into()),
                (MAJ_Y, trace(maj).limbs[1].into()),
                (MAJ_Z, trace(maj).limbs[2].into()),
                (CH_X, trace(ch).limbs[0].into()),
                (CH_Y, trace(ch).limbs[1].into()),
                (CH_Z, trace(ch).limbs[2].into()),
            ],
        );
    }

    fn fill_round_compositions(&mut self, row: usize, trace: impl Fn(TraceItem) -> Bitem) {
        self.fill(
            row,
            [
                (MAJ, trace(maj).word.into()),
                (CH, trace(ch).word.into()),
                (D, trace(d).word.into()),
                (H, trace(h).word.into()),
            ],
        )
    }

    fn fill_round_additions(&mut self, row: usize, trace: impl Fn(TraceItem) -> Bitem) {
        self.fill(row, [(A, trace(a).full.into()), (E, trace(e).full.into())])
    }

    fn fill_round_decompositions(&mut self, row: usize, trace: impl Fn(TraceItem) -> Bitem) {
        self.fill(
            row,
            [
                (AX, trace(a).limbs[0].into()),
                (AY, trace(a).limbs[1].into()),
                (AZ, trace(a).limbs[2].into()),
                (EX, trace(e).limbs[0].into()),
                (EY, trace(e).limbs[1].into()),
                (EZ, trace(e).limbs[2].into()),
            ],
        )
    }

    fn fill_witness_computation(&mut self, row: usize, trace: impl Fn(TraceItem) -> Bitem) {
        self.fill(row, [(W, trace(w).word.into())])
    }
}
