---
id: project-tool-delete-file
kind: doc
target_file: src/project_tool.rs
line_hint: 70
author: 
created: 
title: "delete_file"
tags: []
signature: "pub fn delete_file (root : Option < & Path > , args : & Value) -> (Answer , Option < PathBuf >)"
---

Deletes a file under the project root and returns its path.

`project.delete_file`: removes `path`; the deleted file's path too, for
the shell to close its buffer.

# Examples

```rust
use crawrad_mcp::project_tool::delete_file;
use serde_json::json;

let (ans, path) = delete_file(None, &json!({}));
assert_eq!(ans.1, "error");
assert!(path.is_none());
```
