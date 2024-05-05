use zed_extension_api::{self as zed, Result,LanguageServerId};

struct TyposExtension {
}

impl zed::Extension for TyposExtension {
    fn new() -> Self {
        Self {}
    }
    // fo

    fn language_server_command(
        &mut self,
        _: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("typos-lsp")
            .ok_or_else(|| "Must install https://github.com/tekumara/typos-lsp".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![],
        })
    }

}

zed::register_extension!(TyposExtension);
