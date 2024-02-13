use anyhow::Result;
use yahoo::YahooConnector;
use yahoo_finance_api as yahoo;

pub struct YProvider {
    // XXX getter?
    pub connector: YahooConnector,
}

impl YProvider {
    pub fn new() -> Result<YProvider> {
        Ok(YProvider {
            connector: YahooConnector::new(),
        })
    }

    pub fn search(&self, opt: Vec<&str>) -> Result<()> {
        let ticker = opt[0];

        let resp = tokio_test::block_on(self.connector.search_ticker(ticker))?;

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

    pub fn get_latest_quotes(&self, ticker: &str) -> Result<yahoo::YResponse> {
        Ok(tokio_test::block_on(self.connector.get_latest_quotes(ticker, "1d"))?)
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

    pub fn info(&self, opt: Vec<&str>) -> Result<()> {
        let ticker = opt[0];

        let quote = self.get_last_quote(ticker)?;
        let meta = self.get_metadata(ticker)?;

        println!(
            "{} | {} [{}] : {}",
            meta.instrument_type, meta.symbol, meta.currency, quote.close
        );

        Ok(())
    }
}
