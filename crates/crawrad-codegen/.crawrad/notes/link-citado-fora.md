---
id: link-citado-fora
kind: doc
target_file: src/link.rs
line_hint: 160
author: 
created: 
title: "citado_fora"
tags: []
signature: "pub fn citado_fora (fonte : & str , nome : & str , dono : Range < usize >) -> bool"
---

Checks whether an identifier is referenced outside the item's own snippet.

Does anyone **reference** `nome` outside the `dono` snippet?

The question that a classic RAD IDE does not ask. It deletes empty handlers without looking at who
calls them, and a hand-written `Button1Click(nil)` inside another handler becomes
a compilation error in a file the user didn't touch. Here the file is
in hand and the answer costs a single scan — so it is done.

# Examples

```rust
use crawrad_codegen::link::citado_fora;

let src = "fn foo() { bar(); } fn bar() {}";
assert!(citado_fora(src, "bar", 20..31));
```

The search is performed on the **masked** source and by whole word, for the same two
reasons as always: a `// remember to call btn_okClick` is not a call, and
`btn_okClickDuplo` is not the same name.

`dono` is the range of the item itself, which is excluded — the definition references its
own name, and without this nothing would ever be removable.
