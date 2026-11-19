# Project templates

This page is generated from the project template catalogue: every template File > New Project offers.

## GUI Application

Creates a new Rust application with a graphical user interface using the Makepad framework. It starts with a blank form, open in the Designer, and the code generated from it.

Platforms: Windows, Linux, macOS.

Files created for a project named `my_app` (a Cargo workspace with the project inside it):

```text
my_app/
├── Cargo.toml   (workspace)
├── .gitignore
└── my_app/
    ├── app.rc
    ├── build.rs
    ├── Cargo.toml
    ├── icon.ico
    ├── icon.png
    └── src/
        ├── form_main.designer.rs
        ├── form_main.crawform
        ├── form_main.rs
        └── main.rs
```

## Console Application

Creates a lightweight Rust command-line tool with clean argument parsing, error handling, and terminal output. No UI dependencies or GPU overhead.

Platforms: Windows, Linux, macOS.

Files created for a project named `my_app` (a Cargo workspace with the project inside it):

```text
my_app/
├── Cargo.toml   (workspace)
├── .gitignore
└── my_app/
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Library Crate

Creates a reusable Rust crate structured for sharing components, models, and business logic across projects. Includes crawrad.toml manifest and unit tests.

Platforms: Windows, Linux, macOS.

Files created for a project named `my_app` (a Cargo workspace with the project inside it):

```text
my_app/
├── Cargo.toml   (workspace)
├── .gitignore
└── my_app/
    ├── Cargo.toml
    ├── crawrad.toml
    └── src/
        └── lib.rs
```

## Web App (WASM)

Compiles directly to WebAssembly and runs in any modern browser via Makepad WebGL/WebGPU backend. Zero client installation required.

Platforms: WebAssembly (Browser).

Files created for a project named `my_app` (a Cargo workspace with the project inside it):

```text
my_app/
├── Cargo.toml   (workspace)
├── .gitignore
└── my_app/
    ├── Cargo.toml
    └── src/
        ├── form_main.designer.rs
        ├── form_main.crawform
        ├── form_main.rs
        └── main.rs
```

## Mobile App

Cross-platform mobile application targeting Android and iOS with native touch gestures, responsive layout, and mobile window management.

Platforms: Android, iOS.

Files created for a project named `my_app` (a Cargo workspace with the project inside it):

```text
my_app/
├── Cargo.toml   (workspace)
├── .gitignore
└── my_app/
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   └── app.rs
    └── res/
        └── icon.png
```

## Multi-window Desktop

Advanced desktop application with support for multiple native OS windows, dockable viewports, RAD forms, and custom window chrome.

Platforms: Windows, Linux, macOS.

Files created for a project named `my_app` (a Cargo workspace with the project inside it):

```text
my_app/
├── Cargo.toml   (workspace)
├── .gitignore
└── my_app/
    ├── app.rc
    ├── build.rs
    ├── Cargo.toml
    ├── forms/
    │   └── main.crawform
    ├── icon.ico
    ├── icon.png
    └── src/
        ├── app.rs
        └── main.rs
```

## Package / Plugin

Extends CrawRAD IDE with custom component palettes, visual inspectors, code generator backends, and tooling integrations.

Platforms: Windows, Linux, macOS.

Files created for a project named `my_app` (a Cargo workspace with the project inside it):

```text
my_app/
├── Cargo.toml   (workspace)
├── .gitignore
└── my_app/
    ├── Cargo.toml
    ├── plugin.toml
    └── src/
        └── lib.rs
```

## Custom Template

Start from an empty or custom template scaffold with flexible Cargo configuration and customizable project directory layout.

Platforms: Windows, Linux, macOS, WASM.

Files created for a project named `my_app` (a Cargo workspace with the project inside it):

```text
my_app/
├── Cargo.toml   (workspace)
├── .gitignore
└── my_app/
    ├── app.rc
    ├── build.rs
    ├── Cargo.toml
    ├── icon.ico
    ├── icon.png
    └── src/
        ├── app.rs
        └── main.rs
```
