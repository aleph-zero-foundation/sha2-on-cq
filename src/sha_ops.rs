use std::ops::{BitAnd, BitXor, Not};

use crate::types::{compose, right_rotation, right_shift, Limb, Word};

pub fn rot0(a_limbs: [Limb; 3]) -> Word {
    let a = compose(&a_limbs);
    right_rotation(a, 2) ^ right_rotation(a, 13) ^ right_rotation(a, 22)
}

pub fn rot1(e_limbs: [Limb; 3]) -> Word {
    let e = compose(&e_limbs);
    right_rotation(e, 6) ^ right_rotation(e, 11) ^ right_rotation(e, 25)
}

pub fn majority<T: Copy + BitAnd<Output = T> + BitXor<Output = T>>(x: T, y: T, z: T) -> T {
    (x & y) ^ (x & z) ^ (y & z)
}

pub fn choose<T: Copy + BitAnd<Output = T> + BitXor<Output = T> + Not<Output = T>>(
    x: T,
    y: T,
    z: T,
) -> T {
    (x & y) ^ (!x & z)
}

pub fn witness_op1(w: Word) -> Word {
    right_rotation(w, 17) ^ right_rotation(w, 19) ^ right_shift(w, 10)
}

pub fn witness_op2(w: Word) -> Word {
    right_rotation(w, 7) ^ right_rotation(w, 18) ^ right_shift(w, 3)
}
