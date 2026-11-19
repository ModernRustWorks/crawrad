# CrawRAD Template Architecture and Specification

This directory standardizes all templates provided by CrawRAD, both built-in templates and custom templates.

## 1. Standardized Directory Structure

```text
templates/
├── desktop_gui/
│   ├── template.toml
│   └── template_files/
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs
│           └── app.rs
├── mobile_app/
│   ├── template.toml
│   └── template_files/
│       ├── Cargo.toml
│       ├── res/
│       │   └── icon.png
│       └── src/
│           └── main.rs
├── console_app/
│   ├── template.toml
│   └── template_files/
├── library_crate/
│   ├── template.toml
│   └── template_files/
├── wasm_app/
│   ├── template.toml
│   └── template_files/
├── multiwindow_app/
│   ├── template.toml
│   └── template_files/
├── package_plugin/
│   ├── template.toml
│   └── template_files/
└── custom_template/
    ├── template.toml
    └── template_files/
```

## 2. Template Discovery Locations

The IDE searches for templates in the following sources:
1. **Built-in**: `templates/` directory distributed with the IDE or embedded in the binary.
2. **User / Global (*Custom*)**:
   - Windows: `%APPDATA%\CrawRAD\templates\`
   - Linux / macOS: `~/.config/crawrad/templates/`
3. **Workspace Locations (*Workspace-specific*)**:
   - `.crawrad/templates/` inside the open repository.

## 3. Manifest Specification (`template.toml`)

Each template has a `template.toml` file at its root:

```toml
[template]
id = "mobile_app"
name = "Mobile App"
version = "1.0.0"
category = "mobile" # desktop | console | webassembly | mobile | library | embedded | examples
description = "A mobile application for Android and iOS using Makepad."
icon = "📱"
platforms = ["android", "ios"]
author = "CrawRAD Team"

[options]
include_example = { label = "Include example UI (counter app)", default = true }
live_reload = { label = "Use Makepad live-reload (dev mode)", default = true }
add_menu = { label = "Add basic application menu", default = false }
enable_tracing = { label = "Enable tracing (logging)", default = false }

[structure]
tree = """
{nome}/
├── Cargo.toml
├── src/
│   ├── main.rs
│   └── app.rs
└── res/
    └── icon.png
"""
```

## 4. Interpolation Variables

When generating files from the template, the following placeholders are automatically replaced:
- `{nome}`: Package name in snake_case (e.g. `meu_app`).
- `{organizacao}`: Organization identifier (e.g. `com.minhaempresa`).
- `{autor}`: Default author configured in Git or in the IDE options.
- `{ano}`: Current year (e.g. `2026`).
