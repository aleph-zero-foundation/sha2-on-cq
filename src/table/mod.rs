use std::collections::HashSet;

use crate::types::{AdviceEntry, Index, Limb, Word};

mod render;

#[derive(Clone, Default)]
pub struct Selectors {
    pub lookups: HashSet<Index>,
    pub composition: HashSet<Index>,
    pub addition: HashSet<Index>,
    pub decomposition: HashSet<Index>,
}

#[derive(Clone)]
pub struct FixedPart {
    pub round_constants: [Word; 64],
    pub initial_hash_words: [Limb; 24],
    pub selectors: Selectors,
}

impl FixedPart {
    fn new() -> Self {
        Self {
            round_constants: [0; 64],
            initial_hash_words: [0; 24],
            selectors: Selectors::default(),
        }
    }
}

#[derive(Clone)]
pub struct Witness {
    pub message_schedule: [Word; 64],
    pub advice: [Vec<AdviceEntry>; 24],
}

impl Witness {
    fn new() -> Self {
        Self {
            message_schedule: [0; 64],
            advice: (0..24)
                .map(|_| Vec::new())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

#[derive(Clone)]
pub struct Table {
    pub fixed_part: FixedPart,
    pub witness: Witness,
    pub public_input: [Limb; 24],
}

impl Table {
    pub fn new() -> Self {
        Self {
            fixed_part: FixedPart::new(),
            witness: Witness::new(),
            public_input: [0; 24],
        }
    }
}
