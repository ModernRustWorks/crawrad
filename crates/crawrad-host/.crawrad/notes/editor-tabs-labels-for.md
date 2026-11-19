---
id: editor-tabs-labels-for
kind: doc
target_file: src/editor_tabs.rs
line_hint: 177
author: 
created: 
title: "labels_for"
tags: []
signature: "pub fn labels_for (paths : & [PathBuf]) -> Vec < Label >"
---

Generates distinct labels for a set of open paths.

**The labels of a set of open paths, each one just distinct enough.**

# The rule

A file name that is unique among the open tabs shows the name and nothing
else. A name that repeats gets the SHORTEST run of parent folders that
tells its group apart — one folder if one is enough, two if it is not.

The `…/` is what says the hint is a slice and not the whole path. Showing
the whole path was the first thing tried and it is unreadable at tab width:
`crates/crawrad-ide/src/ide_app` on a tab that exists to be glanced at.

# The case that has no answer, and it is handled

Two identical paths cannot be told apart by any number of folders.
[`EditorTabs::open`] never creates them, but this function is public and
pure, so it stops walking up at the longest path instead of looping.

# Examples

```rust
use std::path::PathBuf;
use crawrad_host::editor_tabs::labels_for;

let labels = labels_for(&[PathBuf::from("src/main.rs")]);
assert_eq!(labels.len(), 1);
assert_eq!(labels[0].name, "main.rs");
```
