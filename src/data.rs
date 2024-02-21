use anyhow::{bail, Context, Result};
use chrono::naive::serde::ts_seconds;
use chrono::prelude::*;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::finance::YProvider;

static PROGNAME: &str = env!("CARGO_PKG_NAME");

#[derive(Serialize, Deserialize, Debug, Default)]
struct AssetOp {
    quantity: u32,
    price: f64,
    #[serde(with = "ts_seconds")]
    date: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct AssetCost {
    min: Option<f32>,
    max: Option<f32>,
    per: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct AssetData {
    cost: Option<AssetCost>,
    op: Vec<AssetOp>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct PortfolioData {
    amount: f32,
    asset: HashMap<String, AssetData>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct SavedData {
    portfolio: HashMap<String, PortfolioData>,
}

impl SavedData {
    fn save<P: AsRef<Path>>(&self, file: P) -> Result<()> {
        let toml = toml::to_string(self)?;
        fs::write(file, toml)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct CtxSavedData {
    saved: SavedData,
    file: PathBuf,
}

fn convert<T>(input: &[&str], index: usize) -> Result<Option<T>, T::Err>
where
    T: FromStr,
{
    input
        .get(index)
        .copied()
        .map(|x| x.parse::<T>())
        .transpose()
}

impl CtxSavedData {
    pub fn load() -> Result<Self> {
        let datadir = ProjectDirs::from("", "", PROGNAME)
            .context("Failed to get project directory")?
            .data_dir()
            .to_owned();

        fs::create_dir_all(&datadir)?;

        let mut datafile = datadir.join(PROGNAME);
        datafile.set_extension("dat");

        let data = if datafile.exists() {
            let tomldata = fs::read_to_string(&datafile)?;
            toml::from_str(&tomldata)?
        } else {
            File::create(&datafile)?;
            let s = SavedData::default();
            s.save(&datafile)?;
            s
        };

        Ok(CtxSavedData {
            saved: data,
            file: datafile,
        })
    }

    pub fn add(&mut self, yprovider: &YProvider, opt: &[&str]) -> Result<()> {
        let portfolio = opt[0];
        let ticker = opt.get(1).copied();

        let pdata = self
            .saved
            .portfolio
            .entry(portfolio.to_string())
            .or_default();

        if let Some(ticker) = ticker {
            let assetdata = pdata.asset.entry(ticker.to_string()).or_default();

            if !yprovider.exists(ticker) {
                bail!("Ticker {ticker} does not exist\n");
            }

            let min: Option<f32> = convert(opt, 2).context("The minimum cost must be a number")?;
            let max: Option<f32> = convert(opt, 3).context("The maximum cost must be a number")?;
            let per: Option<f32> = convert(opt, 4).context("Invalid percentage")?;

            assetdata.cost = Some(AssetCost { min, max, per });
        }

        self.saved.save(&self.file)?;
        Ok(())
    }

    pub fn entry(&mut self, opt: &[&str]) -> Result<()> {
        let portfolio = opt[0];
        let ticker = opt[1];

        let assetop = &mut self
            .saved
            .portfolio
            .get_mut(portfolio)
            .with_context(|| format!("portfolio \"{portfolio}\" not found"))?
            .asset
            .get_mut(ticker)
            .with_context(|| format!("ticker \"{ticker}\" not found"))?
            .op;

        let quantity: u32 = convert(opt, 3).context("Invalid quantity")?.unwrap();
        let price: f64 = convert(opt, 4).context("Invalid price")?.unwrap();

        let date = opt.get(5).copied();
        let date = match date {
            None => Utc::now().date_naive(),
            Some(x) => {
                NaiveDate::parse_from_str(x, "%d/%m/%Y").context("Wrong date format: dd/mm/yy\n")?
            }
        };
        let date = date.and_hms_opt(0, 0, 0).unwrap();

        assetop.push(AssetOp {
            quantity,
            price,
            date,
        });

        self.saved.save(&self.file)?;
        Ok(())
    }

    pub fn show(&mut self, yprovider: &YProvider, opt: &[&str]) -> Result<()> {
        let portfolio = opt[0];
        let assets = &self
            .saved
            .portfolio
            .get(portfolio)
            .with_context(|| format!("portfolio \"{portfolio}\" not found"))?
            .asset;

        let mut res = 0.0;
        for (ticker, assetdata) in assets {
            let quote = yprovider.get_last_quote(ticker)?;

            res = assetdata
                .op
                .iter()
                .fold(res, |acc, x| acc + (quote.close - x.price));
        }

        println!("==> {}\n", res);

        Ok(())
    }

    pub fn list(&self) -> Result<()> {
        println!("{:#?}", self);
        Ok(())
    }
}
