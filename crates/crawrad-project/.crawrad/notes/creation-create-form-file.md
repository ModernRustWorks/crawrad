---
id: creation-create-form-file
kind: doc
target_file: src/creation.rs
line_hint: 537
author: 
created: 
title: "create_form_file"
tags: []
signature: "pub fn create_form_file (folder : & Path, reg : & crawrad_registry :: Registry) -> Result < CreatedForm, String >"
---

Creates the next free form in a folder, **and the code that goes with it**.

The code is born with the form, exactly as it is when a project is created. Until
27/11/2026 this wrote the `.crawform` alone: `cargo` never saw the form, the
Explorer had no unit to list, and the only way to make the form real was to find
`Generate` — which nothing on screen said was needed. A form that compiles into
nothing is not a form yet.

A form outside a crate member (`<member>/src/`) has nowhere to generate into. That
is not a failure: the form is kept and the warning says why no code came with it,
rather than leaving the absence unexplained.

# Errors

Returns `Err` only when the FORM itself could not be written: the folder could not
be created, the document could not be serialized, or the file could not be written.
A failure to generate the code is NOT an error here — the form exists by then, so it
is kept and reported in `CreatedForm::warning`.

# Examples

```rust
use crawrad_project::creation::create_form_file;
use crawrad_registry::Registry;

let folder = std::env::temp_dir().join("crawrad-doc-create-form/my_app/src");
std::fs::create_dir_all(&folder).expect("the folder");
std::fs::write(folder.parent().unwrap().join("Cargo.toml"), "[package]\nname = \"my_app\"\n")
    .expect("the manifest");

let created = create_form_file(&folder, &Registry::builtin()).expect("the form");
assert!(created.path.exists(), "the form itself is always written");
assert_eq!(created.path.extension().and_then(|e| e.to_str()), Some("crawform"));

let _ = std::fs::remove_dir_all(std::env::temp_dir().join("crawrad-doc-create-form"));
```
