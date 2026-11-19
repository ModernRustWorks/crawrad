---
id: generator-generate-for-new-project
kind: doc
target_file: src/generator.rs
line_hint: 328
author: 
created: 
title: "generate_for_new_project"
tags: []
signature: "pub fn generate_for_new_project (member : & Path , doc : & Document) -> Result < () , String >"
---

Writes the initial code for a newly created project.

Write the code of a **freshly created** project — the first `Generate`,
without the button.

**Why it exists at all.** a classic RAD IDE hands you `Unit1.pas` already written, and
nobody has ever seen a classic RAD IDE say "this file does not exist yet, press a
button". Measured in the IDE: create a project, double click a `Button`,
and get the invitation screen instead of the handler — in the first minute
of use, which is the minute that decides whether the product looks
finished.

It is **not a third generation path**: the plan comes from [`plan_for`] and
the writing from [`crawrad_codegen::escrever`], which are exactly the two calls
of the `Generate` on the bar. What changes is only who calls.

There is no pre-pass here, and that is not an omission: the folder was born
a second ago, there is no user file on disk for a link to have expired
against, and [`plan_with_prepass`] would return an empty list after reading
an empty folder.

The ownership rule stays with `escrever`, and it is `escrever` that applies
it: this is the **first** write into a folder that has just been born, so
every file enters as created. The `src/{form}.rs` — the user's file, the one
that promises *"This file is yours"* — is born here and is never overwritten
again, because what decides that is the plan's owner mark and not the
caller.

`Err` is the sentence for the bar, **in English**: the project already
exists on disk by now, and undoing it because of the code would throw away
what did work out.

# Errors

Returns an error sentence if generating or writing the project files fails.

# Examples

```rust,no_run
use std::path::Path;
use crawrad_ir::Document;
use crawrad_project::generator::generate_for_new_project;

let doc = Document::new("MainForm", "Form");
let res = generate_for_new_project(Path::new("."), &doc);
assert!(res.is_ok());
```
