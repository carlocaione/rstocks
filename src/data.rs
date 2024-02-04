use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::path::{Path, PathBuf};

static PROGNAME: &str = env!("CARGO_PKG_NAME");

#[derive(Serialize, Deserialize, Debug, Default)]
struct AssetOp {
    is_buy: bool,
    quantity: u32,
    price: f32,
    date: u64,
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

    pub fn add(
        &mut self,
        portfolio: Option<&str>,
        ticker: Option<&str>,
        cost_min: Option<&str>,
        cost_max: Option<&str>,
        cost_per: Option<&str>,
    ) -> Result<()> {
        let portfolio = portfolio.context("Portfolio not found")?.to_string();
        let pdata = self.saved.portfolio.entry(portfolio).or_default();

        if let Some(ticker) = ticker {
            let assetdata = pdata.asset.entry(ticker.to_string()).or_default();

            // XXX: manage error print
            let min: Option<f32> = cost_min.map(|x| x.parse::<f32>()).transpose()?;
            let max: Option<f32> = cost_max.map(|x| x.parse::<f32>()).transpose()?;
            let per: Option<f32> = cost_per.map(|x| x.parse::<f32>()).transpose()?;

            assetdata.cost = Some(AssetCost { min, max, per });
        }

        self.saved.save(&self.file)?;
        Ok(())
    }

    pub fn list(&self) -> Result<()> {
        println!("{:#?}", self);
        Ok(())
    }
}
