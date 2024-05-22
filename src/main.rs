use std::fs;

use sha_on_cq::{table::Table, types::Word};
use tabled::settings::{
    object::Columns,
    style::{LineChar, Offset},
    Modify, Style,
};

/// Message schedule for '' (empty string).
const MESSAGE_SCHEDULE: [Word; 16] = [
    0b10000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000000000000000000000000000,
];

/// SHA-256 hash of '' (empty string).
const SHA_OUTPUT: [Word; 8] = [
    0xe3b0c442, 0x98fc1c14, 0x9afbf4c8, 0x996fb924, 0x27ae41e4, 0x649b934c, 0xa495991b, 0x7852b855,
];

fn main() {
    let plonk_table = Table::new(MESSAGE_SCHEDULE, SHA_OUTPUT);
    plonk_table.validate();

    let mut table = plonk_table.render();
    table.with(Style::markdown()).with(
        Modify::new(Columns::new(..))
            .with(LineChar::horizontal(':', Offset::Begin(0)))
            .with(LineChar::horizontal(':', Offset::End(0))),
    );
    fs::write("table.md", table.to_string().as_bytes()).unwrap();
}
