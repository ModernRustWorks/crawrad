# Changelog

Notable changes to CrawRAD. Each entry links the issue that tracks it.

## Planned — after the beta

These features are decided and tracked, and they are scheduled for after the beta.

- **Game projects** ([#7](https://github.com/ModernRustWorks/crawrad/issues/7)).
  - **File › New › Game**: design the game's flow (scenes, states, the loop, events) on a canvas.
  - CrawRAD generates readable Rust game code from that flow.
- **Python with Rust** ([#8](https://github.com/ModernRustWorks/crawrad/issues/8)).
  - A project template for Python extensions written in Rust (PyO3 + maturin).
  - Python as a first-class language in the editor, with project environments through `uv`.
  - A `PythonScript` component. A build option chooses whether the app ships its own Python.
- **An open code editor** ([#9](https://github.com/ModernRustWorks/crawrad/issues/9)).
  - Any language that has a language server, through installable **language packages**: the server, highlighting, formatter and snippets.
  - Rust becomes the first package, then Python, HTML/CSS/JS for WebAssembly projects, TOML and Markdown.
- **Editor colour schemes** ([#10](https://github.com/ModernRustWorks/crawrad/issues/10)).
  - Ready-made schemes, chosen like the keymap schemes.
  - Colours grouped by token type, overridable per language.
  - Schemes can be edited, exported and imported.
- **Git inside the IDE** ([#11](https://github.com/ModernRustWorks/crawrad/issues/11)).
  - A source control panel (stage, commit, amend) and changed lines marked in the editor's gutter.
  - Branches, fetch/pull/push, history and blame, and conflict resolution in the editor.
  - The chat agent can do all of it through MCP.
- **A docked terminal** ([#3](https://github.com/ModernRustWorks/crawrad/issues/3)).
  - The human's terminal panel inside the IDE, on `alacritty_terminal`, drawn by CrawRAD itself.
  - Several tabs, a shell chosen in the options, and drivable by the chat agent.
- **Installable project templates** ([#12](https://github.com/ModernRustWorks/crawrad/issues/12)).
  - **New Project › Other…** lists every template in a category tree, with a folder-structure preview.
  - A template is a package with metadata and wizard options. It can be installed from a Git URL, crates.io or a folder, and an open project can be saved as a template.
  - A public gallery offers ready skeletons, e.g. Axum MVC or MVVM, desktop MVC, CLI.
- **Code Quality: custom lints** ([#13](https://github.com/ModernRustWorks/crawrad/issues/13)).
  - One page for Rust and Clippy lints, written to the `[lints]` table of `Cargo.toml` so `cargo` and CI obey the same settings.
  - Custom rules from the team, a company or a component package: declarative patterns, or analyzers in their own process.
  - Findings in the editor and the Problems panel with quick fixes, a baseline for old projects, profiles, and `crawrad lint` for CI.
