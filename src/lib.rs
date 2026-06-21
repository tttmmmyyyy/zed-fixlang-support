//! Zed extension for the Fix programming language.
//!
//! Registers the `.fix` language and launches the Fix language server
//! (`fix language-server`). Highlighting comes from the server's semantic
//! tokens; see the README for enabling them in Zed.

use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

struct FixExtension;

impl zed::Extension for FixExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // Allow users to override the binary via Zed settings, e.g.:
        //   "lsp": { "fix": { "binary": { "path": "/path/to/fix",
        //                                  "arguments": ["language-server"] } } }
        let binary = LspSettings::for_worktree("fix", worktree)
            .ok()
            .and_then(|settings| settings.binary);

        let command = match binary.as_ref().and_then(|b| b.path.clone()) {
            Some(path) => path,
            // Otherwise look for `fix` on the worktree's PATH.
            None => worktree.which("fix").ok_or_else(|| {
                "could not find the `fix` executable in PATH. Install Fix \
                 (https://github.com/tttmmmyyyy/fixlang), or set \
                 `lsp.fix.binary.path` in your Zed settings."
                    .to_string()
            })?,
        };

        let args = binary
            .as_ref()
            .and_then(|b| b.arguments.clone())
            .unwrap_or_else(|| vec!["language-server".to_string()]);

        let env = binary
            .and_then(|b| b.env)
            .map(|env| env.into_iter().collect())
            .unwrap_or_default();

        Ok(zed::Command { command, args, env })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        // Forward the user's Zed `lsp.fix.settings` (e.g.
        //   "lsp": { "fix": { "settings": { "fix": { "analyze": {
        //       "delayMs": 300, "onSave": false } } } } }
        // ) to the Fix language server as its workspace configuration.
        let settings = LspSettings::for_worktree("fix", worktree)
            .ok()
            .and_then(|s| s.settings);
        Ok(settings)
    }
}

zed::register_extension!(FixExtension);
