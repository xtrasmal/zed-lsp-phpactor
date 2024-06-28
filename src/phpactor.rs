use std::{env, fs};
use zed_extension_api::{self as zed, Result};

struct PhpactorExtension {}

impl zed::Extension for PhpactorExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        todo!()
    }

    fn language_server_command(
        &mut self,
        config: zed_extension_api::LanguageServerConfig,
        worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<zed_extension_api::Command> {
        todo!()
    }

    fn language_server_initialization_options(
        &mut self,
        _config: zed_extension_api::LanguageServerConfig,
        _worktree: &zed_extension_api::Worktree,
    ) -> zed_extension_api::Result<Option<String>> {
        Ok(None)
    }
}

zed::register_extension!(PhpactorExtension);
