use std::collections::HashSet;

use crate::{constants::ROUND_CONSTANTS, ROUNDS, types::{Index, Word}};
use crate::table::{NUM_ROWS, ROWS_PER_ROUND};

#[derive(Clone)]
pub struct FixedPart {
    pub round_constants: [Word; NUM_ROWS],
    pub selectors: Selectors,
}

impl FixedPart {
    pub fn new() -> Self {
        Self {
            round_constants: Self::spread_round_constants(),
            selectors: Selectors::new(),
        }
    }

    fn spread_round_constants() -> [Word; NUM_ROWS] {
        let mut col = [0; NUM_ROWS];
        const OFFSET: usize = 16 * ROWS_PER_ROUND;
        for i in 0..ROUNDS {
            col[OFFSET + i * ROWS_PER_ROUND + 2] = ROUND_CONSTANTS[i];
        }
        col
    }
}

#[derive(Clone, Default)]
pub struct Selectors {
    pub lookups: HashSet<Index>,
    pub composition: HashSet<Index>,
    pub addition: HashSet<Index>,
    pub decomposition: HashSet<Index>,
    pub witness_computation: HashSet<Index>,
    pub result_verification: HashSet<Index>,
}

impl Selectors {
    fn new() -> Self {
        // todo
        Self::default()
    }
}
