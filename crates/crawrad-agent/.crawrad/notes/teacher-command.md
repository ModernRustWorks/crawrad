---
id: teacher-command
kind: doc
target_file: src/teacher.rs
line_hint: 243
author: 
created: 
title: "command"
tags: []
signature: "pub fn command (id : & str , rest : & str , lang : & str , selected : bool , grounding : Option < Grounding > ,) -> Reply"
---

Dispatches a teacher command with arguments and contextual grounding.

The teacher command `id` (`teach`, `why`, `fix-explain`, `compare`) with
its argument `rest`: `selected` says whether code is selected, `grounding`
is the caret's diagnostic (only read by `why` and `fix-explain`).

# Examples

```rust
use crawrad_agent::teacher::{Reply, command};

let reply = command("compare", "Rc Arc", "English", false, None);
match reply {
    Reply::Prompt(prompt) => assert!(prompt.contains("Rc")),
    Reply::Say(_) => unreachable!(),
}
```
