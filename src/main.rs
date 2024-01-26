mod data;
mod finance;

use anyhow::Result;
use data::CtxData;
use finance::YProvider;
use rustyline::error::ReadlineError;
use rustyline::hint::{Hint, Hinter};
use rustyline::history::DefaultHistory;
use rustyline::{Completer, Editor, Helper, Highlighter, Validator};
use std::collections::HashSet;

// https://github.com/kkawakam/rustyline/blob/master/examples/diy_hints.rs

#[derive(Completer, Helper, Validator, Highlighter)]
struct CommandHinter {
    hints: HashSet<CommandHint>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct CommandHint {
    display: String,
    complete_up_to: usize,
}

impl CommandHint {
    fn new(text: &str) -> CommandHint {
        let cut = text.find(' ').map_or(text.len(), |l| l + 1);

        CommandHint {
            display: text.into(),
            complete_up_to: cut,
        }
    }

    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
        }
    }
}

impl Hint for CommandHint {
    fn display(&self) -> &str {
        &self.display
    }

    fn completion(&self) -> Option<&str> {
        if self.complete_up_to > 0 {
            Some(&self.display[..self.complete_up_to])
        } else {
            None
        }
    }
}

impl Hinter for CommandHinter {
    type Hint = CommandHint;

    fn hint(&self, line: &str, pos: usize, _ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        if line.is_empty() || pos < line.len() {
            return None;
        }

        self.hints.iter().find_map(|hint| {
            if hint.display.starts_with(line) {
                Some(hint.suffix(pos))
            } else {
                None
            }
        })
    }
}

fn build_hints() -> HashSet<CommandHint> {
    let mut set = HashSet::new();

    set.insert(CommandHint::new("help"));
    set.insert(CommandHint::new("search <ticker>"));
    set.insert(CommandHint::new("list"));
    set.insert(CommandHint::new("info <ticker>"));
    set.insert(CommandHint::new("show <portfolio>"));
    set.insert(CommandHint::new(
        "add <portfolio> [ticker] [cost_min] [cost_max] [cost_perc]",
    ));

    set
}

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
    let h = CommandHinter {
        hints: build_hints(),
    };

    let mut rl: Editor<CommandHinter, DefaultHistory> = Editor::new()?;
    rl.set_helper(Some(h));

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
