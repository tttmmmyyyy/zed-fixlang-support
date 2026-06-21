# Change Log

## [0.2.0] - 2026-06-21

- Forward `lsp.fix.settings` to the Fix language server (via
  `language_server_workspace_configuration`), so the `fix.analyze.delayMs` and
  `fix.analyze.onSave` analysis settings can be configured from Zed.

## [0.1.0] - 2026-06-20

- Initial release.
- Register the `.fix` language: line/block comments, brackets, and autoclose.
- Run the Fix language server (`fix language-server`), providing diagnostics,
  completion, go-to-definition, find-references, document & workspace symbols,
  code actions, rename, and call hierarchy.
- Semantic-token syntax highlighting, with default token colors that follow the
  active theme.
- `lsp.fix.binary` setting to override the `fix` binary path and arguments.
