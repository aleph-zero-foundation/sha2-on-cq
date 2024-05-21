use std::fmt::Display;

pub use composition::*;
pub use ops::*;
pub use printing::*;

pub type WordSum = u64;
pub type Word = u32;
pub type Limb = u16;
pub type Index = usize;

mod printing {
    use super::*;

    pub fn print_wordsum_bits(wordsum: WordSum) {
        println!("{wordsum:064b}");
    }

    pub fn print_word_bits(word: Word) {
        println!("{word:032b}");
    }

    pub fn print_limb_bits(limb: Limb) {
        println!("{limb:016b}");
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
            .into_iter()
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
}

#[derive(Default, Copy, Clone)]
pub struct Worimb {
    pub word: Word,
    pub limbs: [Limb; 3],
}

impl From<Word> for Worimb {
    fn from(word: Word) -> Self {
        Self {
            word,
            limbs: decompose(&word),
        }
    }
}
