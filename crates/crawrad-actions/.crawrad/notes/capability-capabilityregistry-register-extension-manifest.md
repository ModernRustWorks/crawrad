---
id: capability-capabilityregistry-register-extension-manifest
kind: doc
target_file: src/capability.rs
line_hint: 565
author: 
created: 
title: "CapabilityRegistry::register_extension_manifest"
tags: []
signature: "pub fn register_extension_manifest (& mut self , manifest : & ExtensionManifest ,) -> Result < usize , CapabilityError >"
---

Registers actions and tools declared in an [`ExtensionManifest`].

Dynamically registers the actions and tools declared in an [`ExtensionManifest`].

# Errors

Returns [`CapabilityError`] if any contributed action or tool fails ID validation or collides.

# Panics

Panics if an extension tool schema is not a valid JSON object when checking emptiness.

# Examples

```rust
use crawrad_actions::capability::CapabilityRegistry;
use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};

let mut reg = CapabilityRegistry::new();
let manifest = ExtensionManifest {
    id: "ext.sample".to_string(),
    name: "Sample".to_string(),
    version: "0.1.0".to_string(),
    publisher: "Dev".to_string(),
    description: String::new(),
    contributions: ExtensionContributions::default(),
    entrypoint: None,
    activation_events: Vec::new(),
};
assert_eq!(reg.register_extension_manifest(&manifest).unwrap(), 0);
```
