use std::fs;

use tabled::settings::Style;

use sha_on_cq::{
    table::Table,
    types::Word,
};

const MESSAGE_SCHEDULE: [Word; 64] = [
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
    0b10000000000000000000000000000000,
    0b00000000000000000000000000000000,
    0b00000000001000000101000000000000,
    0b00000000000000000000000000000000,
    0b00100010000000000000100000000000,
    0b00000000000000000000000000000000,
    0b00000101000010001001010101000010,
    0b10000000000000000000000000000000,
    0b01011000000010000000000000000000,
    0b00000000010000001010000000000000,
    0b00000000000101100010010100000101,
    0b01100110000000000001100000000000,
    0b11010110001000100010010110000000,
    0b00010100001000100101010100001000,
    0b11010110010001011111100101011100,
    0b11001001001010000010000000000000,
    0b11000011111100010000000010010100,
    0b00101000010011001010011101100110,
    0b00000110100010000110110111000110,
    0b10100011011110111111000100010110,
    0b01110001011111001011111010010110,
    0b11111110110000101101011101001010,
    0b10100111101101100111111100000000,
    0b10000001000101011001011010100010,
    0b10011000101001101110011101101000,
    0b00000011101100100000110010000010,
    0b01011101000111011010011111001001,
    0b10110001010101101011100100110101,
    0b11000011110111011100101000010001,
    0b00100100100111000001000001111111,
    0b11000100100011010010010011101111,
    0b01011101111001010100110000110000,
    0b11011110111111101100111001100101,
    0b00101100101000010100100000001101,
    0b00111100000101010011001100101100,
    0b00000001110011101100100110101101,
    0b00010110000011001100110011010000,
    0b00001011101011001101101010011000,
    0b00110110000110111000111111100000,
    0b11010010001100100000101110100110,
    0b00000010100110110111000000000111,
    0b01110101010001100101100001111100,
    0b00000111111101010100111100111001,
    0b11111000000010001101110111000011,
    0b11011100110010100111011000001000,
    0b01011110010000100111000110001000,
    0b01000100101111001110110001011101,
    0b00111011010111101100010010011011,
];

const SHA_OUTPUT: [Word; 8] = [
    0xe3b0c442, 0x98fc1c14, 0x9afbf4c8, 0x996fb924, 0x27ae41e4, 0x649b934c, 0xa495991b, 0x7852b855,
];

fn main() {
    let plonk_table = Table::new(Some(MESSAGE_SCHEDULE), Some(SHA_OUTPUT));
    let mut table = plonk_table.render();
    table.with(Style::markdown());
    fs::write("table.md", table.to_string().as_bytes()).unwrap();
}
