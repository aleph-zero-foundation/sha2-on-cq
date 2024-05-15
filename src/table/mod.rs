use std::collections::HashSet;

use crate::{
    table::{
        advice_generation::compute_advice,
        constants::{INITIAL_HASH_WORDS, ROUND_CONSTANTS},
    },
    types::{decompose_many, AdviceEntry, Index, Limb, Word},
};

mod advice_generation;
mod constants;
mod indices;
mod render;

pub const NUM_ROWS: usize = 257;

#[derive(Clone)]
pub struct Selectors {
    pub lookups: HashSet<Index>,
    pub composition: HashSet<Index>,
    pub addition: HashSet<Index>,
    pub decomposition: HashSet<Index>,
}

impl Selectors {
    fn new() -> Self {
        let mut lookups = HashSet::new();
        let mut composition = HashSet::new();
        let mut addition = HashSet::new();
        let mut decomposition = HashSet::new();

        for i in 0..NUM_ROWS {
            match i % 4 {
                0 => lookups.insert(i),
                1 => composition.insert(i),
                2 => addition.insert(i),
                3 => decomposition.insert(i),
                _ => unreachable!(),
            };
        }

        Self {
            lookups,
            composition,
            addition,
            decomposition,
        }
    }
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
            round_constants: ROUND_CONSTANTS,
            initial_hash_words: decompose_many(&INITIAL_HASH_WORDS),
            selectors: Selectors::new(),
        }
    }
}

#[derive(Clone)]
pub struct Witness {
    pub message_schedule: [Word; 64],
    pub advice: [Vec<AdviceEntry>; 24],
}

impl Witness {
    fn empty() -> Self {
        Self {
            message_schedule: [0; 64],
            advice: (0..24)
                .map(|_| Vec::new())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }

    fn new(private_input: [Word; 64]) -> Self {
        Self {
            message_schedule: private_input,
            advice: compute_advice(private_input),
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
    pub fn new(private_input: Option<[Word; 64]>, public_input: Option<[Word; 8]>) -> Self {
        Self {
            fixed_part: FixedPart::new(),
            witness: private_input.map_or_else(Witness::empty, Witness::new),
            public_input: public_input.map_or_else(|| [0; 24], |words| decompose_many(&words)),
        }
    }
}
