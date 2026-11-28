---
id: workspace-recent-menu-items
kind: doc
target_file: src/workspace.rs
line_hint: 254
author: 
created: 
title: "recent_menu_items"
tags: []
signature: "pub fn recent_menu_items () -> Vec < (String, PathBuf) >"
---

The recent projects as a menu wants them: a label and the folder it opens.

Built from `welcome_rows` on purpose — the project the Welcome page calls
`crawrad` must be called `crawrad` in the File menu too, and a second notion of a
project's name is how two places start disagreeing about one thing. It lives here
beside the list it reads rather than in the shell, which is under a downward line
ratchet and should not carry data work.

# Examples

```rust
// Whatever the machine's list holds, every row can open something.
for (label, folder) in crawrad_project::workspace::recent_menu_items() {
    assert!(!label.is_empty());
    assert!(folder.is_absolute() || folder.components().count() > 0);
}
```
