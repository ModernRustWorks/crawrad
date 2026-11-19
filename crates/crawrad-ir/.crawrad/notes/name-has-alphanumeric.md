---
id: name-has-alphanumeric
kind: doc
target_file: src/lib.rs
line_hint: 587
author: 
created: 
title: "name_has_alphanumeric"
tags: []
signature: "pub fn name_has_alphanumeric (name : & str) -> bool"
---

A `Name` requires at least one letter or digit.

# Examples

```rust
use crawrad_ir::name_has_alphanumeric;

assert!(name_has_alphanumeric("btn_ok"));
assert!(!name_has_alphanumeric("___"));
```

**This is not stylistic fussiness: it is what prevents user code
loss.** Every target derives an identifier from `Name` — the form becomes
type and module, the component becomes a field. The derivation is always from the same
family: splits the name on `_` and joins the parts. A name made only of underscores
has no parts, and the derivation comes out **empty**.

What an empty derivation did, measured: renaming the form to `___`
generated `impl  {`, and ordered the rename migrator to replace the word `FormMain`
with *nothing* inside the handwritten file. `fn _help(_f: &FormMain) {}`
became `fn _help(_f: &) {}`, and the user's project stopped compiling. The
`___` slipped through because `"___".trim()` is not empty, and because it **is** a
legitimate Rust identifier — it is just not one from which a type can be derived.

The rule lives here, and not in the generator, because it is target-neutral and
language-neutral: no target can name anything with this. What is
Rust-specific — ASCII, no spaces, outside reserved keywords —
remains in `crawrad-codegen`, which rejects before writing any file.
