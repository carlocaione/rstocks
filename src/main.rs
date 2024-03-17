mod cli;
mod data;
mod finance;
mod table;

use anyhow::{anyhow, Result};
use data::CtxSavedData;
use finance::YProvider;
use rustyline::error::ReadlineError;

fn do_line(data: &mut CtxSavedData, yprovider: &YProvider, line: &str) -> Result<()> {
    let mut v: Vec<&str> = line.split_whitespace().collect();

    let cmd = v.remove(0);

    let ret = match cmd {
        "help" => cli::do_help(),
        "search" => yprovider.search(&v),
        "list" => data.list(yprovider),
        "info" => yprovider.info(&v),
        "add" => data.add(yprovider, &v),
        "entry" => data.entry(&v),
        "show" => data.show(yprovider, &v),

        _ => Err(anyhow!("Unknown command: \"{cmd}\"")),
    };

    if let Err(err) = ret {
        // XXX: print usage again
        eprintln!("ERROR: {}", err);
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut rl = cli::build_editor()?;
    let mut data = CtxSavedData::load()?;
    let yprovider = YProvider::new("cBII4JLYGw9lzlMBsyyrM41X9aHLpaUI83ctFXdZ");

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) if !line.is_empty() => {
                do_line(&mut data, &yprovider, &line)?;
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Bye!");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
