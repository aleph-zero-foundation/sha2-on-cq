use crate::{
    sha_ops,
    table::{advice::AdviceEntry, fixed::Selector::Lookup, indices::*, Table, ROWS_PER_ROUND},
};

pub trait Gate {
    fn check(table: &Table, row: usize);
}

pub struct LookupGate;
impl Gate for LookupGate {
    fn check(table: &Table, row: usize) {
        if !table.fixed_part.is_enabled(Lookup, row) {
            return;
        }

        let get_limbs = |col1: usize, col2: usize, col3: usize, back_offset: usize| {
            [
                table.advice[col1][row - back_offset].limb(),
                table.advice[col2][row - back_offset].limb(),
                table.advice[col3][row - back_offset].limb(),
            ]
        };

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

        let eq = |col, item, exp: AdviceEntry| {
            assert_eq!(exp, table.advice[col][row + 1], "{item} mismatch at row {row}");
        };

        eq(ROT0, "rot0", exp_rot0.into());
        eq(ROT1, "rot1", exp_rot1.into());
        eq(MAJ_X, "maj_x", exp_maj_x.into());
        eq(MAJ_Y, "maj_y", exp_maj_y.into());
        eq(MAJ_Z, "maj_z", exp_maj_z.into());
        eq(CH_X, "ch_x", exp_ch_x.into());
        eq(CH_Y, "ch_y", exp_ch_y.into());
        eq(CH_Z, "ch_z", exp_ch_z.into());
    }
}
