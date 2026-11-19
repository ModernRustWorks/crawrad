---
id: link-handlers-do-arquivo
kind: doc
target_file: src/link.rs
line_hint: 65
author: 
created: 
title: "handlers_do_arquivo"
tags: []
signature: "pub fn handlers_do_arquivo (fonte : & str) -> Vec < (String , Local) >"
---

Every `fn` defined in the file, in the order it appears.

# Examples

```rust
use crawrad_codegen::link::handlers_do_arquivo;

let src = "fn foo() {} fn bar() {}";
let handlers = handlers_do_arquivo(src);
assert_eq!(handlers.len(), 2);
```

It is the inventory that the IDE crosses with the document's `events`: what is here
and not there is orphaned (the user disconnected the event and the code remained); what
is there and not here is a handler to create.

Lists **all** `fn`s, not just the ones that look like a handler. A helper that the
user wrote looks exactly like an orphaned handler, and separating
the two would require guessing — which is what ADR 0015 forbids. Duplicates
stay on the list on purpose: repeated names are the ambiguity that
[`localizar_handler`] resolved by choosing the first.

# This list alone can NEVER feed a removal

It is two paragraphs above and the reason this warning exists: *a helper
that the user wrote looks exactly like an orphaned handler*. Pruning
crosses this list with the **document** and with the mark that the generator
left above the stub; whoever takes what comes out of here as "what can be
deleted" deletes the `fn calcular_total` that someone wrote by hand.
