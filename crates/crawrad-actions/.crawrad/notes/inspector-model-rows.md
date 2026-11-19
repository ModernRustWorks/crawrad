---
id: inspector-model-rows
kind: doc
target_file: src/inspector_model.rs
line_hint: 305
author: 
created: 
title: "rows"
tags: []
signature: "pub fn rows (registry : & Registry , doc : Option < & Document > , selection : & [NodeId] , target : & str , tab : Tab , open : & BTreeSet < String > , folded : & BTreeSet < String > ,) -> Vec < Row >"
---

The rows of the Object Inspector, in the order the screen shows them.

`target` is the runtime that draws the canvas: it filters the properties by
prefix, by the contract of ADR 0002 (`PropDesc::props_for`). No property of
the catalogue has a prefix today, so it cuts nothing — and that is exactly
why it has to be right from the start.

With **several** selected the list is the INTERSECTION: only the keys every
descriptor has. Where the value differs the row becomes a mute one — the
egui shell leaves the divergent text field editable
(`inspector.rs:213`), and here it is not: writing over two different values
from a field that was showing one of them is the change nobody asked for.

`tab` chooses WHAT the list brings — the properties or the events — and not
how it is drawn: the preamble (no form, no selection, no descriptor) is the
same in both, because the question *"what is this panel talking about?"* is
the same.

`open` is the set of property keys whose sub-rows are unfolded. It lives in
the shell and is moved by the `inspector.toggle_group` action; keyed by key
and not by `(component, key)` on purpose — `Margins` left open on one
component stays open on the next, which is what a classic RAD IDE does.

# Examples

```rust
use crawrad_actions::inspector_model::{rows, Tab};
use crawrad_registry::Registry;
use std::collections::BTreeSet;

let reg = Registry::builtin();
let open = BTreeSet::new();
let folded = BTreeSet::new();
let result = rows(&reg, None, &[], "", Tab::Properties, &open, &folded);
assert_eq!(result.len(), 1);
```
