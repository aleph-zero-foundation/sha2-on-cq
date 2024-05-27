use std::ops::{Index, IndexMut};

pub use advice_entry::AdviceEntry;
use TraceItem::*;

use crate::{
    constants::INITIAL_HASH_WORDS,
    sha_ops::{witness_op1, witness_op2},
    table::{indices::*, ADVICE_COLUMNS, INITIAL_BUFFER, NUM_ROWS, ROWS_PER_ROUND},
    trace::{Trace, TraceItem},
    types::{decompose, Bitem, WordSum},
    ROUNDS,
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
        .fill_auxiliary_rows()
        .fill_round_rows(&trace)
    }

    fn fill_auxiliary_rows(mut self) -> Self {
        [
            self[BX][2 * ROWS_PER_ROUND],
            self[BY][2 * ROWS_PER_ROUND],
            self[BZ][2 * ROWS_PER_ROUND],
        ] = decompose(&INITIAL_HASH_WORDS[1]).map(Into::into);
        [
            self[FX][2 * ROWS_PER_ROUND],
            self[FY][2 * ROWS_PER_ROUND],
            self[FZ][2 * ROWS_PER_ROUND],
        ] = decompose(&INITIAL_HASH_WORDS[5]).map(Into::into);

        [
            self[CX][1 * ROWS_PER_ROUND],
            self[CY][1 * ROWS_PER_ROUND],
            self[CZ][1 * ROWS_PER_ROUND],
        ] = decompose(&INITIAL_HASH_WORDS[2]).map(Into::into);
        [
            self[GX][1 * ROWS_PER_ROUND],
            self[GY][1 * ROWS_PER_ROUND],
            self[GZ][1 * ROWS_PER_ROUND],
        ] = decompose(&INITIAL_HASH_WORDS[6]).map(Into::into);

        [self[DX][0], self[DY][0], self[DZ][0]] = decompose(&INITIAL_HASH_WORDS[3]).map(Into::into);
        [self[HX][0], self[HY][0], self[HZ][0]] = decompose(&INITIAL_HASH_WORDS[7]).map(Into::into);

        self
    }

    fn fill_round_rows(mut self, trace: &Trace) -> Self {
        const OFFSET: usize = INITIAL_BUFFER * ROWS_PER_ROUND;

        self.fill_first_round_input(OFFSET);

        for round in 0..ROUNDS {
            let row = OFFSET + round * ROWS_PER_ROUND;
            let trace_access = |col| trace[col][round];

            self.fill_round_lookups(row + 1, trace_access);
            self.fill_round_compositions(row + 2, trace_access);
            self.fill_round_additions(row + 3, trace_access);
            self.fill_round_decompositions(row + 4, trace_access);
            self.fill_witness(row, trace_access);
            if round > 15 {
                self.fill_witness_computation(row, round, trace);
            }
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
        self.fill(
            row + 1,
            [
                (W1, witness_op1(trace(w).word).into()),
                (W2, witness_op2(trace(w).word).into()),
            ],
        );
    }

    fn fill_round_compositions(&mut self, row: usize, trace: impl Fn(TraceItem) -> Bitem) {
        self.fill(
            row,
            [
                (MAJ, trace(maj).word.into()),
                (CH, trace(ch).word.into()),
                (D, trace(d_in).word.into()),
                (H, trace(h_in).word.into()),
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

    fn fill_witness(&mut self, row: usize, trace: impl Fn(TraceItem) -> Bitem) {
        self.fill(row, [(W, trace(w).word.into())])
    }

    fn fill_witness_computation(&mut self, row: usize, round: usize, trace: &Trace) {
        let get_w = |round: usize| trace[w][round].word;

        self.fill(
            row,
            [(
                W_SUM,
                (witness_op1(get_w(round - 2)) as WordSum
                    + get_w(round - 7) as WordSum
                    + witness_op2(get_w(round - 15)) as WordSum
                    + get_w(round - 16) as WordSum)
                    .into(),
            )],
        )
    }
}
