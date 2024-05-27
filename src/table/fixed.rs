use std::collections::HashSet;

use crate::{
    constants::{INITIAL_HASH_WORDS, ROUND_CONSTANTS},
    table::{INITIAL_BUFFER, NUM_ROWS, ROWS_PER_ROUND},
    types::{Index, Word},
    ROUNDS,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Selector {
    Lookup,
    Composition,
    Addition,
    Decomposition,
    WitnessComputation,
    ResultVerification,
}

pub struct FixedPart {
    pub constants: [Word; NUM_ROWS],
    pub selectors: Selectors,
}

impl FixedPart {
    pub fn new() -> Self {
        Self {
            constants: Self::setup_constants(),
            selectors: Selectors::new(),
        }
    }

    #[allow(clippy::erasing_op)]
    fn setup_constants() -> [Word; NUM_ROWS] {
        let mut col = [0; NUM_ROWS];

        let offset = INITIAL_BUFFER * ROWS_PER_ROUND;
        for i in 0..ROUNDS {
            col[offset + i * ROWS_PER_ROUND + 2] = ROUND_CONSTANTS[i];
        }

        let offset = (INITIAL_BUFFER + ROUNDS - 3) * ROWS_PER_ROUND;
        col[offset + 0 * ROWS_PER_ROUND] = INITIAL_HASH_WORDS[3];
        col[offset + 0 * ROWS_PER_ROUND + 1] = INITIAL_HASH_WORDS[7];
        col[offset + 1 * ROWS_PER_ROUND] = INITIAL_HASH_WORDS[2];
        col[offset + 1 * ROWS_PER_ROUND + 1] = INITIAL_HASH_WORDS[6];
        col[offset + 2 * ROWS_PER_ROUND] = INITIAL_HASH_WORDS[1];
        col[offset + 2 * ROWS_PER_ROUND + 1] = INITIAL_HASH_WORDS[5];
        col[offset + 3 * ROWS_PER_ROUND] = INITIAL_HASH_WORDS[0];
        col[offset + 3 * ROWS_PER_ROUND + 1] = INITIAL_HASH_WORDS[4];

        col
    }

    pub fn is_enabled(&self, selector: Selector, row: Index) -> bool {
        match selector {
            Selector::Lookup => self.selectors.lookups.contains(&row),
            Selector::Composition => self.selectors.composition.contains(&row),
            Selector::Addition => self.selectors.addition.contains(&row),
            Selector::Decomposition => self.selectors.decomposition.contains(&row),
            Selector::WitnessComputation => self.selectors.witness_computation.contains(&row),
            Selector::ResultVerification => self.selectors.result_verification.contains(&row),
        }
    }
}

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
        let per_round = |in_round_offset, from| {
            (from..ROUNDS)
                .map(|r| INITIAL_BUFFER * ROWS_PER_ROUND + r * ROWS_PER_ROUND + in_round_offset)
                .collect()
        };

        let mut result_verification: HashSet<_> = per_round(0, 61);
        result_verification.insert((INITIAL_BUFFER + ROUNDS) * ROWS_PER_ROUND);

        Self {
            lookups: per_round(0, 0),
            composition: per_round(1, 0),
            addition: per_round(2, 0),
            decomposition: per_round(3, 0),
            witness_computation: per_round(0, 16),
            result_verification,
        }
    }
}
