---
id: ide-app-host-api-ideapp-land-edit
kind: doc
target_file: src/ide_app/host_api.rs
line_hint: 155
author: 
created: 
title: "IdeApp::land_edit"
tags: []
signature: "pub fn land_edit (& mut self , tool : & str , path : std :: path :: PathBuf , (before , after) : (String , String) , gate : bool ,) -> Result < Option < crawrad_agent :: diff_review :: DiffReview > , String >"
---

Applies an agent's buffer edit or submits it for review.

An agent's edit lands: proposed for review when `gate` and the mode
asks for one — and, waiting for approval, not applied (`Ok(Some)`) —
else applied to the buffer with its undo.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let mut app = IdeApp::default();
let res = app.land_edit("patch", std::path::PathBuf::from("a.rs"), ("".into(), "".into()), false);
assert!(res.is_ok());
```

# Errors

Returns an error if proposing the diff review fails.
