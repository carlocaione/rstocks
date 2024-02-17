use rustyline::{history::DefaultHistory, Editor};
use crate::{cli, cli::CommandHinter, data::CtxSavedData, finance::YProvider};
use anyhow::Result;

pub struct Context {
    pub data: CtxSavedData,
    pub provider: YProvider,
}

impl Context {
    pub fn build() -> Result<Self> {
        let data = CtxSavedData::load()?;
        let provider = YProvider::new()?;

        Ok(Context { data, provider })
    }

    pub fn build_rl() -> Result<Editor<CommandHinter, DefaultHistory>> {
        let helper = CommandHinter {
            hints: cli::build_hints(),
        };

        let mut rl: Editor<CommandHinter, DefaultHistory> = Editor::new()?;
        rl.set_helper(Some(helper));

        Ok(rl)
    }
}
