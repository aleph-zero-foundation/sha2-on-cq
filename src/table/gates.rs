use std::fmt::Debug;

use crate::{
    lookup_tables::{LookupTable, TCh, TDec, TMaj, TMod, TRot0, TRot1, TW1, TW2},
    table::{
        advice::AdviceEntry,
        fixed::Selector::{
            Addition, Composition, Decomposition, Lookup, ResultVerification, WitnessComputation,
        },
        indices::*,
        Table, ROWS_PER_ROUND,
    },
    types::{compose, WordSum},
};

macro_rules! helpers {
    ($table:ident, $row:ident) => {
        (
            |col1: usize, col2: usize, col3: usize, offset: isize| {
                [
                    $table.advice[col1][($row as isize + offset) as usize].limb(),
                    $table.advice[col2][($row as isize + offset) as usize].limb(),
                    $table.advice[col3][($row as isize + offset) as usize].limb(),
                ]
            },
            |col: usize, offset: isize| {
                $table.advice[col][($row as isize + offset) as usize].word()
            },
            |col: usize, offset: isize| {
                $table.advice[col][($row as isize + offset) as usize].word_sum()
            },
            |col: usize, item: &str, exp: AdviceEntry| {
                assert_eq!(
                    exp,
                    $table.advice[col][$row + 1],
                    "{item} mismatch at row {}",
                    $row
                );
            },
        )
    };
}

macro_rules! enable_for {
    ($table:ident, $selector:expr, $row:ident) => {
        if !$table.fixed_part.is_enabled($selector, $row) {
            return;
        }
    };
}

const ROUND_BACK: isize = -(ROWS_PER_ROUND as isize);

pub trait Gate {
    fn check(table: &Table, row: usize);
}

pub struct LookupGate;
impl Gate for LookupGate {
    fn check(table: &Table, row: usize) {
        enable_for!(table, Lookup, row);
        let (get_limbs, get_word, _, check) = helpers!(table, row);

        let [ax, ay, az] = get_limbs(AX, AY, AZ, 0);
        let [ex, ey, ez] = get_limbs(EX, EY, EZ, 0);

        let [bx, by, bz] = get_limbs(BX, BY, BZ, ROUND_BACK);
        let [fx, fy, fz] = get_limbs(FX, FY, FZ, ROUND_BACK);

        let [cx, cy, cz] = get_limbs(CX, CY, CZ, 2 * ROUND_BACK);
        let [gx, gy, gz] = get_limbs(GX, GY, GZ, 2 * ROUND_BACK);

        let [w, w1, w2] = [get_word(W, 0), get_word(W1, 2), get_word(W2, 2)];

        check(ROT0, "rot0", TRot0::lookup([ax, ay, az]).into());
        check(ROT1, "rot1", TRot1::lookup([ex, ey, ez]).into());
        check(MAJ_X, "maj_x", TMaj::lookup([ax, bx, cx]).into());
        check(MAJ_Y, "maj_y", TMaj::lookup([ay, by, cy]).into());
        check(MAJ_Z, "maj_z", TMaj::lookup([az, bz, cz]).into());
        check(CH_X, "ch_x", TCh::lookup([ex, fx, gx]).into());
        check(CH_Y, "ch_y", TCh::lookup([ey, fy, gy]).into());
        check(CH_Z, "ch_z", TCh::lookup([ez, fz, gz]).into());
        assert_eq!(w1, TW1::lookup(w), "w1 mismatch at row {row}");
        assert_eq!(w2, TW2::lookup(w), "w1 mismatch at row {row}");
    }
}

pub struct CompositionGate;
impl Gate for CompositionGate {
    fn check(table: &Table, row: usize) {
        enable_for!(table, Composition, row);
        let (get_limbs, _, _, check) = helpers!(table, row);

        let [maj_x, maj_y, maj_z] = get_limbs(MAJ_X, MAJ_Y, MAJ_Z, 0);
        let [ch_x, ch_y, ch_z] = get_limbs(CH_X, CH_Y, CH_Z, 0);
        let [dx, dy, dz] = get_limbs(DX, DY, DZ, 3 * ROUND_BACK - 1);
        let [hx, hy, hz] = get_limbs(HX, HY, HZ, 3 * ROUND_BACK - 1);

        check(MAJ, "maj", compose(&[maj_x, maj_y, maj_z]).into());
        check(CH, "ch", compose(&[ch_x, ch_y, ch_z]).into());
        check(D, "d", compose(&[dx, dy, dz]).into());
        check(H, "h", compose(&[hx, hy, hz]).into());
    }
}

