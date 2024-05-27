#![allow(non_snake_case, non_camel_case_types)]

pub const ROUNDS: usize = 64;

mod constants;
pub mod table;
mod trace;
pub mod types;
mod sha_ops;
mod lookup_tables;
