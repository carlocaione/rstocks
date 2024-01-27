mod data;
mod finance;

use anyhow::Result;
use data::CtxData;
use finance::YProvider;
use rustyline::error::ReadlineError;
use rustyline::hint::{Hint, Hinter};
use rustyline::history::DefaultHistory;
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Completer, Editor, Helper, Highlighter};
use std::collections::HashSet;

#[derive(Completer, Helper, Highlighter)]
struct CommandHinter {
    hints: HashSet<CommandHint>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
struct CommandHint {
    display: String,
    complete_up_to: usize,
    mandatory_param: usize,
}

impl CommandHint {
    fn new(text: &str) -> CommandHint {
        let v: Vec<&str> = text.split_whitespace().collect();
        let mandatory_param = v.iter().skip(1).filter(|w| w.starts_with('<')).count();
        let complete_up_to = v[0].len();

        CommandHint {
            display: text.into(),
            complete_up_to,
            mandatory_param,
        }
    }

    fn suffix(&self, strip_chars: usize) -> CommandHint {
        CommandHint {
            display: self.display[strip_chars..].to_owned(),
            complete_up_to: self.complete_up_to.saturating_sub(strip_chars),
            mandatory_param: 0,
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

impl Validator for CommandHinter {
    fn validate(&self, ctx: &mut ValidationContext) -> rustyline::Result<ValidationResult> {
        use ValidationResult::{Invalid, Valid};
        let input = ctx.input();
        let vparam: Vec<&str> = input.split_whitespace().collect();
        let nparam_passed = vparam.len() - 1;

        let r = self.hints.iter().find_map(|hint| {
            if hint.display().starts_with(vparam[0]) {
                if nparam_passed >= hint.mandatory_param {
                    Some(Valid(None))
                } else {
                    Some(Invalid(Some(format!(
                        "\nMissing parameters. Usage: \"{}\"",
                        hint.display()
                    ))))
                }
            } else {
                None
            }
        });

        if let Some(rvalid) = r {
            Ok(rvalid)
        } else {
            Ok(Invalid(Some("\ncommand not found".to_owned())))
        }
    }

    fn validate_while_typing(&self) -> bool {
        false
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

    match cmd {
        "help" => {
            do_help();
        }

        "search" => {
            let ticker = v.get(1).copied();
            provider.search(ticker)?;
        }

        "list" => {
            data.list()?;
        }

        "info" => {
            let ticker = v.get(1).copied();
            provider.info(ticker)?;
        }

        "add" => {
            let portfolio = v.get(1).copied();
            let ticker = v.get(2).copied();
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
