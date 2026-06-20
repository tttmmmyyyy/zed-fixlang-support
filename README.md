# Fix language support for Zed

> [!IMPORTANT]
> ## ⚠️ Add this setting to get syntax highlighting
>
> Highlighting for `.fix` files comes from the language server's LSP semantic
> tokens, which Zed leaves **disabled by default**. To use them, add this to your
> Zed `settings.json` (or a project-level `.zed/settings.json`):
>
> ```json
> {
>   "languages": {
>     "Fix": {
>       "semantic_tokens": "full"
>     }
>   }
> }
> ```
>
> See [Enable syntax highlighting](#enable-syntax-highlighting) for details.

A Zed extension for the [Fix programming language](https://github.com/tttmmmyyyy/fixlang). It:

- registers the `.fix` language, and
- launches the Fix language server (`fix language-server`), giving you
  diagnostics, completion, go-to-definition, find-references, document &
  workspace symbols, code actions, rename, call hierarchy, and semantic-token
  syntax highlighting.

## Requirements

- `fix` on your `PATH` (or configured via settings, see below). Syntax
  highlighting needs a `fix` whose language server supports semantic tokens.
- Zed with LSP semantic-token support (the `semantic_tokens` setting).

## Install (as a dev extension)

1. `zed: install dev extension` from the command palette.
2. Select this directory (`zed-plugin`).

## Enable syntax highlighting

Highlighting for Fix is provided by the language server's LSP semantic tokens.
Zed leaves them off by default, so turn them on for Fix in your Zed
`settings.json`:

```json
{
  "languages": {
    "Fix": {
      "semantic_tokens": "full"
    }
  }
}
```

- `"full"` — highlighting comes solely from the language server (recommended).
- `"combined"` — LSP semantic tokens layered on top of tree-sitter highlighting.
- `"off"` (default) — semantic tokens are not requested.

After changing this setting, fully quit and reopen Zed (`Ctrl-Q`) so the language
server is re-initialized with semantic tokens enabled.

Token colors follow Zed's built-in defaults for the standard LSP token types and
your active theme. To customize them, add rules under
`global_lsp_settings.semantic_token_rules` in `settings.json`.

## Configuring the `fix` binary (optional)

By default the extension runs `fix language-server` using the `fix` found on
your `PATH`. To point at a specific binary or pass different arguments, add to
`settings.json`:

```json
{
  "lsp": {
    "fix": {
      "binary": {
        "path": "/absolute/path/to/fix",
        "arguments": ["language-server"]
      }
    }
  }
}
```
