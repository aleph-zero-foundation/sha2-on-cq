use std::ops::Index;

use crate::{
    constants::{INITIAL_HASH_WORDS, ROUND_CONSTANTS},
    types::{right_rotation, Word, Worimb},
};

pub const ROUNDS: usize = 64;

pub struct Trace {
    round_outputs: [[Worimb; ROUNDS]; 8],

    majority: [Worimb; ROUNDS],
    choose: [Worimb; ROUNDS],
    rotations: [[Worimb; ROUNDS]; 2],

    k: [Worimb; ROUNDS],
    w: [Worimb; ROUNDS],
}

impl Trace {
    pub fn new(input: [Word; 16]) -> Self {
        let w = Self::compute_message_schedule(input);
        let (round_outputs, [majority, choose], rotations) = Self::do_rounds(w);

        Self {
            round_outputs,
            rotations,
            majority,
            choose,
            k: ROUND_CONSTANTS.map(Worimb::from),
            w: w.map(Worimb::from),
        }
    }

    fn compute_message_schedule(input: [Word; 16]) -> [Word; ROUNDS] {
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

        result
    }

    fn do_rounds(
        w: [Word; ROUNDS],
    ) -> (
        [[Worimb; ROUNDS]; 8],
        [[Worimb; ROUNDS]; 2],
        [[Worimb; ROUNDS]; 2],
    ) {
        let mut words = [[Worimb::default(); ROUNDS]; 8];

        let [mut maj, mut ch] = [[Worimb::default(); ROUNDS]; 2];
        let mut rotations = [[Worimb::default(); ROUNDS]; 2];

        let mut round_input = INITIAL_HASH_WORDS.map(Worimb::from);
        for round in 0..ROUNDS {
            let (new_words, [new_maj, new_ch], [new_rot0, new_rot1]) =
                Self::do_round(w[round], ROUND_CONSTANTS[round], round_input);

            for (word, new_word) in words.iter_mut().zip(new_words.iter()) {
                word[round] = *new_word;
            }
            (maj[round], ch[round]) = (new_maj, new_ch);
            (rotations[0][round], rotations[1][round]) = (new_rot0, new_rot1);

            round_input = new_words;
        }

        (words, [maj, ch], rotations)
    }

    fn do_round(w: Word, k: Word, words: [Worimb; 8]) -> ([Worimb; 8], [Worimb; 2], [Worimb; 2]) {
        let [a, b, c, d, e, f, g, h] = words;

        let maj = (a.word & b.word) ^ (a.word & c.word) ^ (b.word & c.word);
        let ch = (e.word & f.word) ^ ((!e.word) & g.word);

        let rot0 =
            right_rotation(a.word, 2) ^ right_rotation(a.word, 13) ^ right_rotation(a.word, 22);
        let rot1 =
            right_rotation(e.word, 6) ^ right_rotation(e.word, 11) ^ right_rotation(e.word, 25);

        let temp1 = h
            .word
            .wrapping_add(rot1)
            .wrapping_add(ch)
            .wrapping_add(k)
            .wrapping_add(w);
        let temp2 = rot0.wrapping_add(maj);

        (
            [
                temp1.wrapping_add(temp2).into(),
                a,
                b,
                c,
                d.word.wrapping_add(temp1).into(),
                e,
                f,
                g,
            ],
            [maj.into(), ch.into()],
            [rot0.into(), rot1.into()],
        )
    }
}

#[allow(non_camel_case_types)]
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
    type Output = [Worimb; ROUNDS];

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
