---
id: menu-code-action-rows
kind: doc
target_file: src/menu.rs
line_hint: 617
author: 
created: 
title: "code_action_rows"
tags: []
signature: "pub fn code_action_rows (actions : & [(String , bool)] , teach : bool) -> Vec < Row >"
---

Builds menu rows for code actions and quick fixes offered at the caret.

The rows of the code editor's CONTEXT menu (#257), in the fixed order of
its groups — Navigation, View, Refactor, Clipboard, Selection, Command
Palette — so a later action finds its place. Only actions that exist:
no row for what does not run (the AI group waits for #259). The keys are
the active scheme's.
The code actions offered at the caret (#359), one row each: running a row
is `editor.apply_code_action` with its index. A preferred action is
marked, as VS Code marks it.
The quick fixes of the caret's place, and with `teach` (the line has a
compiler error) the teacher's "Fix and Explain" after a rule (#611).

# Examples

```rust
use crawrad_actions::menu::code_action_rows;

let rows = code_action_rows(&[("Import symbol".into(), true)], false);
assert_eq!(rows.len(), 1);
```
