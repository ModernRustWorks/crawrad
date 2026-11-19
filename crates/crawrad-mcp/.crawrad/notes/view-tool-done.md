---
id: view-tool-done
kind: doc
target_file: src/view_tool.rs
line_hint: 62
author: 
created: 
title: "done"
tags: []
signature: "pub fn done (drive : & Drive , doc : Option < & crawrad_doc :: Doc >) -> Answer"
---

The answer once the shell has run `drive`.

# Examples

```rust
use crawrad_mcp::view_tool::{done, Drive};

let (result, status) = done(&Drive::RunAction("edit.undo".to_string()), None);
assert_eq!(status, "applied");
```
