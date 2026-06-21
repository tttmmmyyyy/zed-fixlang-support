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
> See [Configuration → Syntax highlighting](#syntax-highlighting) for details.

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

## Configuration

Settings go in Zed's `settings.json` — open it with `Ctrl-,` or the command
palette's `zed: open settings`. For per-project settings, use a
`.zed/settings.json` at the project root.

### Syntax highlighting

Highlighting for Fix is provided by the language server's LSP semantic tokens.
Zed leaves them off by default, so turn them on for Fix:

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

### Analysis (diagnostics)

The language server re-analyzes your code shortly after you stop typing (the
burst of edits is debounced into a single run). Configure it under the `fix`
language server's `settings`:

```json
{
  "lsp": {
    "fix": {
      "settings": {
        "analyze": {
          "delayMs": 400,
          "onSave": true
        }
      }
    }
  }
}
```

- `delayMs` (number, default `400`) — how long the server waits for typing to
  pause before re-analyzing. `0` disables on-type analysis (it then runs only on
  save and on initial load).
- `onSave` (boolean, default `true`) — whether saving a file also triggers
  analysis.

### Fix binary (optional)

By default the extension runs `fix language-server` using the `fix` found on
your `PATH`. To point at a specific binary or pass different arguments:

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
