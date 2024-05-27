#![allow(non_snake_case, non_camel_case_types)]
#![allow(clippy::identity_op)]

pub const ROUNDS: usize = 64;

mod constants;
mod lookup_tables;
mod sha_ops;
pub mod table;
mod trace;
pub mod types;
