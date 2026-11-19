---
id: host-shellextensionhost-contributed-templates
kind: doc
target_file: src/host.rs
line_hint: 355
author: 
created: 
title: "ShellExtensionHost::contributed_templates"
tags: []
signature: "pub fn contributed_templates (& self) -> Vec < & ProjectTemplateContribution >"
---

List all project templates contributed across registered extension manifests.

# Examples

```rust
use crawrad_extension_host::ShellExtensionHost;

let host = ShellExtensionHost::default();
assert!(host.contributed_templates().is_empty());
```
