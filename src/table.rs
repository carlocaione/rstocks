use owo_colors::OwoColorize;
use tabled::{
    settings::{
        format::Format,
        object::{Columns, Object, Rows},
        Style,
    },
    Table, Tabled,
};
use yahoo_finance_api::{Quote, YMetaData, YQuoteItem};

#[derive(Tabled)]
struct TableSearch<'a> {
    asset: &'a str,
    symbol: &'a str,
    exchange: &'a str,
    description: &'a str,
}

pub fn search(quotes: &[YQuoteItem]) {
    let content: Vec<TableSearch> = quotes
        .iter()
        .map(|q| TableSearch {
            asset: &q.type_display,
            symbol: &q.symbol,
            exchange: &q.exchange,
            description: if q.long_name.is_empty() {
                &q.short_name
            } else {
                &q.long_name
            },
        })
        .collect();

    let mut table = Table::new(content);
    table
        .with(Style::sharp())
        .modify(
            Columns::single(1).intersect(Rows::new(1..)),
            Format::content(|s| s.bold().red().to_string()),
        )
        .modify(
            Columns::last().intersect(Rows::new(1..)),
            Format::content(|s| s.green().to_string()),
        );

    println!("{}", table);
}

#[derive(Tabled, Default)]
struct TableInfo<'a> {
    asset: &'a str,
    symbol: &'a str,
    currency: &'a str,
    price: f64,
    #[tabled(rename = "day gain")]
    day_gain: f64,
    #[tabled(rename = "day gain (%)")]
    day_gain_perc: f64,
}

pub fn info(quote: &Quote, meta: &YMetaData) {
    let info = vec![TableInfo {
        asset: &meta.instrument_type,
        symbol: &meta.symbol,
        currency: &meta.currency,
        price: quote.close,
        day_gain: quote.close - quote.open,
        day_gain_perc: (quote.close - quote.open) / quote.open,
    }];

    let mut table = Table::new(info);
    table.with(Style::sharp()).modify(
        Columns::single(1).intersect(Rows::new(1..)),
        Format::content(|s| s.bold().red().to_string()),
    );

    println!("{}", table);
}
