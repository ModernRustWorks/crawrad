---
id: action-list-action-new
kind: doc
target_file: src/action_list.rs
line_hint: 25
author: 
created: 
title: "ACTION_NEW"
tags: []
signature: "pub const ACTION_NEW : & str = \"action.new\""
---

The THREE CATALOGUE ACTIONS that this window's buttons trigger.

They live here, in the feature module, and not loose in the shell — it is the
same place and the same reason as `inspector::ACTION_OPEN_EDITOR`: a command
id written in the middle of drawing is a second list of commands, and the
`the_shell_writes_no_command_id` guard rejects it. `New Action` and `Delete`
are argumentless commands — the SAME ones the menu already offers —, and that
is precisely why their id cannot be a literal in the shell.

# Examples

```rust
use crawrad_actions::action_list::ACTION_NEW;

assert_eq!(ACTION_NEW, "action.new");
```
