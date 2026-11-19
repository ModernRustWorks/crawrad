---
id: vinculos-vencidos
kind: doc
target_file: src/lib.rs
line_hint: 210
author: 
created: 
title: "vinculos_vencidos"
tags: []
signature: "pub fn vinculos_vencidos (raiz : & Path , doc : & crawrad_ir :: Document) -> Vec < Desligamento >"
---

Identifies bindings pointing to empty handlers that must be removed.

**The pruning pre-pass:** bindings that must leave the document BEFORE
generation begins, because the handler on the other side of them is empty.

# Examples

```rust
use std::path::Path;
use crawrad_codegen::vinculos_vencidos;
use crawrad_ir::Document;

let doc = Document::new("MainForm", "Form");
let vencidos = vinculos_vencidos(Path::new("."), &doc);
assert!(vencidos.is_empty());
```

Returns what the IDE should disconnect; **writes nothing and changes nothing**.
What changes a document is `crawrad-doc`, which is the repository's only write gate,
and the generator does not depend on it nor should it come to depend. The shell takes this
list, builds **one** `Transaction` and passes it through `aplicar` — and that is where
`Ctrl+Z` comes from, which a classic RAD IDE does not have.

# Why a separate pass, and not everything inside [`escrever`]

**Because `.designer.rs` is derived from the document.** As long as the binding
exists, it emits the call — `self.btn_okClick();` — and it is rewritten
in its entirety on every generation. Removing the `fn` from the user's file in the same pass
in which the designer comes out with the call produces a project that **does not compile**.
Measured on 08/09/2026 with `cargo build` over the sample app generated
in a real project:

```text
error[E0599]: no method named `btn_okClick` found for mutable reference
              `&mut FormPrincipal` in the current scope
   --> srcormPrincipal.designer.rs:312:22
```

In a classic RAD IDE, the pair leaves together because the `.dfm` is a file that is **edited**:
saving removes the method from the `.pas` and the `OnClick =` line from the `.dfm` in the same
gesture. Here the designer is not edited, it is **re-emitted** — so the only order
that works out is: decide, mutate the document, and only then generate. Whoever "simplifies"
this into `escrever` brings `E0599` back.

# Newly created stub immunity comes from here for free

Notice what this function reads: the **disk**. The stub that double click just
requested is not there yet when it runs — the `Generate` triggered by the double
click writes *afterwards* —, so it is not a candidate, without a single line
saying so. It is the separation that a classic RAD IDE gets for free,
because there creating does not save: **it is created in one gesture, cleaned up in the next
save**. If that were made into a written condition, it would be the third condition that
this feature already paid a high price not to need.

# The rule is the same, and it is a single one

What decides is the same guard chain of [`rxp::podar`]
([`rxp::podaveis`], with empty `protegidos`): generator marker, name
derived by [`handler_padrao`], event in the `EVENTOS` table, empty body,
nothing of the user's glued above the marker, nobody mentioning the name here nor in
neighbors. What left the contract on 08/09/2026 was **only** the requirement that the
event be disconnected — the owner's measurement in his classic RAD IDE: *"a classic RAD IDE,
regardless of whether the link on the object is connected or not, if it finds it empty it
cleans it up"*.

A method that is not a handler remains invisible here, and a handler with a `//` in the
body remains the user's — both exits remain valid, now
also with the event connected, which is a new path.

# What it returns when it does not know

An empty list: a document without a root, a root whose name is not a Rust
identifier (generation will fail later anyway), and — the common case —
**a user file that does not yet exist**, which is every first generation.
Not knowing the text is a reason to propose nothing.
