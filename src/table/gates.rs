use crate::{
    sha_ops,
    table::{
        advice::AdviceEntry,
        fixed::Selector::{Addition, Composition, Lookup},
        indices::*,
        Table, INITIAL_BUFFER, ROWS_PER_ROUND,
    },
    types::{compose, WordSum},
};

macro_rules! helpers {
    ($table:ident, $row:ident) => {
        (
            |col1: usize, col2: usize, col3: usize, back_offset: usize| {
                [
                    $table.advice[col1][$row - back_offset].limb(),
                    $table.advice[col2][$row - back_offset].limb(),
                    $table.advice[col3][$row - back_offset].limb(),
                ]
            },
            |col: usize, back_offset: usize| $table.advice[col][$row - back_offset].word(),
            |col: usize, back_offset: usize| $table.advice[col][$row - back_offset].word_sum(),
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

pub trait Gate {
    fn check(table: &Table, row: usize);
}

pub struct LookupGate;
impl Gate for LookupGate {
    fn check(table: &Table, row: usize) {
        if !table.fixed_part.is_enabled(Lookup, row) {
            return;
        }

        let (get_limbs, _, _, check) = helpers!(table, row);

        let [ax, ay, az] = get_limbs(AX, AY, AZ, 0);
        let [ex, ey, ez] = get_limbs(EX, EY, EZ, 0);

        let [bx, by, bz] = get_limbs(BX, BY, BZ, ROWS_PER_ROUND);
        let [fx, fy, fz] = get_limbs(FX, FY, FZ, ROWS_PER_ROUND);

        let [cx, cy, cz] = get_limbs(CX, CY, CZ, 2 * ROWS_PER_ROUND);
        let [gx, gy, gz] = get_limbs(GX, GY, GZ, 2 * ROWS_PER_ROUND);

        let exp_rot0 = sha_ops::rot0([ax, ay, az]);
        let exp_rot1 = sha_ops::rot1([ex, ey, ez]);
        let [exp_maj_x, exp_maj_y, exp_maj_z] = [
            sha_ops::majority(ax, bx, cx),
            sha_ops::majority(ay, by, cy),
            sha_ops::majority(az, bz, cz),
        ];
        let [exp_ch_x, exp_ch_y, exp_ch_z] = [
            sha_ops::choose(ex, fx, gx),
            sha_ops::choose(ey, fy, gy),
            sha_ops::choose(ez, fz, gz),
        ];

        check(ROT0, "rot0", exp_rot0.into());
        check(ROT1, "rot1", exp_rot1.into());
        check(MAJ_X, "maj_x", exp_maj_x.into());
        check(MAJ_Y, "maj_y", exp_maj_y.into());
        check(MAJ_Z, "maj_z", exp_maj_z.into());
        check(CH_X, "ch_x", exp_ch_x.into());
        check(CH_Y, "ch_y", exp_ch_y.into());
        check(CH_Z, "ch_z", exp_ch_z.into());
    }
}

pub struct CompositionGate;
impl Gate for CompositionGate {
    fn check(table: &Table, row: usize) {
        if !table.fixed_part.is_enabled(Composition, row) {
            return;
        }
        let (get_limbs, _, _, check) = helpers!(table, row);

        let [maj_x, maj_y, maj_z] = get_limbs(MAJ_X, MAJ_Y, MAJ_Z, 0);
        let [ch_x, ch_y, ch_z] = get_limbs(CH_X, CH_Y, CH_Z, 0);
        let [dx, dy, dz] = get_limbs(DX, DY, DZ, 3 * ROWS_PER_ROUND + 1);
        let [hx, hy, hz] = get_limbs(HX, HY, HZ, 3 * ROWS_PER_ROUND + 1);

        check(MAJ, "maj", compose(&[maj_x, maj_y, maj_z]).into());
        check(CH, "ch", compose(&[ch_x, ch_y, ch_z]).into());
        check(D, "d", compose(&[dx, dy, dz]).into());
        check(H, "h", compose(&[hx, hy, hz]).into());
    }
}

pub struct AdditionGate;
impl Gate for AdditionGate {
    fn check(table: &Table, row: usize) {
        if !table.fixed_part.is_enabled(Addition, row) {
            return;
        }
        let (_, get_word, _, check) = helpers!(table, row);

        let [maj, ch] = [get_word(MAJ, 0) as WordSum, get_word(CH, 0) as WordSum];
        let [d, h] = [get_word(D, 0) as WordSum, get_word(H, 0) as WordSum];
        let [rot0, rot1] = [get_word(ROT0, 1) as WordSum, get_word(ROT1, 1) as WordSum];
        let k = table.fixed_part.round_constants[row] as WordSum;
        let w = get_word(W, 2) as WordSum;

        let temp1 = h
            .wrapping_add(rot1)
            .wrapping_add(ch)
            .wrapping_add(k)
            .wrapping_add(w);
        let temp2 = rot0.wrapping_add(maj);

        let exp_a = temp1.wrapping_add(temp2);
        let exp_e = temp1.wrapping_add(d);

        check(A, "A", exp_a.into());
        check(E, "E", exp_e.into());
    }
}
