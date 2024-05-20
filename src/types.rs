use std::fmt::Display;

pub type Word = u32;
pub type Limb = u16;
pub type Index = usize;

pub fn print_word_bits(word: Word) {
    println!("{word:032b}");
}

pub fn print_limb_bits(limb: Limb) {
    println!("{limb:016b}");
}

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

pub fn right_rotation(word: Word, n: usize) -> Word {
    (word >> n) | (word << (32 - n))
}

#[derive(Copy, Clone, Debug)]
pub enum AdviceEntry {
    Word(Word),
    Limb(Limb),
    Mpty,
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
}

impl From<Word> for AdviceEntry {
    fn from(word: Word) -> Self {
        AdviceEntry::Word(word)
    }
}

impl From<Limb> for AdviceEntry {
    fn from(limb: Limb) -> Self {
        AdviceEntry::Limb(limb)
    }
}

impl Display for AdviceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdviceEntry::Word(word) => write!(f, "{word}"),
            AdviceEntry::Limb(limb) => write!(f, "{limb}"),
            AdviceEntry::Mpty => write!(f, ""),
        }
    }
}
