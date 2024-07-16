use zed_extension_api::{self as zed, Result};

struct PhpExtension;

impl zed::Extension for PhpExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _config: zed::LanguageServerConfig,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("phpactor")
            .ok_or_else(|| "phpactor is not installed".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["language-server".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(PhpExtension);
