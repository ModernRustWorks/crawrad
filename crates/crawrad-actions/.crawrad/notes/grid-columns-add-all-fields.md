---
id: grid-columns-add-all-fields
kind: doc
target_file: src/grid_columns.rs
line_hint: 68
author: 
created: 
title: "add_all_fields"
tags: []
signature: "pub fn add_all_fields (c : & Context , grid : & str , fields : & [& str] , kinds : & [& str]) -> Effect"
---

Adds one column per field that has none yet, in ONE undo step (#936).

`kinds` holds each field's type in the same order (`INTEGER`, `REAL`...):
a whole number is right-aligned and 70 wide, a real number right-aligned
with the `#,##0.00` mask. A component without grid columns, or no fields,
is refused with a status message and no change.

# Examples

```rust
use crawrad_actions::grid_columns::add_all_fields;
# use crawrad_actions::{Args, Author, Context, Project};
# use crawrad_designer::clipboard::Clipboard;
# use crawrad_doc::Doc;
# use crawrad_ir::{Document, Node};
# use crawrad_registry::Registry;
let mut form = Document::new("form_main", "Form");
form.add_child("form_main", Node::new("grid1", "Grid")).unwrap();
let (doc, reg, clip) = (Doc::new(form), Registry::builtin(), Clipboard::default());
let c = Context {
    doc: Some(&doc),
    registry: &reg,
    clipboard: &clip,
    args: Args::none(),
    project: Project::default(),
    author: Author::User,
    editor_dirty: false,
    text: Default::default(),
};
let effect = add_all_fields(&c, "grid1", &["id", "balance"], &["INTEGER", "REAL"]);
assert!(effect.transaction.is_some());
```
