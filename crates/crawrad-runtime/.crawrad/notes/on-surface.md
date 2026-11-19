---
id: on-surface
kind: doc
target_file: src/lib.rs
line_hint: 234
author: 
created: 
title: "on_surface"
tags: []
signature: "pub fn on_surface (doc : & Document , id : & str , is_visual : impl Fn (& str) -> bool) -> bool"
---

Returns whether the node appears on the drawing surface.

Does the node appear on the drawing surface?

Lives here, at the boundary, because **the designer and the adapter need the same
answer** — and they already diverged. When containers arrived, the tree
started being walked in four places and I changed one; and the non-visual
rule existed twice, with the designer asking the catalog and the
adapter carrying a hand-written list. A rule that two paths
need to know is a rule that must exist once.

What changes between the two is only **where** "this class is visual" comes from: the
designer asks the catalog, the adapter knows what it knows how to render. That is why it
comes in as a parameter instead of being built-in.

Two reasons for not appearing, and they are independent:

1. **Lives inside a non-visual.** An `Action` belongs to the `ActionList`
   editor, as in the classic RAD IDE, where actions were never on the
   form.
2. **Lives on a page that is not the active one** — the conditional visibility
   amendment of ADR 0012. The node remains in the document and remains in `Structure`;
   it is just not on the screen.
3. **Lives in a collapsed `CategoryPanel`** (#500): folded into the header, it
   hides children like a closed page.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_runtime::on_surface;

let doc = Document::new("form", "Form");
assert!(on_surface(&doc, "form", |_| true));
```
