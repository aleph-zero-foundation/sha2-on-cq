use std::collections::HashSet;

use crate::{
    constants::ROUND_CONSTANTS,
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
        const OFFSET: usize = INITIAL_BUFFER * ROWS_PER_ROUND;
        for i in 0..ROUNDS {
            col[OFFSET + i * ROWS_PER_ROUND + 2] = ROUND_CONSTANTS[i];
        }
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
        let per_round = |in_round_offset| {
            (0..ROUNDS)
                .map(|r| INITIAL_BUFFER * ROWS_PER_ROUND + r * ROWS_PER_ROUND + in_round_offset)
                .collect()
        };

        Self {
            lookups: per_round(0),
            composition: per_round(1),
            addition: per_round(2),
            ..Self::default()
        }
    }
}
