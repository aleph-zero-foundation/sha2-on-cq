use std::collections::HashSet;

use crate::{
    constants::{INITIAL_HASH_WORDS, ROUND_CONSTANTS},
    table::advice::Advice,
    trace::Trace,
    types::{decompose_many, Index, Limb, Word},
};

mod advice;
mod indices;
mod render;

pub const NUM_ROWS: usize = 259;

#[derive(Clone)]
pub struct Selectors {
    pub lookups: HashSet<Index>,
    pub composition: HashSet<Index>,
    pub addition: HashSet<Index>,
    pub decomposition: HashSet<Index>,
    pub finalization: HashSet<Index>,
}

impl Selectors {
    fn new() -> Self {
        let mut lookups = HashSet::new();
        let mut composition = HashSet::new();
        let mut addition = HashSet::new();
        let mut decomposition = HashSet::new();

        for i in 0..(NUM_ROWS - 3) {
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
            finalization: HashSet::from([NUM_ROWS - 3]),
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
pub struct Table {
    pub fixed_part: FixedPart,
    pub witness: Advice,
    pub public_input: [Word; 8],
}

impl Table {
    pub fn new(private_input: Option<[Word; 16]>, public_input: Option<[Word; 8]>) -> Self {
        let slf = Self {
            fixed_part: FixedPart::new(),
            witness: private_input
                .map_or_else(Advice::empty, |input| Advice::new(&Trace::new(input))),
            public_input: public_input.unwrap_or_else(|| [0; 8]),
        };

        if public_input.is_some() && private_input.is_some() {}

        slf
    }
}
