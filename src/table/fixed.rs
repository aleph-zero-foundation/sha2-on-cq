use std::collections::HashSet;

use crate::{
    constants::ROUND_CONSTANTS,
    types::{Index, Word},
};

#[derive(Clone)]
pub struct FixedPart {
    pub round_constants: [Word; 64],
    pub selectors: Selectors,
}

impl FixedPart {
    pub fn new() -> Self {
        Self {
            round_constants: ROUND_CONSTANTS,
            selectors: Selectors::new(),
        }
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
