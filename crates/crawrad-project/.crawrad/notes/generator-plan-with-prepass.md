---
id: generator-plan-with-prepass
kind: doc
target_file: src/generator.rs
line_hint: 191
author: 
created: 
title: "plan_with_prepass"
tags: []
signature: "pub fn plan_with_prepass (destination : Option < & Path > , ir : & Document , package : & str , editing : & dyn Fn (& Path) -> Option < String > ,) -> (Vec < crawrad_codegen :: Desligamento > , Result < Vec < crawrad_codegen :: Arquivo > , crawrad_codegen :: Erro > ,)"
---

Generates the plan and identifies unlinked event bindings.

The plan of `Generate` **and** the unlinking it already assumes — the two
together, because apart the order between them is only a comment.

# Why one function, and not two statements at the call site

The `.designer.rs` is our `.dfm`, and it is DERIVED from the document:
while the link exists, it emits `self.btn_okClick();`. The pruning takes
that `fn` out of the user's file in the same write. A plan built BEFORE the
unlinking delivers a designer that calls a method which has just left —

```text
error[E0599]: no method named `btn_okClick` found for mutable reference
              `&mut FormPrincipal` in the current scope
```

— measured with `cargo build` on a real project on 2026-09-08, and it is an
error in the CLIENT's project, not in ours. Written as "first this, then
that", the order was held by a comment: moving the pre-pass after the plan
kept the whole suite green, and that is how the review of PR #48 found the
hole.

# The plan does not depend on the transaction going through

The unlinking returned here is what the shell will apply to the document,
with `Ctrl+Z` — but the plan **already** assumes it. If the transaction is
refused (`Lock Controls`, a document that will not take it), the files
written stay coherent with each other: the designer does not call, and the
`fn` leaves, because the list of live handlers the pruning uses are the
additions of THIS plan (`crawrad_codegen::crescer_e_podar`). What is left is a
document one step behind, and the next generation recreates the stub —
which is what the link means.

`destination` is the member that owns the document, and `None` when there is
none: with no folder there is no user file to read, and the pre-pass
proposes nothing.

`editing` is the text on screen for a file with unsaved edits, or `None`
for the disk (#59). [`prepare`] writes the editor first and passes
nothing; the Explorer preview, which may not write, passes its buffers, so
both judge the same text.

# Examples

```rust
use crawrad_ir::Document;
use crawrad_project::generator::plan_with_prepass;

let doc = Document::new("MainForm", "Form");
let (unlinked, plan) = plan_with_prepass(None, &doc, "app", &|_| None);
assert!(unlinked.is_empty());
assert!(plan.is_ok());
```
