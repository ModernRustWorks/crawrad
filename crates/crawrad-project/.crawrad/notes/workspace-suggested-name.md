---
id: workspace-suggested-name
kind: doc
target_file: src/workspace.rs
line_hint: 1798
author: 
created: 
title: "suggested_name"
tags: []
signature: "pub fn suggested_name (raw : & str) -> String"
---

The crate name that arbitrary text suggests.

**Formerly `crate_name(&Path)`**, which *decided* the package name: it came
sanitized from the chosen folder. With typed names ([ADR 0017], Amendment 3)
it stopped deciding and began to **suggest** — it is the second half of
[`refusal_typed_name`]'s phrase, and the difference between the IDE returning
the problem (*"that is invalid"*) and returning the solution (*"try `meu_app`"*).

`My App` is a perfectly normal name for a person, and previously the IDE
translated it in silence. Translating in silence was right when the name came
from the folder and no one had chosen it; with a typed name, changing what the
person wrote without saying so is the IDE deciding for them.

The rule: lowercase, non-letter/digit becomes `_`, and a leading digit gets
a leading `_` — Rust identifiers cannot start with a number.


# Examples

```rust
use crawrad_project::workspace::suggested_name;

assert_eq!(suggested_name("My App"), "my_app");
assert_eq!(suggested_name("123app"), "_123app");
```
