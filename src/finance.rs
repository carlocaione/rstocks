use anyhow::{bail, Context, Result};
use yahoo::YahooConnector;
use yahoo_finance_api as yahoo;

use crate::table;

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

        table::search(&resp.quotes);
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

        table::info(&quote, &meta);
        Ok(())
    }
}
