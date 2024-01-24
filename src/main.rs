mod data;
mod finance;

use anyhow::Result;
use data::CtxData;
use finance::YProvider;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn do_help() {
    todo!();
}

fn do_line(data: &mut CtxData, provider: &YProvider, line: &str) -> Result<()> {
    let v: Vec<&str> = line.split_whitespace().collect();

    let cmd = v[0];
    let (portfolio, ticker) = (v.get(1).copied(), v.get(2).copied());

    match cmd {
        "help" => {
            do_help();
        }

        "search" => {
            provider.search(ticker)?;
        }

        "list" => {
            data.list()?;
        }

        "info" => {
            provider.info(ticker)?;
        }

        "add" => {
            let cost_min = v.get(3).copied();
            let cost_max = v.get(4).copied();
            let cost_per = v.get(5).copied();

            data.add(portfolio, ticker, cost_min, cost_max, cost_per)?;
        }

        _ => {
            println!("Unknown command: \"{cmd}\"");
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    let mut data = CtxData::load()?;
    let provider = YProvider::new()?;

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) if !line.is_empty() => {
                do_line(&mut data, &provider, &line)?;
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!("Bye!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
            _ => {}
        }
    }

    Ok(())
}
