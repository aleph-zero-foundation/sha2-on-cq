pub use bitem::*;
pub use composition::*;
pub use ops::*;
pub use printing::*;

pub type WordSum = u64;
pub type Word = u32;
pub type Limb = u16;
pub type Index = usize;

mod printing {
    use super::*;

    pub fn format_wordsum_bits(wordsum: WordSum) -> String {
        format!("{wordsum:064b}")
    }

    pub fn format_word_bits(word: Word) -> String {
        format!("{word:032b}")
    }

    pub fn format_limb_bits(limb: Limb) -> String {
        format!("{limb:016b}")
    }
}

mod composition {
    use super::*;

    pub const fn decompose(word: &Word) -> [Limb; 3] {
        [
            ((*word >> 21) % (1 << 11)) as Limb,
            ((*word >> 10) % (1 << 11)) as Limb,
            (*word % (1 << 10)) as Limb,
        ]
    }

    pub fn decompose_many<const N: usize, const M: usize>(words: &[Word; N]) -> [Limb; M] {
        words
            .iter()
            .flat_map(decompose)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap()
    }

    pub const fn compose(limbs: &[Limb; 3]) -> Word {
        (limbs[0] as Word) << 21 | (limbs[1] as Word) << 10 | limbs[2] as Word
    }
}

mod ops {
    use super::*;

    pub fn right_rotation(word: Word, n: usize) -> Word {
        (word >> n) | (word << (32 - n))
    }

    pub fn right_shift(word: Word, n: usize) -> Word {
        word >> n
    }
}

mod bitem {
    use crate::types::{decompose, Limb, Word, WordSum};

    /// A 'bitem' represents a value that might occur during the computation of the hash function.
    /// It fits within 64 bits and the full value is available at the field `full: WordSum`. Its
    /// truncated version is represented by the field `word: Word`. The field `limbs` contains the
    /// three 16-bit values that make up the `word` field.
    #[derive(Copy, Clone, Default)]
    pub struct Bitem {
        pub full: WordSum,
        pub word: Word,
        pub limbs: [Limb; 3],
    }

    impl From<WordSum> for Bitem {
        fn from(wordsum: WordSum) -> Self {
            let truncated = wordsum as Word;
            Self {
                full: wordsum,
                word: truncated,
                limbs: decompose(&truncated),
            }
        }
    }

    impl From<Word> for Bitem {
        fn from(word: Word) -> Self {
            Self {
                full: word as WordSum,
                word,
                limbs: decompose(&word),
            }
        }
    }
}
