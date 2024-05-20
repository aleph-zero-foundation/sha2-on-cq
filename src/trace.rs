use crate::{
    constants::{INITIAL_HASH_WORDS, ROUND_CONSTANTS},
    types::{right_rotation, Word, Worimb},
};

pub const ROUNDS: usize = 64;

pub struct Trace {
    round_outputs: [[Worimb; ROUNDS]; 8],

    majority: [Worimb; ROUNDS],
    choose: [Worimb; ROUNDS],
    rotations: [[Word; ROUNDS]; 2],

    k: [Word; ROUNDS],
    w: [Word; ROUNDS],
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
            k: ROUND_CONSTANTS,
            w,
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
        [[Word; ROUNDS]; 2],
    ) {
        let mut words = [[Worimb::default(); ROUNDS]; 8];

        let [mut maj, mut ch] = [[Worimb::default(); ROUNDS]; 2];
        let mut rotations = [[Word::default(); ROUNDS]; 2];

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

    fn do_round(w: Word, k: Word, words: [Worimb; 8]) -> ([Worimb; 8], [Worimb; 2], [Word; 2]) {
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
                Worimb::from(temp1.wrapping_add(temp2)),
                a,
                b,
                c,
                Worimb::from(d.word.wrapping_add(temp1)),
                e,
                f,
                g,
            ],
            [Worimb::from(maj), Worimb::from(ch)],
            [rot0, rot1],
        )
    }
}
