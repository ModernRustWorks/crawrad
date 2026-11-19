---
id: targets-makepad-components-emit-children
kind: doc
target_file: src/targets/makepad/components/mod.rs
line_hint: 430
author: 
created: 
title: "emit_children"
tags: []
signature: "pub fn emit_children (doc : & Document , reg : & crawrad_registry :: Registry , node : & Node , indent : & str ,) -> Result < String , Erro >"
---

Emits the children of a container, already indented and positioned.

**The children of a container**, already indented and already in the right coordinates.

# The coordinate is RELATIVE TO THE PARENT, and this is not a detail

It is the one `.crawform` stores, and the one the margin measures: the margin starts from
the parent's turtle, and the parent's turtle **already carries its scroll offset**. It is what
makes the child move when the user drags the scrollbar.

It was `rect_no_formulario` for a day, while the position
was emitted as `abs_pos` — and then it had to be the form's, because `abs_pos` is
absolute. Both things reverted together, and the reason is written in
[`super::layout::posicao`]: **what is absolute does not scroll.**

# And why it is ONE function

There were four copies of the same loop — `Panel`, `GroupBox`, `ScrollBox`, and the
`PageControl` page —, and all four were wrong in the same way. Fixing
four times means forgetting one.
**The `Child` verb of ADR 0036**,
and it was already generic — no line here knows class names.

# The flaw it had, which the typed collection revealed

It nested **every** child in `live_design!`, as if every child were a
widget. For area containers this is correct: the children of a `Panel`
appear inside it.

For a **collection** this is wrong, and the bug only failed to appear by luck: the only
collection in the catalog is `ActionList`, which has `gera_codigo: false` and therefore
never reaches here. A third-party grid declaring
`children = "collection"` with `item_class = "Column"` and **generating code** would have
its columns emitted as `<View>` in the widget strip — columns are not
windows, they are owner data.

The decision belongs to the PARENT's descriptor (`Filhos`, ADR 0037), and not a list
written here.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::emit_children;
use crawrad_ir::{Document, Node};
use crawrad_registry::Registry;

let doc = Document::new("MainForm", "Form");
let reg = Registry::builtin();
let node = Node::new("panel", "Panel");
let dsl = emit_children(&doc, &reg, &node, "    ").unwrap();
assert!(dsl.is_empty());
```

# Errors

Returns an error if child DSL generation fails.
