use crate::{
    table::{advice::Advice, fixed::FixedPart},
    trace::Trace,
    types::Word,
};

mod advice;
mod fixed;
mod indices;
mod render;

pub const NUM_ROWS: usize = 259;

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
            advice: Advice::new(&Trace::new(private_input)),
            public_input,
        }
    }
}
