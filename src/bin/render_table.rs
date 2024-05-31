use std::fs;

use sha_on_cq::{
    sha::{prepare_input, sha_hash},
    table::Table,
};
use tabled::settings::{
    object::Columns,
    style::{LineChar, Offset},
    Modify, Style,
};

fn main() {
    let input = "";

    let plonk_table = Table::new(prepare_input(input), sha_hash(input));
    plonk_table.validate();

    let mut table = plonk_table.render();
    table.with(Style::markdown()).with(
        Modify::new(Columns::new(..))
            .with(LineChar::horizontal(':', Offset::Begin(0)))
            .with(LineChar::horizontal(':', Offset::End(0))),
    );

    fs::write("table.md", table.to_string().as_bytes()).unwrap();
}
