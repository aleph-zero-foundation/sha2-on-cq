use std::ops::Index;

use crate::{
    constants::{INITIAL_HASH_WORDS, ROUND_CONSTANTS},
    types::{right_rotation, Bitem, Word, WordSum},
    ROUNDS,
};

pub struct Trace {
    round_outputs: [[Bitem; ROUNDS]; 8],

    majority: [Bitem; ROUNDS],
    choose: [Bitem; ROUNDS],
    rotations: [[Bitem; ROUNDS]; 2],

    k: [Bitem; ROUNDS],
    w: [Bitem; ROUNDS],
}

impl Trace {
    pub fn new(input: [Word; 16]) -> Self {
        let nothing = [Bitem::default(); ROUNDS];
        Self {
            round_outputs: [nothing; 8],
            majority: nothing,
            choose: nothing,
            rotations: [nothing; 2],
            k: ROUND_CONSTANTS.map(Bitem::from),
            w: nothing,
        }
        .compute_message_schedule(input)
        .do_rounds()
    }

    fn compute_message_schedule(mut self, input: [Word; 16]) -> Self {
        let mut result = [Word::default(); ROUNDS];
        result[..16].copy_from_slice(&input);

        for i in 16..ROUNDS {
            let s0 = right_rotation(result[i - 15], 3)
                ^ right_rotation(result[i - 15], 7)
                ^ right_rotation(result[i - 15], 18);

            let s1 = right_rotation(result[i - 2], 10)
                ^ right_rotation(result[i - 2], 17)
                ^ right_rotation(result[i - 2], 19);

            result[i] = result[i - 16]
                .wrapping_add(s0)
                .wrapping_add(result[i - 7])
                .wrapping_add(s1);
        }

        self.w = result.map(Bitem::from);
        self
    }

    fn do_rounds(mut self) -> Self {
        let mut round_input = INITIAL_HASH_WORDS.map(Bitem::from);
        for round in 0..ROUNDS {
            round_input = self.do_round(round, round_input);
        }
        self
    }

    fn do_round(&mut self, round: usize, round_input: [Bitem; 8]) -> [Bitem; 8] {
        let [a, b, c, d, e, f, g, h] = round_input;

        let maj = (a.word & b.word) ^ (a.word & c.word) ^ (b.word & c.word);
        let ch = (e.word & f.word) ^ ((!e.word) & g.word);

        let rot0 =
            right_rotation(a.word, 2) ^ right_rotation(a.word, 13) ^ right_rotation(a.word, 22);
        let rot1 =
            right_rotation(e.word, 6) ^ right_rotation(e.word, 11) ^ right_rotation(e.word, 25);

        let k = ROUND_CONSTANTS[round] as WordSum;
        let w = self.w[round].full;

        let temp1 = h.full + (rot1 as WordSum) + (ch as WordSum) + k + w;
        let temp2 = (rot0 as WordSum) + (maj as WordSum);

        let A = temp1 + temp2;
        let E = d.full + temp2;

        let outputs = [A.into(), a, b, c, E.into(), e, f, g];
        for (column, item) in self.round_outputs.iter_mut().zip(outputs.iter()) {
            column[round] = *item;
        }
        outputs
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[allow(unused)]
pub enum TraceItem {
    a,
    b,
    c,
    d,
    e,
    f,
    g,
    h,
    maj,
    ch,
    rot0,
    rot1,
    k,
    w,
}

impl Index<TraceItem> for Trace {
    type Output = [Bitem; ROUNDS];

    fn index(&self, item: TraceItem) -> &Self::Output {
        match item {
            TraceItem::a => &self.round_outputs[0],
            TraceItem::b => &self.round_outputs[1],
            TraceItem::c => &self.round_outputs[2],
            TraceItem::d => &self.round_outputs[3],
            TraceItem::e => &self.round_outputs[4],
            TraceItem::f => &self.round_outputs[5],
            TraceItem::g => &self.round_outputs[6],
            TraceItem::h => &self.round_outputs[7],
            TraceItem::maj => &self.majority,
            TraceItem::ch => &self.choose,
            TraceItem::rot0 => &self.rotations[0],
            TraceItem::rot1 => &self.rotations[1],
            TraceItem::k => &self.k,
            TraceItem::w => &self.w,
        }
    }
}
