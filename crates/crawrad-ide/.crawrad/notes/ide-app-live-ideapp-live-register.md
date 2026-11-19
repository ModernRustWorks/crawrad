---
id: ide-app-live-ideapp-live-register
kind: doc
target_file: src/ide_app/live.rs
line_hint: 15
author: 
created: 
title: "IdeApp::live_register"
tags: []
signature: "fn live_register (cx : & mut Cx)"
---

Registers every live module the shell's DSL links to, in dependency order.

THIS IS THE RIGHT HOOK, not a matter of taste: `#[derive(Live)]` generates
`live_design_with` calling `<Self as LiveRegister>::live_register(cx)`
(`makepad-derive-live-1.0.0/src/derive_live.rs:439-440`), and
`live_design!` calls each type's `live_design_with` BEFORE registering the
DSL body of the file (`live_design_macro.rs:29-30`). Registered here, every
`use link::…::*;` of the shell resolves. A `link` only resolves what was
already registered, hence the order:

1. `makepad_widgets` — the theme derives from its templates (`<Button>`,
   `<Label>`, `<TextInput>`).
2. The shell theme (`tema`), then the eleven modal dialogs, which derive
   from its moulds (`CrawradDialog`, `CrawradDialogBody`).
3. The minimap (`makepad-code-editor`) before the dock panels that use it,
   then the panels (`use link::crawrad_panels::*;`).
4. The code editor. Without it `<CodeView>` fails SILENTLY — `Can't find
   live definition of CodeView` goes only to the log, the view is EMPTY and
   no build complains; the `dsl_gate` probe reads the log and fails it. The
   Rust `use makepad_code_editor::code_view::CodeView;` registers nothing:
   it imports the TYPE, and the live definition lives in the framework's
   table, which only `live_design` feeds
   (`makepad-code-editor-1.0.0/src/lib.rs:28`).
5. The adapter's shaders and `DesignCanvas` (`use link::crawrad::*;`).
6. The dock tab NAMES in the framework's id table: without it the three
   tabs absent from the DSL come out of the saved layout as 64-bit numbers
   (measured; see `Panel::tab_name`).
7. The shell's main DSL (ADR 0043): window, bars, work area, central dock
   and dialogs.
