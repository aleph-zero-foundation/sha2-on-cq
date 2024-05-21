use std::fmt::Display;

use crate::types::{Limb, Word, WordSum};

#[derive(Copy, Clone, Debug, Default)]
pub enum AdviceEntry {
    #[default]
    Mpty,
    Limb(Limb),
    Word(Word),
    WSum(WordSum),
}

impl AdviceEntry {
    pub fn limb(&self) -> Limb {
        match self {
            AdviceEntry::Limb(limb) => *limb,
            _ => panic!("Expected limb"),
        }
    }

    pub fn word(&self) -> Word {
        match self {
            AdviceEntry::Word(word) => *word,
            _ => panic!("Expected word"),
        }
    }

    pub fn word_sum(&self) -> WordSum {
        match self {
            AdviceEntry::WSum(word_sum) => *word_sum,
            _ => panic!("Expected word sum"),
        }
    }
}

impl From<Limb> for AdviceEntry {
    fn from(limb: Limb) -> Self {
        AdviceEntry::Limb(limb)
    }
}

impl From<Word> for AdviceEntry {
    fn from(word: Word) -> Self {
        AdviceEntry::Word(word)
    }
}

impl From<WordSum> for AdviceEntry {
    fn from(wordsum: WordSum) -> Self {
        AdviceEntry::WSum(wordsum)
    }
}

impl Display for AdviceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdviceEntry::Mpty => write!(f, ""),
            AdviceEntry::Limb(limb) => write!(f, "{limb}"),
            AdviceEntry::Word(word) => write!(f, "{word}"),
            AdviceEntry::WSum(wordsum) => write!(f, "{wordsum}"),
        }
    }
}
