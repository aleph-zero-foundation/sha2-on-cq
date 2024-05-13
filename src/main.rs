fn main() {
    let mut table = Table::new().render();
    table.with(Style::markdown());

    fs::write("table.md", table.to_string().as_bytes()).unwrap();
}
