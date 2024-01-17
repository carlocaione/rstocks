mod finance;
mod data;

use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use finance::YProvider;
use data::CtxData;

fn do_line(data: &mut CtxData, provider: &YProvider, line: &str) -> Result<()> {
    let v: Vec<&str> = line.split_whitespace().collect();

    let cmd = v[0];
    let params = v[1..].to_vec();

    match cmd {
        "search" => {
            provider.search(params)?;
        }

        "show" => {
            provider.show(params)?;
        }

        "new" => {
            data.add_portfolio(params)?;
        }
        _ => {
            println!("Unknown command: \"{cmd}\"");
        }
    }

    Ok(())
}

fn main() -> Result<()> {
    // TODO: Add input validator
    let mut rl = DefaultEditor::new()?;

    let provider = YProvider::new()?;

    let mut data = CtxData::load()?;

    // TODO: maybe while let?
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
