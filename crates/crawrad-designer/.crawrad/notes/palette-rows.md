---
id: palette-rows
kind: doc
target_file: src/palette.rs
line_hint: 89
author: 
created: 
title: "rows"
tags: []
signature: "pub fn rows (registry : & Registry , target : & str , filter : & str , web : bool) -> Vec < Row >"
---

Returns the filtered Toolbox rows in display order.

The Toolbox rows, in the order the screen shows them, already filtered by
the text in the `Filter` field.

The order is that of the catalog and not a choice here: `Registry::palette`
returns groups in a `BTreeMap` and items in class order
(`crates/crawrad-registry/src/lib.rs:544-552`). The `target` filters by
the runtimes that the descriptor declares supporting (`ComponentDesc::supports`).

The `filter` is the raw text of the field — the caller does not need to trim or
lower-case, and that is deliberate: normalization done in two places is
two rules to diverge. A blank filter returns the entire catalog.

**A group that the filter emptied does not become a dangling header**, for the same reason
as in egui's Toolbox (`crates/crawrad-ide/src/paleta.rs:265-269`): ADR
0014 already states that an empty region is not drawn. And when nothing matches, the list
is not left blank — it has a [`Row::Empty`].

In a web project (`web`), a component that cannot work in the browser stays
listed but greyed: its row carries the reason from [`web_block`].

# Examples

```rust
use crawrad_designer::palette::rows;
use crawrad_registry::Registry;

let reg = Registry::builtin();
let list = rows(&reg, "makepad", "", false);
assert!(!list.is_empty());
```