pub struct AdditionGate;
impl Gate for AdditionGate {
    fn check(table: &Table, row: usize) {
        enable_for!(table, Addition, row);
        let (_, get_word, _, check) = helpers!(table, row);

        let [maj, ch] = [get_word(MAJ, 0) as WordSum, get_word(CH, 0) as WordSum];
        let [d, h] = [get_word(D, 0) as WordSum, get_word(H, 0) as WordSum];
        let [rot0, rot1] = [get_word(ROT0, -1) as WordSum, get_word(ROT1, -1) as WordSum];
        let k = table.fixed_part.constants[row] as WordSum;
        let w = get_word(W, -2) as WordSum;

        let temp1 = h + rot1 + ch + k + w;
        let temp2 = rot0 + maj;

        let exp_a = temp1 + temp2;
        let exp_e = temp1 + d;

        check(A, "A", exp_a.into());
        check(E, "E", exp_e.into());
    }
}

pub struct DecompositionGate;
impl Gate for DecompositionGate {
    fn check(table: &Table, row: usize) {
        enable_for!(table, Decomposition, row);
        let (get_limbs, _, get_word_sum, _) = helpers!(table, row);

        let [a, e] = [get_word_sum(A, 0), get_word_sum(E, 0)];
        let a_limbs = get_limbs(AX, AY, AZ, 1);
        let e_limbs = get_limbs(EX, EY, EZ, 1);

        assert_eq!(a_limbs, TDec::lookup(a), "a mismatch at row {row}");
        assert_eq!(e_limbs, TDec::lookup(e), "e mismatch at row {row}");
    }
}

pub struct WitnessComputationGate;
impl Gate for WitnessComputationGate {
    fn check(table: &Table, row: usize) {
        enable_for!(table, WitnessComputation, row);
        let (_, get_word, get_wordsum, _) = helpers!(table, row);

        let (w_result, w_sum, w1, w2, w3, w4) = (
            get_word(W, 0),
            get_wordsum(W_SUM, 0),
            get_word(W1, 2 * ROUND_BACK + 2) as WordSum,
            get_word(W, 7 * ROUND_BACK) as WordSum,
            get_word(W2, 15 * ROUND_BACK + 2) as WordSum,
            get_word(W, 16 * ROUND_BACK) as WordSum,
        );

        assert_eq!(w_sum, w1 + w2 + w3 + w4, "w_sum mismatch at row {row}");
        assert_eq!(w_result, TMod::lookup(w_sum), "w mismatch at row {row}")
    }
}

pub struct ResultVerificationGate;
impl Gate for ResultVerificationGate {
    fn check(table: &Table, row: usize) {
        enable_for!(table, ResultVerification, row);
        let (get_limbs, _, get_wordsum, _) = helpers!(table, row);

        let [out_low, out_high] = [table.public_input[row], table.public_input[row + 1]];

        let sum_low = compose(&get_limbs(AX, AY, AZ, 0)) as WordSum
            + table.fixed_part.constants[row] as WordSum;
        let sum_high = compose(&get_limbs(EX, EY, EZ, 0)) as WordSum
            + table.fixed_part.constants[row + 1] as WordSum;

        fn eq<T: Eq + Debug>(left: T, right: T, item: &'static str, row: usize) {
            assert_eq!(left, right, "{item} mismatch at row {row}");
        }

        eq(sum_low, get_wordsum(LOW_SUM, 3), "low sum", row);
        eq(sum_high, get_wordsum(HIGH_SUM, 3), "high sum", row);
        eq(out_low, TMod::lookup(sum_low), "low out", row);
        eq(out_high, TMod::lookup(sum_high), "high out", row);
    }
}
