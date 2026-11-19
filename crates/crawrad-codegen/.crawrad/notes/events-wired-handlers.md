---
id: events-wired-handlers
kind: doc
target_file: src/events.rs
line_hint: 113
author: 
created: 
title: "wired_handlers"
tags: []
signature: "pub fn wired_handlers < 'a > (root : & 'a Node , emitted : & [& 'a Node]) -> Result < Vec < Handler < 'a > > , Erro >"
---

Returns the deduplicated handlers wired in the document.

**The handlers this document gives the user's file**, deduplicated by `fn`
name and in the order the stubs are written.

Two wirings can point at the same `fn` name — the user typed the same name
twice in the Object Inspector, or two components share a handler on
purpose. Only one `fn` can be written, and the one whose `(component,
event)` pair **derives** that name is the one that gets to decide the
signature: it is the pair CrawRAD itself would have named that way, so its
parameters are the ones the user is going to expect.

# Examples

```rust
use crawrad_codegen::events::wired_handlers;
use crawrad_ir::Node;

let root = Node::new("form", "Form");
let handlers = wired_handlers(&root, &[]).unwrap();
assert!(handlers.is_empty());
```

# Errors

Returns an error if any handler target is not a valid Rust identifier.
