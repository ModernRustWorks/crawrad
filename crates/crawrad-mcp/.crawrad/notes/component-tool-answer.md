---
id: component-tool-answer
kind: doc
target_file: src/component_tool.rs
line_hint: 18
author: 
created: 
title: "answer"
tags: []
signature: "pub fn answer (args : & Value , install : impl FnOnce (& Path) -> Result < Installed , String >) -> Answer"
---

Answers `project.install_component`: checks the path, installs, reports.

The install itself is the caller's, so this crate never touches the disk.

# Examples

```
use crawrad_mcp::component_tool::answer;

let args = serde_json::json!({ "path": "relative/gauge" });
let (_, status) = answer(&args, |_| Err("never called".into()));
assert_eq!(status, "error");
```
