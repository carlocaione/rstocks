mod cli;
mod data;
mod finance;

use anyhow::{anyhow, Result};
use cli::CommandHinter;
use data::CtxSavedData;
use finance::YProvider;
use rustyline::error::ReadlineError;
use rustyline::{history::DefaultHistory, Editor};

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
    let helper = CommandHinter {
        hints: cli::build_hints(),
    };

    let mut rl: Editor<CommandHinter, DefaultHistory> = Editor::new()?;
    rl.set_helper(Some(helper));

    let mut data = CtxSavedData::load()?;
    let yprovider = YProvider::new();

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
