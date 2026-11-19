---
id: file-dialog-pick-picture
kind: doc
target_file: src/file_dialog.rs
line_hint: 174
author: 
created: 
title: "pick_picture"
tags: []
signature: "pub fn pick_picture (file : Option < & str >) -> Option < Result < crawrad_ir :: Picture , String > >"
---

Obtains a picture from an explicit file path or an open dialog.

The picture for a `Picture` property (#553): the `file` an agent named, or
the one picked in the dialog filtered to images. `None` when the user gave
up, which is not a refusal and has nothing to say.

# Examples

```rust
use crawrad_runtime::file_dialog::pick_picture;

let res = pick_picture(Some("nonexistent-file.png"));
assert!(res.is_some());
assert!(res.unwrap().is_err());
```
