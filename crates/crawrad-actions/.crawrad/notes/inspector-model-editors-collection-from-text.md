---
id: inspector-model-editors-collection-from-text
kind: doc
target_file: src/inspector_model/editors/collection.rs
line_hint: 56
author: 
created: 
title: "from_text"
tags: []
signature: "pub fn from_text (text : & str , hierarchical : bool) -> Vec < Item >"
---

The lines of the pad read back as a collection.

# The two rules, and the one thing they cost

1. **A blank line is not an item.** An item with no text is a row of a
   `ListBox` that cannot be clicked and cannot be told apart from its
   neighbour; whoever left a blank line between two items was spacing them
   out, not asking for a third.
2. **A child cannot be more than one level deeper than its parent.** Two
   levels of indent under a leaf is a typo, and the alternative — inventing
   an empty parent to hang it from — would put in the document something
   nobody typed. The line is clamped to the deepest place it can legally go.

**What this shape cannot spell**: an item whose own text begins with
spaces. It is the price of the indentation carrying the structure, it is
declared here rather than discovered later, and it is the same trade every
indentation-bearing format makes. A collection that needs it is a collection
that needs a grid, which is an issue of its own.

`flat` folds every line to the top level: a `ListBox` has no nesting to
read, so indentation in its pad is whitespace and nothing more.

# Examples

```rust
use crawrad_actions::inspector_model::editors::collection::from_text;

let items = from_text("item1\nitem2", false);
assert_eq!(items.len(), 2);
```
