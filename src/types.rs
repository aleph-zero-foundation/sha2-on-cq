use std::fmt::Display;

pub type Word = u32;
pub type Limb = u16;
pub type Index = usize;

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