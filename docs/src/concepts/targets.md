# Targets: native, web and mobile

CrawRAD does not interpret your form at run time. It **generates** a normal
Rust project from it, written for one UI framework: the project's **target**.
Two targets are available today:

| Target | What it is | Platforms |
|---|---|---|
| **Makepad** | GPU-drawn UI with live reload of the design, shaders and effects. | Windows, Linux, macOS, Web, Mobile |
| **egui** | Immediate-mode UI, small and quick to build. | Windows, Linux, macOS, Web |

You choose the target when you create the project. After that, CrawRAD reads
it from the project's `Cargo.toml`: a project that depends on `makepad-widgets`
is a Makepad project, and one that depends on `eframe` is an egui project.

## The same form, every target

A form does not change with the target. The components, their properties and
your event code stay the same. Only the generated designer file is written for
a different framework.

## Web projects

A web project builds to WebAssembly. **Run** builds it and opens it in your
browser.
