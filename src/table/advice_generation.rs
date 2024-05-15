use crate::{
    table::{
        constants::{INITIAL_HASH_WORDS, ROUND_CONSTANTS},
        indices::*,
        NUM_ROWS,
    },
    types::{compose, decompose, decompose_many, AdviceEntry, Limb, Word},
};

type AdviceArea = [Vec<AdviceEntry>; 24];

const ROWS_PER_ROUND: usize = 4;

pub fn compute_advice(message_schedule: [Word; 64]) -> AdviceArea {
    let mut advice = vec![vec![AdviceEntry::Mpty; NUM_ROWS]; 24]
        .try_into()
        .unwrap();

    fill_first_row(&mut advice);
    for i in 0..64 {
        compute_round(&mut advice, message_schedule[i], i);
    }

    advice
}

fn fill_first_row(advice: &mut AdviceArea) {
    let initializers: [Limb; 24] = decompose_many(&INITIAL_HASH_WORDS);
    for (advice_column, initializer) in advice.iter_mut().zip(initializers) {
        advice_column[0] = AdviceEntry::Limb(initializer);
    }
}

fn compute_round(advice: &mut AdviceArea, w: Word, round: usize) {
    let base_row = ROWS_PER_ROUND * round;
    let round_inputs = advice
        .iter()
        .map(|column| column[base_row].limb())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    compute_lookup_row(advice, &round_inputs, base_row + 1);
    compute_composition_row(advice, w, base_row + 2, round);
    compute_addition_row(advice, base_row + 3);
    prepare_next_round(advice, &round_inputs, base_row + 4);
}

fn compute_lookup_row(advice: &mut AdviceArea, input_limbs: &[Limb; 24], row: usize) {
    advice[ROT0][row] = compute_rot0(input_limbs[AX], input_limbs[AY], input_limbs[AZ]).into();
    advice[ROT1][row] = compute_rot1(input_limbs[EX], input_limbs[EY], input_limbs[EZ]).into();

    advice[MAJ_X][row] = compute_maj(input_limbs[AX], input_limbs[BX], input_limbs[CX]).into();
    advice[MAJ_Y][row] = compute_maj(input_limbs[AY], input_limbs[BY], input_limbs[CY]).into();
    advice[MAJ_Z][row] = compute_maj(input_limbs[AZ], input_limbs[BZ], input_limbs[CZ]).into();

    advice[CH_X][row] = compute_choose(input_limbs[EX], input_limbs[FX], input_limbs[GX]).into();
    advice[CH_Y][row] = compute_choose(input_limbs[EY], input_limbs[FY], input_limbs[GY]).into();
    advice[CH_Z][row] = compute_choose(input_limbs[EZ], input_limbs[FZ], input_limbs[GZ]).into();

    advice[DX][row] = input_limbs[DX].into();
    advice[DY][row] = input_limbs[DY].into();
    advice[DZ][row] = input_limbs[DZ].into();

    advice[HX][row] = input_limbs[HX].into();
    advice[HY][row] = input_limbs[HY].into();
    advice[HZ][row] = input_limbs[HZ].into();
}

fn compute_rot0(x: Limb, y: Limb, z: Limb) -> Word {
    (x + y + z) as Word
}

fn compute_rot1(x: Limb, y: Limb, z: Limb) -> Word {
    (x + y + z) as Word
}

fn compute_maj(x: Limb, y: Limb, z: Limb) -> Limb {
    x + y + z
}

fn compute_choose(x: Limb, y: Limb, z: Limb) -> Limb {
    x + y + z
}

fn compute_composition_row(advice: &mut AdviceArea, message_word: Word, row: usize, round: usize) {
    advice[ROT0][row] = advice[ROT0][row - 1];
    advice[ROT1][row] = advice[ROT1][row - 1];

    let mut composition = |target: usize, x: usize, y: usize, z: usize| {
        advice[target][row] = compose(&[
            advice[x][row - 1].limb(),
            advice[y][row - 1].limb(),
            advice[z][row - 1].limb(),
        ])
        .into()
    };

    composition(MAJ_X, MAJ_X, MAJ_Y, MAJ_Z);
    composition(CH_X, CH_X, CH_Y, CH_Z);
    composition(D, DX, DY, DZ);
    composition(H, HX, HY, HZ);

    advice[K][row] = ROUND_CONSTANTS[round].into();
    advice[W][row] = message_word.into();
}

fn compute_addition_row(advice: &mut AdviceArea, row: usize) {
    let above = |column: usize| advice[column][row - 1].word();

    let temp1 = above(H)
        .wrapping_add(above(ROT1))
        .wrapping_add(above(CH))
        .wrapping_add(above(W))
        .wrapping_add(above(K));
    let temp2 = above(ROT0).wrapping_add(above(MAJ));
    let d = above(D);

    advice[A][row] = temp1.wrapping_add(temp2).into();
    advice[E][row] = temp1.wrapping_add(d).into();
}

fn prepare_next_round(advice: &mut AdviceArea, input_limbs: &[Limb; 24], row: usize) {
    for (from, to) in [
        AX, AY, AZ, BX, BY, BZ, CX, CY, CZ, EX, EY, EZ, FX, FY, FZ, GX, GY, GZ,
    ]
    .into_iter()
    .zip([
        BX, BY, BZ, CX, CY, CZ, DX, DY, DZ, FX, FY, FZ, GX, GY, GZ, HX, HY, HZ,
    ]) {
        advice[to][row] = input_limbs[from].into();
    }

    let a_limbs = decompose(&advice[A][row - 1].word());
    advice[AX][row] = a_limbs[0].into();
    advice[AY][row] = a_limbs[1].into();
    advice[AZ][row] = a_limbs[2].into();

    let e_limbs = decompose(&advice[E][row - 1].word());
    advice[EX][row] = e_limbs[0].into();
    advice[EY][row] = e_limbs[1].into();
    advice[EZ][row] = e_limbs[2].into();
}
