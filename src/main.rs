mod finance;

use anyhow::Result;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

fn do_line(line: &str) -> Result<()> {
    let provider = finance::connect()?;

    let v: Vec<&str> = line.split_whitespace().collect();

    let cmd = v[0];
    let params = v[1..].to_vec();

    match cmd {
        "search" => {
            finance::search(&provider, params)?;
        }

        "show" => {
            finance::show(&provider, params)?;
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

    rl.clear_screen()?;

    // TODO: maybe while let?
    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) if !line.is_empty() => {
                do_line(&line)?;
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
