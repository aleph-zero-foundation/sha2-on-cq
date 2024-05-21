use std::collections::HashSet;

use tabled::builder::Builder;

use crate::{
    table::{ADVICE_COLUMNS, NUM_ROWS, Table},
    types::Index,
};

fn init_rows(builder: &mut Builder) {
    builder.push_record(["row index"]);
    for i in 0..NUM_ROWS {
        builder.push_record([i.to_string()]);
    }
}

impl Table {
    pub fn render(&self) -> tabled::tables::Table {
        let mut builder = Builder::default();

        init_rows(&mut builder);
        self.render_instance(&mut builder);
        self.render_fixed(&mut builder);
        self.render_witness(&mut builder);
        self.render_selectors(&mut builder);

        builder.build()
    }

    fn render_instance(&self, builder: &mut Builder) {
        builder.push_column(render_column("result (public input)", self.public_input));
    }

    fn render_fixed(&self, builder: &mut Builder) {
        builder.push_column(render_column(
            "round constants (fixed)",
            self.fixed_part.round_constants,
        ));
    }

    fn render_selectors(&self, builder: &mut Builder) {
        builder.push_column(render_column(
            "lookup",
            render_selector(&self.fixed_part.selectors.lookups),
        ));
        builder.push_column(render_column(
            "composition",
            render_selector(&self.fixed_part.selectors.composition),
        ));
        builder.push_column(render_column(
            "addition",
            render_selector(&self.fixed_part.selectors.addition),
        ));
        builder.push_column(render_column(
            "decomposition",
            render_selector(&self.fixed_part.selectors.decomposition),
        ));
        builder.push_column(render_column(
            "witness computation",
            render_selector(&self.fixed_part.selectors.witness_computation),
        ));
        builder.push_column(render_column(
            "result verification",
            render_selector(&self.fixed_part.selectors.result_verification),
        ));
    }

    fn render_witness(&self, builder: &mut Builder) {
        for i in 0..ADVICE_COLUMNS {
            builder.push_column(render_column("", &self.advice[i]));
        }
    }
}

fn render_selector(selector: &HashSet<Index>) -> impl IntoIterator<Item = impl ToString> + '_ {
    (0..NUM_ROWS).map(|i| selector.contains(&i).then(|| "✅").unwrap_or(""))
}

fn render_column(
    title: &'static str,
    values: impl IntoIterator<Item = impl ToString>,
) -> impl IntoIterator<Item = String> {
    [title.to_string()]
        .into_iter()
        .chain(values.into_iter().map(|x| x.to_string()))
}
