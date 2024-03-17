use derive_more::FromStr;
use financeapi::{FinanceapiAutocomplete, FinanceapiQuote};
use owo_colors::OwoColorize;
use std::{fmt, str::FromStr};
use tabled::{
    settings::{
        format::Format,
        object::{Columns, Object, Rows},
        Style,
    },
    Table, Tabled,
};

#[derive(FromStr, Default)]
struct Symbol(String);

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.bold().red())
    }
}

#[derive(Default)]
struct Price(f64);

impl fmt::Display for Price {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0.bold())
    }
}

#[derive(Default)]
struct Gain(f64);

impl fmt::Display for Gain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = if self.0.is_sign_positive() {
            format!("+{:.2}", self.0)
        } else {
            format!("{:.2}", self.0)
        };
        write!(f, "{}", s.green())
    }
}

#[derive(Default)]
struct PercGain(f64);

impl fmt::Display for PercGain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = if self.0.is_sign_positive() {
            format!("+{:.2}%", self.0)
        } else {
            format!("{:.2}%", self.0)
        };
        write!(f, "{}", s.green())
    }
}

#[derive(Tabled)]
struct TableSearch<'a> {
    asset: &'a str,
    #[tabled(rename = "ticker")]
    symbol: Symbol,
    exchange: &'a str,
    description: &'a str,
}

pub fn search(quotes: &[FinanceapiAutocomplete]) {
    let content: Vec<TableSearch> = quotes
        .iter()
        .map(|q| TableSearch {
            asset: &q.type_disp,
            symbol: Symbol::from_str(&q.symbol).unwrap(),
            exchange: &q.exch_disp,
            description: &q.name,
        })
        .collect();

    let mut table = Table::new(content);
    table.with(Style::sharp()).modify(
        Columns::last().intersect(Rows::new(1..)),
        Format::content(|s| s.green().to_string()),
    );

    println!("{}", table);
}

#[derive(Tabled, Default)]
struct TableInfo<'a> {
    asset: &'a str,
    #[tabled(rename = "ticker")]
    symbol: Symbol,
    currency: &'a str,
    price: Price,
    #[tabled(rename = "day gain")]
    day_gain: Gain,
    #[tabled(rename = "day gain (%)")]
    day_gain_perc: PercGain,
}

pub fn info(quote: &FinanceapiQuote) {
    let info = vec![TableInfo {
        asset: quote.type_disp.as_ref().unwrap(),
        symbol: Symbol::from_str(&quote.symbol).unwrap(),
        currency: quote.currency.as_ref().unwrap(),
        price: Price(quote.regular_market_price.unwrap_or_default()),
        day_gain: Gain(quote.regular_market_change.unwrap_or_default()),
        day_gain_perc: PercGain(quote.regular_market_change_percent.unwrap_or_default()),
    }];

    let mut table = Table::new(info);
    table.with(Style::sharp());

    println!("{}", table);
}

#[derive(Tabled, Default)]
struct TableList {
    #[tabled(rename = "portfolio")]
    symbol: Symbol,
    #[tabled(rename = "total gain")]
    tot_gain: Gain,
    #[tabled(rename = "total gain (%)")]
    tot_gain_perc: PercGain,
}

pub fn list(portfolio: &str, gain: f64, perc_gain: f64) {
    let list = vec![TableList {
        symbol: Symbol::from_str(portfolio).unwrap(),
        tot_gain: Gain(gain),
        tot_gain_perc: PercGain(perc_gain),
    }];

    let mut table = Table::new(list);
    table.with(Style::sharp());

    println!("{}", table);
}
