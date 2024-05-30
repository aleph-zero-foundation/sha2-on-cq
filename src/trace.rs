use std::ops::Index;

use crate::{
    sha_constants::{INITIAL_HASH_WORDS, ROUNDS, ROUND_CONSTANTS},
    sha_ops,
    types::{right_rotation, right_shift, Bitem, Word, WordSum},
};

pub struct Trace {
    round_inputs: [[Bitem; ROUNDS]; 8],
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
            round_inputs: [nothing; 8],
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
            let s0 = right_rotation(result[i - 15], 7)
                ^ right_rotation(result[i - 15], 18)
                ^ right_shift(result[i - 15], 3);

            let s1 = right_rotation(result[i - 2], 17)
                ^ right_rotation(result[i - 2], 19)
                ^ right_shift(result[i - 2], 10);

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
        for (column, item) in self.round_inputs.iter_mut().zip(round_input.iter()) {
            column[round] = *item;
        }

        let maj = sha_ops::majority(a.word, b.word, c.word);
        let ch = sha_ops::choose(e.word, f.word, g.word);

        let rot0 = sha_ops::rot0(a.limbs);
        let rot1 = sha_ops::rot1(e.limbs);

        let k = ROUND_CONSTANTS[round] as WordSum;
        let w = self.w[round].word as WordSum;

        let temp1 = (h.word as WordSum) + (rot1 as WordSum) + (ch as WordSum) + k + w;
        let temp2 = (rot0 as WordSum) + (maj as WordSum);

        let A = temp1 + temp2;
        let E = (d.word as WordSum) + temp1;

        let outputs = [A.into(), a, b, c, E.into(), e, f, g];

        for (column, item) in self.round_outputs.iter_mut().zip(outputs.iter()) {
            column[round] = *item;
        }
        [self.rotations[0][round], self.rotations[1][round]] = [rot0.into(), rot1.into()];
        [self.majority[round], self.choose[round]] = [maj.into(), ch.into()];

        outputs
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
#[allow(unused)]
pub enum TraceItem {
    a,
    a_in,
    b,
    b_in,
    c,
    c_in,
    d,
    d_in,
    e,
    e_in,
    f,
    f_in,
    g,
    g_in,
    h,
    h_in,
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
            TraceItem::a_in => &self.round_inputs[0],
            TraceItem::b_in => &self.round_inputs[1],
            TraceItem::c_in => &self.round_inputs[2],
            TraceItem::d_in => &self.round_inputs[3],
            TraceItem::e_in => &self.round_inputs[4],
            TraceItem::f_in => &self.round_inputs[5],
            TraceItem::g_in => &self.round_inputs[6],
            TraceItem::h_in => &self.round_inputs[7],
        }
    }
}
