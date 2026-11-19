---
id: menu-inject-menu-contributions
kind: doc
target_file: src/menu.rs
line_hint: 353
author: 
created: 
title: "inject_menu_contributions"
tags: []
signature: "pub fn inject_menu_contributions (target_menu : Menu , rows : & mut Vec < Row > , contributions : & [crawrad_extension_api :: contribution :: MenuContribution] , registry : Option < & crate :: capability :: CapabilityRegistry > , ctx : & Context ,)"
---

Injects extension menu contributions into the target menu.

Injects menu contributions (`MenuContribution`) declared by extensions into the target menu.

# Examples

```rust
use crawrad_actions::menu::{inject_menu_contributions, Menu};
use crawrad_actions::{Args, Author, Context, EditorText, Project, Row};
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
let mut rows = Vec::new();
inject_menu_contributions(Menu::File, &mut rows, &[], None, &ctx);
assert!(rows.is_empty());
```
