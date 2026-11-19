---
id: host-shellextensionhost-register-extension
kind: doc
target_file: src/host.rs
line_hint: 202
author: 
created: 
title: "ShellExtensionHost::register_extension"
tags: []
signature: "pub fn register_extension (& mut self , mut ext : Box < dyn IdeExtension > ,) -> Result < () , ExtensionError >"
---

Register and activate a compiled Rust plugin extension.

# Examples

```rust
use crawrad_extension_api::traits::{ExtensionContext, IdeExtension};
use crawrad_extension_api::error::ExtensionError;
use crawrad_extension_host::ShellExtensionHost;

struct DummyExt;
impl IdeExtension for DummyExt {
    fn id(&self) -> &str { "dummy" }
    fn activate(&mut self, _: &mut dyn ExtensionContext) -> Result<(), ExtensionError> { Ok(()) }
}

let mut host = ShellExtensionHost::default();
assert!(host.register_extension(Box::new(DummyExt)).is_ok());
```

# Errors

Returns [`ExtensionError::Duplicate`](crawrad_extension_api::error::ExtensionError::Duplicate) if an extension with this ID exists, or if activation fails.
