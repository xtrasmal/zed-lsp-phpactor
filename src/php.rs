use std::{env, fs};
use zed::LanguageServerConfig;
use zed_extension_api::{self as zed, Result};

const SERVER_PATH: &str = "/usr/local/bin/phpactor";
const PACKAGE_NAME: &str = "phpactor";

struct PhpactorExtension {
    did_find_server: bool,
}

impl PhpactorExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }
        let path = worktree
            .which(PACKAGE_NAME)
            .ok_or_else(|| "phpactor must be installed manually.".to_string())?;

        self.did_find_server = true;
        Ok(path)
    }
}

impl zed::Extension for PhpactorExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        _config: LanguageServerConfig,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(worktree)?;
        let command = zed::Command {
            command: self.server_script_path(worktree)?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "language-server".to_string(),
            ],
            env: worktree.shell_env(),
        };
        Ok(command)
    }
}

zed::register_extension!(PhpactorExtension);
