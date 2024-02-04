use anyhow::{Context, Result};
use yahoo::YahooConnector;
use yahoo_finance_api as yahoo;

pub struct YProvider {
    connector: YahooConnector,
}

impl YProvider {
    pub fn new() -> Result<YProvider> {
        Ok(YProvider {
            connector: YahooConnector::new(),
        })
    }

    pub fn search(&self, ticker: Option<&str>) -> Result<()> {
        let t = ticker.context("Ticker not found")?;
        let resp = tokio_test::block_on(self.connector.search_ticker(t))?;

        for q in resp.quotes {
            let desc = if q.long_name.is_empty() {
                q.short_name
            } else {
                q.long_name
            };
            println!(
                "{} \t| {} [{}]\t| {}",
                q.type_display, q.symbol, q.exchange, desc
            );
        }

        Ok(())
    }

    pub fn info(&self, ticker: Option<&str>) -> Result<()> {
        let t = ticker.context("Ticker not found")?;
        let response = tokio_test::block_on(self.connector.get_latest_quotes(t, "1d"))?;
        let quote = response.last_quote()?;
        let meta = response.metadata()?;

        println!(
            "{} | {} [{}] : {}",
            meta.instrument_type, meta.symbol, meta.currency, quote.close
        );

        Ok(())
    }
}
