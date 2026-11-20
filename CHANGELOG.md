# Changelog

Notable changes to CrawRAD. Each entry links the issue that tracks it.

## 0.1.1 — fixes

- **First start:** the buttons of the Environment Check (install, cancel) answer the mouse again — in 0.1.0 they ignored clicks, so the check could not be completed. The same fix reaches the Form Check and Notes dialogs.
- **macOS:** the file watcher ignores `target/` also under the real path of a temporary or symlinked folder (`/private/var/...`), so build output no longer floods the IDE.
- **Keymap:** the default scheme is "Modern" everywhere, and keyboard shortcuts use the platform's primary key (Cmd on macOS).

## 0.1.0 — first public release (beta)

CrawRAD's first public release: a visual RAD IDE for Rust. Design a form, write
its events, and get an ordinary Rust project you own. The full list is in
[ROADMAP.md](ROADMAP.md) under **Today**. The highlights:

- **Visual designer.**
  - Form Designer and Object Inspector with typed property editors.
  - Visual form inheritance.
  - A form checker with one-click fixes.
  - The Sizes and Translations tabs.
- **51 components.** A Data Grid bound to a real table at design time, a Dataset on CrawRAD's own data engine, and your own components with no restart.
- **Code editor.**
  - rust-analyzer: completion, hover, code actions and diagnostics.
  - Code templates.
  - Local file history with diff and restore.
  - Notes kept outside the source.
- **Build, run and test.**
  - A build window with a real Cancel.
  - Desktop and WebAssembly runs.
  - Your app's panics in the Problems panel.
  - A Tests panel.
  - Seven project templates.
- **AI and automation.** An MCP server that drives the whole IDE, and a chat that sees the form and the selection.
- **The user manual:** [isaquepinheiro.com.br/crawrad/manual](https://isaquepinheiro.com.br/crawrad/manual/).

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
