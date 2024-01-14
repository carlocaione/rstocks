use anyhow::Result;
use yahoo::YahooConnector;
use yahoo_finance_api as yahoo;
use tokio_test;

pub fn connect() -> Result<YahooConnector> {
    Ok(YahooConnector::new())
}

pub fn search(provider: &YahooConnector, params: Vec<&str>) -> Result<()> {
    let resp = tokio_test::block_on(provider.search_ticker("Apple")).unwrap();

    Ok(())
}
