use crate::{
    sha_constants::ROUNDS,
    table::{
        advice::Advice,
        fixed::FixedPart,
        gates::{
            AdditionGate, CompositionGate, DecompositionGate, Gate, LookupGate,
            ResultVerificationGate, WitnessComputationGate,
        },
    },
    trace::Trace,
    types::Word,
};

mod advice;
mod fixed;
mod gates;
mod indices;
mod render;

pub const INITIAL_BUFFER: usize = 3;
pub const ROWS_PER_ROUND: usize = 4;
pub const NUM_ROWS: usize = (INITIAL_BUFFER + ROUNDS + 1) * ROWS_PER_ROUND;
pub const ADVICE_COLUMNS: usize = 8;

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

    #[allow(clippy::erasing_op)]
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
            LookupGate::check(self, row);
            CompositionGate::check(self, row);
            AdditionGate::check(self, row);
            DecompositionGate::check(self, row);
            WitnessComputationGate::check(self, row);
            ResultVerificationGate::check(self, row);
        }
    }
}
