---
id: package-manager-parse-dependencies-json
kind: doc
target_file: src/package_manager.rs
line_hint: 242
author: 
created: 
title: "parse_dependencies_json"
tags: []
signature: "pub fn parse_dependencies_json (json : & str , pacote_alvo : Option < & str >) -> Vec < InstalledPackage >"
---

Extracts installed dependencies from `cargo metadata --no-deps` JSON.

# Examples

```rust
use crawrad_project::package_manager::parse_dependencies_json;

let deps = parse_dependencies_json("{}", None);
assert!(deps.is_empty());
```
