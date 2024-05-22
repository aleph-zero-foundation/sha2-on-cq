use crate::{
    table::{advice::Advice, fixed::FixedPart, gates::Gate},
    trace::Trace,
    types::Word,
    ROUNDS,
};

mod advice;
mod fixed;
mod gates;
mod indices;
mod render;

pub const INITIAL_BUFFER: usize = 3;
pub const ROWS_PER_ROUND: usize = 4;
pub const NUM_ROWS: usize = (INITIAL_BUFFER + ROUNDS) * ROWS_PER_ROUND + 2;
pub const ADVICE_COLUMNS: usize = 8;

#[derive(Clone)]
pub struct Table {
    fixed_part: FixedPart,
    advice: Advice,
    public_input: [Word; NUM_ROWS],
}

impl Table {
    pub fn new(private_input: [Word; 16], public_input: [Word; 8]) -> Self {
        Self {
            fixed_part: FixedPart::new(),
            advice: Advice::from_trace(&Trace::new(private_input)),
            public_input: Self::spread_public_input(public_input),
        }
    }

    fn spread_public_input(input: [Word; 8]) -> [Word; NUM_ROWS] {
        const OFFSET: usize = (INITIAL_BUFFER + ROUNDS - 3) * ROWS_PER_ROUND;
        let mut col = [0; NUM_ROWS];

        col[OFFSET + 0 * ROWS_PER_ROUND] = input[3];
        col[OFFSET + 0 * ROWS_PER_ROUND + 1] = input[7];
        col[OFFSET + 1 * ROWS_PER_ROUND] = input[2];
        col[OFFSET + 1 * ROWS_PER_ROUND + 1] = input[6];
        col[OFFSET + 2 * ROWS_PER_ROUND] = input[1];
        col[OFFSET + 2 * ROWS_PER_ROUND + 1] = input[5];
        col[OFFSET + 3 * ROWS_PER_ROUND] = input[0];
        col[OFFSET + 3 * ROWS_PER_ROUND + 1] = input[4];

        col
    }

    pub fn validate(&self) {
        for row in 0..NUM_ROWS {
            gates::LookupGate::check(self, row);
            gates::CompositionGate::check(self, row);
        }
    }
}
