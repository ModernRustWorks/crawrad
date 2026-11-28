---
id: workspace-refusal-unwritable-folder
kind: doc
target_file: src/workspace.rs
line_hint: 2074
author: 
created: 
title: "refusal_unwritable_folder"
tags: []
signature: "pub fn refusal_unwritable_folder (folder : & Path , app_dir : Option < & Path >) -> Option < Refusal >"
---

Says why a project cannot be created in this folder, **without creating one**.

`None` means a project can be written there. `Some` carries the sentence for the
dialog, in English, and it is the SAME sentence `make_project_parent` refuses
with, because that function calls this one first.

Three cases it answers without touching the disk — a path that is a file, a path
inside CrawRAD's own installation folder, and a path on a drive this computer
does not have — and one it can only answer by asking the folder itself: whether
it accepts a write. `is_dir()` says a folder EXISTS, never that it is writable,
and on Windows the metadata permission bits do not say it either; the ACL does.
So the folder is asked by creating one directory under a name nobody else uses
and removing it. A folder that does not exist yet is judged by the nearest
ancestor that does, because that is where the first `mkdir` lands.

# Examples

```rust
use crawrad_project::workspace::refusal_unwritable_folder;

// A folder the user owns takes a project.
let home = std::env::temp_dir();
assert!(refusal_unwritable_folder(&home, None).is_none());

// A file is not a folder, and the sentence says so.
let file = home.join(format!("crawrad-note-unwritable-{}", std::process::id()));
std::fs::write(&file, b"not a folder").unwrap();
let refusal = refusal_unwritable_folder(&file, None).expect("a file cannot hold a project");
assert!(refusal.phrase.contains("is a file, not a folder"));
let _ = std::fs::remove_file(&file);

// Inside the install folder, whatever the folder itself would allow.
let refusal = refusal_unwritable_folder(&home.join("app").join("x"), Some(&home.join("app")))
    .expect("the install folder is read-only on Windows");
assert!(refusal.phrase.contains("installation folder"));
```
