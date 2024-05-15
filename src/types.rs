use std::fmt::Display;

pub type Word = u32;
pub type Limb = u16;
pub type Index = usize;

pub fn decompose(word: &Word) -> [Limb; 3] {
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

#[derive(Copy, Clone, Debug)]
pub enum AdviceEntry {
    Word(Word),
    Limb(Limb),
}

impl Display for AdviceEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AdviceEntry::Word(word) => write!(f, "{word}"),
            AdviceEntry::Limb(limb) => write!(f, "{limb}"),
        }
    }
}
