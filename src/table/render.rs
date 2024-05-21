use std::collections::HashSet;

use tabled::builder::Builder;

use crate::{
    table::{Table, ADVICE_COLUMNS, NUM_ROWS},
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
        builder.push_column(render_column_no_zeros("result (public input)", self.public_input));
    }

    fn render_fixed(&self, builder: &mut Builder) {
        builder.push_column(render_column_no_zeros(
            "round constants (fixed)",
            self.fixed_part.round_constants,
        ));
    }

    fn render_selectors(&self, builder: &mut Builder) {
        builder.push_column(render_column_no_zeros(
            "lookup",
            render_selector(&self.fixed_part.selectors.lookups),
        ));
        builder.push_column(render_column_no_zeros(
            "composition",
            render_selector(&self.fixed_part.selectors.composition),
        ));
        builder.push_column(render_column_no_zeros(
            "addition",
            render_selector(&self.fixed_part.selectors.addition),
        ));
        builder.push_column(render_column_no_zeros(
            "decomposition",
            render_selector(&self.fixed_part.selectors.decomposition),
        ));
        builder.push_column(render_column_no_zeros(
            "witness computation",
            render_selector(&self.fixed_part.selectors.witness_computation),
        ));
        builder.push_column(render_column_no_zeros(
            "result verification",
            render_selector(&self.fixed_part.selectors.result_verification),
        ));
    }

    fn render_witness(&self, builder: &mut Builder) {
        for i in 0..ADVICE_COLUMNS {
            builder.push_column(render_column_with_zeros("", &self.advice[i]));
        }
    }
}

fn render_selector(selector: &HashSet<Index>) -> impl IntoIterator<Item = impl ToString> + '_ {
    (0..NUM_ROWS).map(|i| selector.contains(&i).then(|| "✅").unwrap_or(""))
}

fn render_column_no_zeros(
    title: &'static str,
    values: impl IntoIterator<Item = impl ToString>,
) -> impl IntoIterator<Item = String> {
    render_column(title, values, true)
}

fn render_column_with_zeros(
    title: &'static str,
    values: impl IntoIterator<Item = impl ToString>,
) -> impl IntoIterator<Item = String> {
    render_column(title, values, false)
}

fn render_column(
    title: &'static str,
    values: impl IntoIterator<Item = impl ToString>,
    ignore_zeros: bool,
) -> impl IntoIterator<Item = String> {
    [title.to_string()]
        .into_iter()
        .chain(values.into_iter().map(move |x| {
            let xs = x.to_string();
            if ignore_zeros && xs == "0" {
                "".to_string()
            } else {
                xs
            }
        }))
}
