use rustyline::{history::DefaultHistory, Editor};
use crate::{cli, cli::CommandHinter, data::CtxSavedData, finance::YProvider};
use anyhow::Result;

pub struct Context {
    pub data: CtxSavedData,
    pub provider: YProvider,
    pub rl: Editor<CommandHinter, DefaultHistory>,
}

impl Context {
    pub fn build() -> Result<Self> {
        let helper = CommandHinter {
            hints: cli::build_hints(),
        };

        let mut rl: Editor<CommandHinter, DefaultHistory> = Editor::new()?;
        rl.set_helper(Some(helper));

        let data = CtxSavedData::load()?;
        let provider = YProvider::new()?;

        Ok(Context { data, provider, rl })
    }
}
