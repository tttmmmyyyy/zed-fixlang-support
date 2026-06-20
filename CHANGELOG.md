# Change Log

## [0.1.0] - 2026-06-20

- Initial release.
- Register the `.fix` language: line/block comments, brackets, and autoclose.
- Run the Fix language server (`fix language-server`), providing diagnostics,
  completion, go-to-definition, find-references, document & workspace symbols,
  code actions, rename, and call hierarchy.
- Semantic-token syntax highlighting, with default token colors that follow the
  active theme.
- `lsp.fix.binary` setting to override the `fix` binary path and arguments.
