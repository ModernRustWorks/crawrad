---
id: menu-editor-keys
kind: doc
target_file: src/menu.rs
line_hint: 613
author: 
created: 
title: "EDITOR_KEYS"
tags: []
signature: "pub const EDITOR_KEYS : & [(& str , crate :: KeymapScheme , & str)] = & [(\"editor.rename_symbol\" , crate :: KeymapScheme :: VsCode , \"F2\")]"
---

Special keys handled directly by the code editor mapped by keymap scheme.

Keys the code editor takes itself, by scheme, that the catalog cannot
carry because another action owns the key elsewhere: VS Code's F2 renames
a symbol in the editor and a file in the Explorer (#258). The editor's
menu shows them; `editor/keys.rs` maps them.

# Examples

```rust
use crawrad_actions::menu::EDITOR_KEYS;

assert!(!EDITOR_KEYS.is_empty());
```
