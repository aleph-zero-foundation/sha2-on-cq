pub type Word = u32;
pub type Limb = u16;
pub type Index = usize;

#[derive(Copy, Clone, Debug)]
pub enum AdviceEntry {
    Word(Word),
    Limb(Limb),
}
