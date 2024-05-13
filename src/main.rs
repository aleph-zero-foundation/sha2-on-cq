use std::fs;

use tabled::settings::Style;

use sha_on_cq::table::Table;

fn main() {
    let mut table = Table::new().render();
    table.with(Style::markdown());
    fs::write("table.md", table.to_string().as_bytes()).unwrap();
}
