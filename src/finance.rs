use anyhow::{bail, Context, Result};
use financeapi::{FinanceapiConnector, FinanceapiQuote};

use crate::table;

pub struct YProvider {
    pub connector: FinanceapiConnector,
}

impl YProvider {
    pub fn new<T: Into<String>>(key: T) -> YProvider {
        YProvider {
            connector: FinanceapiConnector::new(key),
        }
    }

    pub fn search(&self, opt: &[&str]) -> Result<()> {
        let ticker = opt[0];
        let resp = tokio_test::block_on(self.connector.autocomplete(ticker))?;

        if resp.is_empty() {
            bail!("{ticker} not found");
        }

        table::search(&resp);
        Ok(())
    }

    pub fn get_quote(&self, ticker: &str) -> Result<FinanceapiQuote> {
        Ok(tokio_test::block_on(self.connector.quote(ticker))?)
    }

    pub fn exists(&self, ticker: &str) -> bool {
        self.get_quote(ticker).is_ok()
    }

    pub fn info(&self, opt: &[&str]) -> Result<()> {
        let ticker = opt[0];

        let quote = self
            .get_quote(ticker)
            .with_context(|| format!("{ticker} not found"))?;

        table::info(&quote);
        Ok(())
    }
}
