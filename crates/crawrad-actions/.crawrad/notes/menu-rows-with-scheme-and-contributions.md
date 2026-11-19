---
id: menu-rows-with-scheme-and-contributions
kind: doc
target_file: src/menu.rs
line_hint: 281
author: 
created: 
title: "rows_with_scheme_and_contributions"
tags: []
signature: "pub fn rows_with_scheme_and_contributions (menu : Menu , ctx : & Context , scheme : crate :: KeymapScheme , registry : Option < & crate :: capability :: CapabilityRegistry > , contributions : & [crawrad_extension_api :: contribution :: MenuContribution] ,) -> Vec < Row >"
---

The rows of a menu, including dynamic contributions from extensions.

# Examples

```rust
use crawrad_actions::menu::{rows_with_scheme_and_contributions, Menu};
use crawrad_actions::{Args, Author, Context, EditorText, KeymapScheme, Project};
use crawrad_designer::clipboard::Clipboard;
use crawrad_registry::Registry;

let registry = Registry::builtin();
let clipboard = Clipboard::default();
let ctx = Context {
    doc: None,
    registry: &registry,
    clipboard: &clipboard,
    args: Args::none(),
    project: Project { has_binary: false, ..Project::default() },
    author: Author::User,
    editor_dirty: false,
    text: EditorText::default(),
};
let rows = rows_with_scheme_and_contributions(Menu::File, &ctx, KeymapScheme::Modern, None, &[]);
assert!(!rows.is_empty());
```
