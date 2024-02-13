use rustyline::hint::{Hint, Hinter};
use rustyline::validate::{ValidationContext, ValidationResult, Validator};
use rustyline::{Completer, Helper, Highlighter};
use std::collections::HashSet;
use anyhow::Result;

#[derive(Completer, Helper, Highlighter)]
pub struct CommandHinter {
    pub hints: HashSet<CommandHint>,
}

#[derive(Hash, Debug, PartialEq, Eq)]
pub struct CommandHint {
    display: String,
    complete_up_to: usize,
    mandatory_param: usize,
}

impl CommandHint {
    pub fn new(text: &str) -> CommandHint {
        let v: Vec<&str> = text.split_whitespace().collect();
        let mandatory_param = v.iter().skip(1).filter(|w| w.starts_with('<')).count();
        let complete_up_to = v[0].len();

        CommandHint {
            display: text.into(),
            complete_up_to,
            mandatory_param,
        }
    }

    pub fn suffix(&self, strip_chars: usize) -> CommandHint {
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

        if input.is_empty() {
            return Ok(Valid(None));
        }

        Ok(self
            .hints
            .iter()
            .find_map(|hint| {
                let vparam: Vec<&str> = input.split_whitespace().collect();
                let nparam_passed = vparam.len() - 1;

                if hint.display().starts_with(vparam[0]) {
                    if nparam_passed >= hint.mandatory_param {
                        Some(Valid(None))
                    } else {
                        Some(Invalid(Some(format!(
                            "\nMissing parameters.\n\nUsage: \"{}\"",
                            hint.display()
                        ))))
                    }
                } else {
                    None
                }
            })
            .unwrap_or(Invalid(Some(("\nCommand not found").to_string()))))
    }

    fn validate_while_typing(&self) -> bool {
        false
    }
}

pub fn build_hints() -> HashSet<CommandHint> {
    let mut set = HashSet::new();

    set.insert(CommandHint::new("help"));
    set.insert(CommandHint::new("search <ticker>"));
    set.insert(CommandHint::new("list"));
    set.insert(CommandHint::new("info <ticker>"));
    set.insert(CommandHint::new("show <portfolio>"));
    set.insert(CommandHint::new(
        "add <portfolio> [ticker] [cost_min] [cost_max] [cost_perc]",
    ));
    set.insert(CommandHint::new(
        "entry <portfolio> <ticker> <buy | sell> <quantity> [purchase_price] [purchase_date]",
    ));

    set
}

pub fn do_help() -> Result<()> {
    Ok(())
}

