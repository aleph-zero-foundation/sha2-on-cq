use std::ops::Index;

use TraceItem::*;

use crate::{
    table::indices::*,
    trace::{Trace, TraceItem},
    types::Bitem,
};
use crate::table::advice::advice_entry::AdviceEntry;

mod advice_entry;

pub const ADVICE_COLUMNS: usize = 9;
pub const ROWS_PER_ROUND: usize = 4;
pub const NUM_ROWS: usize = (16 + 64) * ROWS_PER_ROUND;
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

impl Advice {
    pub fn new(trace: &Trace) -> Self {
        let mut columns = [[AdviceEntry::Mpty; NUM_ROWS]; ADVICE_COLUMNS];

        Self::fill_auxiliary_rows(&mut columns, &trace);
        Self::fill_round_rows(&mut columns, &trace);

        Self { columns }
    }

    fn fill_auxiliary_rows(columns: &mut AdviceArea, trace: &Trace) {
        for round in 0..16 {
            let row = (round + 1) * ROWS_PER_ROUND - 1;
            columns[W][row] = trace[w][round].word.into();

            [columns[AX][row], columns[AY][row], columns[AZ][row]] =
                trace[a][round].limbs.map(Into::into);
            [columns[EX][row], columns[EY][row], columns[EZ][row]] =
                trace[e][round].limbs.map(Into::into);
        }
    }

    fn fill_round_rows(columns: &mut AdviceArea, trace: &Trace) {
        const OFFSET: usize = 16 * ROWS_PER_ROUND;

        for round in 0..64 {
            let row = OFFSET + round * ROWS_PER_ROUND;
            let trace_access = |col| trace[col][round];

            Self::fill_round_lookups(|col, value| columns[col][row] = value, trace_access);
            Self::fill_round_compositions(|col, value| columns[col][row + 1] = value, trace_access);
            Self::fill_round_additions(|col, value| columns[col][row + 2] = value, trace_access);
            Self::fill_round_decompositions(
                |col, value| columns[col][row + 3] = value,
                trace_access,
            );
        }
    }

    fn fill_round_lookups(
        mut fill: impl FnMut(usize, AdviceEntry),
        trace: impl Fn(TraceItem) -> Bitem,
    ) {
        fill(ROT0, trace(rot0).word.into());
        fill(ROT1, trace(rot1).word.into());

        fill(MAJ_X, trace(maj).limbs[0].into());
        fill(MAJ_Y, trace(maj).limbs[1].into());
        fill(MAJ_Z, trace(maj).limbs[2].into());

        fill(CH_X, trace(ch).limbs[0].into());
        fill(CH_Y, trace(ch).limbs[1].into());
        fill(CH_Z, trace(ch).limbs[2].into());

        fill(W, trace(w).word.into());
    }

    fn fill_round_compositions(
        mut fill: impl FnMut(usize, AdviceEntry),
        trace: impl Fn(TraceItem) -> Bitem,
    ) {
        fill(MAJ, trace(maj).word.into());
        fill(CH, trace(ch).word.into());
        fill(D, trace(d).word.into());
        fill(H, trace(h).word.into());
    }

    fn fill_round_additions(
        mut fill: impl FnMut(usize, AdviceEntry),
        trace: impl Fn(TraceItem) -> Bitem,
    ) {
        // todo: trace should contain u37 and two new variants A and E
        fill(A, trace(a).word.into());
        fill(E, trace(e).word.into());
    }

    fn fill_round_decompositions(
        mut fill: impl FnMut(usize, AdviceEntry),
        trace: impl Fn(TraceItem) -> Bitem,
    ) {
        fill(AX, trace(a).limbs[0].into());
        fill(AY, trace(a).limbs[1].into());
        fill(AZ, trace(a).limbs[2].into());

        fill(EX, trace(e).limbs[0].into());
        fill(EY, trace(e).limbs[1].into());
        fill(EZ, trace(e).limbs[2].into());
    }
}
