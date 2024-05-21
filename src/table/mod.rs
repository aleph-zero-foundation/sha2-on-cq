use crate::{
    table::{advice::Advice, fixed::FixedPart},
    trace::Trace,
    types::Word,
};

mod advice;
mod fixed;
mod indices;
mod render;

pub const ROWS_PER_ROUND: usize = 4;
pub const NUM_ROWS: usize = (16 + 64) * ROWS_PER_ROUND + 1;
pub const ADVICE_COLUMNS: usize = 8;

#[derive(Clone)]
pub struct Table {
    fixed_part: FixedPart,
    advice: Advice,
    public_input: [Word; 8],
}

impl Table {
    pub fn new(private_input: [Word; 16], public_input: [Word; 8]) -> Self {
        Self {
            fixed_part: FixedPart::new(),
            advice: Advice::from_trace(&Trace::new(private_input)),
            public_input,
        }
    }
}
