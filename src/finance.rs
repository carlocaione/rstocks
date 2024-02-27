use anyhow::{bail, Context, Result};
use owo_colors::OwoColorize;
use tabled::{builder::Builder, settings::Style};
use yahoo::YahooConnector;
use yahoo_finance_api as yahoo;

pub struct YProvider {
    pub connector: YahooConnector,
}

impl YProvider {
    pub fn new() -> YProvider {
        YProvider {
            connector: YahooConnector::new(),
        }
    }

    pub fn search(&self, opt: &[&str]) -> Result<()> {
        let ticker = opt[0];
        let resp = tokio_test::block_on(self.connector.search_ticker(ticker))?;

        if resp.quotes.is_empty() {
            bail!("{ticker} not found");
        }

        for q in resp.quotes {
            let desc = if q.long_name.is_empty() {
                q.short_name
            } else {
                q.long_name
            };

            let mut builder = Builder::default();
            builder.push_record(vec![&q.type_display, &q.symbol, &q.exchange, &desc]);
            let mut table = builder.build();
            table.with(Style::modern());
        }

        Ok(())
    }

    pub fn get_latest_quotes(&self, ticker: &str) -> Result<yahoo::YResponse> {
        Ok(tokio_test::block_on(
            self.connector.get_latest_quotes(ticker, "1d"),
        )?)
    }

    pub fn exists(&self, ticker: &str) -> bool {
        self.get_metadata(ticker).is_ok()
    }

    pub fn get_metadata(&self, ticker: &str) -> Result<yahoo::YMetaData> {
        Ok(self.get_latest_quotes(ticker)?.metadata()?)
    }

    pub fn get_last_quote(&self, ticker: &str) -> Result<yahoo::Quote> {
        Ok(self.get_latest_quotes(ticker)?.last_quote()?)
    }

    pub fn info(&self, opt: &[&str]) -> Result<()> {
        let ticker = opt[0];

        let quote = self
            .get_last_quote(ticker)
            .with_context(|| format!("{ticker} not found"))?;
        let meta = self.get_metadata(ticker)?;

        let mut builder = Builder::default();
        builder.push_record(vec![
            &meta.instrument_type.on_red().to_string(),
            &meta.symbol,
            &meta.currency,
            &quote.close.to_string(),
        ]);
        let mut table = builder.build();
        table.with(Style::modern());

        println!("{}", table);

        Ok(())
    }
}
