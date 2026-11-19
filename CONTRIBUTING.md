# Contributing to CrawRAD

Thank you for helping. Before your first pull request can be merged:

1. **Sign the Contributor License Agreement.** Read [`CLA.md`](CLA.md). A bot asks you
   to accept it on your first pull request; until you do, the pull request cannot be
   merged. You sign once, and it covers every later contribution.
2. **Send only code you wrote**, or code whose licence you have checked is compatible
   and whose origin you name in the pull request.
3. **Run the gate** before you push: `./gate.ps1 -Fast -NoDocker` (format, clippy,
   tests). The CI runs the full gate on Windows, Linux and macOS.

## Licensing in one line

The IDE is GPL-3.0; the crates that go into the apps people build (runtime, binding,
component SDK, `components/`) are MIT OR Apache-2.0; the code CrawRAD generates belongs
to its user. See the licence section of the [`README`](README.md).

## Makepad component rules

When you work on the `makepad` target:

1. **Makepad DSL.** Always use `link widgets; use link::widgets::*; use link::theme::*;`.
   Children inside a `<View>` always carry explicit ids (`item_0 = ...`). Use `<Window>`
   at the App level.
2. **Containers.** A `PageControl` always draws a visible tab strip with every
   `TabSheet` and a content area with the active page. Components of inactive pages never
   cover the active one. Scroll bars come from `<ScrollBars>`.
3. **Idiomatic Rust.** `snake_case` everywhere. Never add `#[allow(non_snake_case)]` or
   any other warning suppression. Generated code compiles with zero warnings.
4. **One source of truth.** The component and property catalogue is `crawrad-registry`.
   No loose hand-written tables, no monolithic `match` blocks and no central
   `if class == "..."` chains in `form.rs` or `actions.rs`. Each component in
   `crates/crawrad-codegen/src/targets/makepad/components/<component>.rs` settles its own
   cycle: `struct_definition`, `emit_default`, `emit_dsl`, `emit_sync`, `emit_actions`.
   `form.rs` and `actions.rs` only orchestrate.
