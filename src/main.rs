mod data;
mod finance;
mod cli;
mod ctx;

use anyhow::{anyhow, Result};
use ctx::Context;
use rustyline::error::ReadlineError;

fn do_line(ctx: &mut Context, line: &str) -> Result<()> {
    let mut v: Vec<&str> = line.split_whitespace().collect();

    let cmd = v.remove(0);

    // XXX: Array of function pointers?
    let ret = match cmd {
        "help" => cli::do_help(),
        "search" => ctx.provider.search(v),
        "list" => ctx.data.list(),
        "info" => ctx.provider.info(v),
        "add" => ctx.data.add(&ctx.provider, v),
        "entry" => ctx.data.entry(v),
        "show" => ctx.data.show(&ctx.provider, v),

        _ => Err(anyhow!("Unknown command: \"{cmd}\"")),
    };

    if let Err(err) = ret {
        // XXX: print usage again
        eprintln!("ERROR: {}", err);
    }

    Ok(())
}

fn main() -> Result<()> {
    let mut rl = Context::build_rl()?;
    let mut ctx = Context::build()?;

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) if !line.is_empty() => {
                do_line(&mut ctx, &line)?;
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
