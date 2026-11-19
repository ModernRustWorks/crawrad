---
id: services-hostprojectservice
kind: doc
target_file: src/services.rs
line_hint: 165
author: 
created: 
title: "HostProjectService"
tags: []
signature: "pub struct HostProjectService"
---

Host implementation of `ProjectService` managing workspace files and forms.

Host implementation of `ProjectService` managing workspace files, forms, and build triggers.

# Examples

```rust
use crawrad_extension_host::HostProjectService;

let svc = HostProjectService::new(None);
assert!(svc.root_path.is_none());
```
