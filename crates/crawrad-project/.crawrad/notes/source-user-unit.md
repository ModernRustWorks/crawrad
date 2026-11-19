---
id: source-user-unit
kind: doc
target_file: src/source.rs
line_hint: 120
author: 
created: 
title: "user_unit"
tags: []
signature: "pub fn user_unit (form : & Path , doc : Option < & crawrad_ir :: Document >) -> String"
---

The unit as it is on disk or the skeleton the generator would write.

The unit as it is ON DISK — or, before the first generation, the SKELETON
the generator would write.

# Why the skeleton, and not a sentence

The owner named it looking at the screen:

> *"this `.rs` in egui already had the initial header, which was the form's
> code, so that event code could be added in
> sequence"*
>
> *"we need to bring this back: when creating a new project, already have this
> header with a `use` for the form, and leave it ready to add the event
> blocks"*

A view that answers *"no generated unit on disk yet"* is technically true
and useless: the thing the user came to see is WHERE his code goes, and
that shape exists before any file does.

# And it is DERIVED, never a second copy

The skeleton comes out of the same `gerar()` call that
[`form_source`] uses, picking the file the generator marks as
[`crawrad_codegen::Dono::Usuario`]. Writing a header here would be the
generator's contract spelled twice — and the copy here would be the one to
age, because nobody generates from this function.

A banner says it is not on disk yet. Showing the skeleton without saying so
would be the opposite lie: the user would look for a file that is not
there.

# Examples

```rust
use std::path::Path;
use crawrad_project::source::{user_unit, NOT_GENERATED_YET};

let unit = user_unit(Path::new("non_existent.crawform"), None);
assert_eq!(unit, NOT_GENERATED_YET);
```
