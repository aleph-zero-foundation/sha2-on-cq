use crate::types::{AdviceEntry, Word};

pub fn compute_advice(message_schedule: [Word; 64]) -> [Vec<AdviceEntry>; 24] {
    let mut advice = vec![Vec::new(); 24];

    // for i in 0..24 {
    //     advice[i] = message_schedule
    //         .iter()
    //         .map(|word| AdviceEntry::Word(*word))
    //         .collect();
    // }

    advice.try_into().unwrap()
}
