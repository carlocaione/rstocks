use serde::{Serialize, Deserialize};
use anyhow::{Context, Result};
use directories::ProjectDirs;
use std::fs;
use std::fs::OpenOptions;
use std::path::PathBuf;

static PROGNAME: &'static str = env!("CARGO_PKG_NAME");

#[derive(Serialize, Deserialize, Debug)]
struct AssetOp {
    is_buy: bool,
    quantity: u32,
    price: f32,
    date: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct AssetCost {
    min: Option<f32>,
    max: Option<f32>,
    per: Option<f32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Asset {
    ticker: String,
    cost: Option<AssetCost>,
    op: Option<Vec<AssetOp>>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Portfolio {
    name: String,
    asset: Option<Vec<Asset>>, 
}

impl PartialEq for Portfolio {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialEq<str> for Portfolio {
    fn eq(&self, other: &str) -> bool {
        self.name == other
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct SavedData {
    portfolio: Option<Vec<Portfolio>>,
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

        OpenOptions::new().write(true)
            .create(true)
            .open(&datafile)?;

        let tomldata = fs::read_to_string(&datafile)?;
        let data: SavedData = toml::from_str(&tomldata)?;

        let ctx = CtxData {
            saved: data,
            file: datafile,
        };

        println!("{:?}", ctx);

        Ok(ctx)
    }

    fn save(&self) -> Result<()> {
        let toml = toml::to_string(&self.saved)?;
        fs::write(&self.file, &toml)?;

        println!("{}", toml);

        Ok(())
    }

    pub fn add_portfolio(&mut self, params: Vec<&str>) -> Result<()> {
        let p = Portfolio {
            asset: None,
            name: String::from(params[0]),
        };

        if let Some(ref mut portfolio) = self.saved.portfolio {
            if !portfolio.contains(&p) {
                portfolio.push(p);
            }
        } else {
            self.saved.portfolio = Some(vec![p]);
        }

        self.save()?;

        Ok(())
    }

    pub fn add_asset(&mut self, params: Vec<&str>) -> Result<()> {
        let portfolio = params[0];

        Ok(())
    }
}
