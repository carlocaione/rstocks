use anyhow::{Context, Result};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::fs::File;
use std::path::PathBuf;

static PROGNAME: &'static str = env!("CARGO_PKG_NAME");

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
struct Asset {
    ticker: String,
    cost: Option<AssetCost>,
    op: Vec<AssetOp>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Portfolio {
    name: String,
    asset: Vec<Asset>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct SavedData {
    portfolio: Vec<Portfolio>,
}

impl SavedData {
    // TODO: manage PathBuf vs Path
    fn save(&self, file: &PathBuf) -> Result<()> {
        let toml = toml::to_string(self)?;
        fs::write(file, &toml)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct CtxData {
    saved: SavedData,
    file: PathBuf,
}

impl CtxData {
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

        Ok(CtxData {
            saved: data,
            file: datafile,
        })
    }

    pub fn add_portfolio(&mut self, params: Vec<&str>) -> Result<()> {
        let name = params[0];

        if !self.saved.portfolio.iter().any(|x| x.name == name) {
            self.saved.portfolio.push(Portfolio {
                asset: vec![],
                name: name.to_owned(),
            });
            self.saved.save(&self.file)?;
        }

        Ok(())
    }

    pub fn add_asset(&mut self, params: Vec<&str>) -> Result<()> {
        let (portfolio, ticker): (&str, &str) = (params[0], params[1]);

        Ok(())
    }
}
