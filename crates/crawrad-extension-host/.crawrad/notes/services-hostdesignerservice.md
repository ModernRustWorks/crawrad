---
id: services-hostdesignerservice
kind: doc
target_file: src/services.rs
line_hint: 285
author: 
created: 
title: "HostDesignerService"
tags: []
signature: "pub struct HostDesignerService"
---

Host implementation of `DesignerService` managing visual design inspection.

Host implementation of `DesignerService` managing visual design inspection and property updates.

# Examples

```rust
use crawrad_extension_host::HostDesignerService;

let svc = HostDesignerService::new();
assert!(svc.components.is_empty());
```
