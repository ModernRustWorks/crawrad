---
id: key-routing-not-here
kind: doc
target_file: src/key_routing.rs
line_hint: 175
author: 
created: 
title: "not_here"
tags: []
signature: "pub fn not_here (id : & str , editor_focused : bool , code_in_front : bool) -> bool"
---

Returns whether the key action should be ignored in the current focus.

**Does this chord belong to the TEXT EDITOR, and not the designer?**

# The defect this blocks, and it causes work loss

The `handle_key_down` of this shell receives **every** `Event::KeyDown`: the focus
filter is `event.hits`, and only widgets call that. While the code view
was read-only, this was ugly and not critical. Since the editor
edits (`super::editor_buffer`), it became critical:

| key | what the finger asked for | what happened |
|---|---|---|
| `Ctrl+Z` | undo the typed line | undid the form's last TRANSACTION |
| `Del` | delete the character | deleted the selected COMPONENT |
| `Ctrl+C` | copy the text | copied the component to the designer area |

And both happened in the same press, because the editor also handles it: the
user saw the right text and did not see the form changing behind it.

# Why the criterion is the GROUP, and not a list of ids

Because the group is what the catalog already specifies. `Edit` are the actions that operate
on *what is currently selected*, and with the cursor in a file what is
selected is text — which has an owner. A list of ids written here would be the
second table that [ADR 0011] exists to forbid, and it would fall behind
the day the ninth `edit.*` was born.

Actions OUTSIDE the group continue to pass through, and must continue to: `Ctrl+S`, `F5`,
`F12`, and `Ctrl+Shift+B` belong to the IDE, and the IDE does not cease to exist because the
cursor is in a file.

The key's action does not run here: the editor has the focus and the
action is its editing keyboard (the system's events do it, see
[`e_do_editor_de_texto`]), or the action is the other view's twin
([`serves_view`]).

# Examples

```rust
use crawrad_actions::key_routing::not_here;

assert!(not_here("edit.undo", true, true));
```
