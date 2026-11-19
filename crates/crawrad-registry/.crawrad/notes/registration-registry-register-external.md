---
id: registration-registry-register-external
kind: doc
target_file: src/registration.rs
line_hint: 101
author: 
created: 
title: "Registry::register_external"
tags: []
signature: "pub fn register_external (& mut self , c : ExternalComponent) -> Result < () , RegistrationError >"
---

Registers a third-party external component into the registry.

**Takes a component declared from outside — or says why it will not.**

See the module header for why this door refuses and
`Registry::register` does not.

# Examples

```rust
use crawrad_registry::Registry;
use crawrad_registry::button;
use crawrad_registry::external::ExternalComponent;

let mut reg = Registry::default();
let ext = ExternalComponent::from(&button());
assert!(reg.register_external(ext).is_ok());
```

# Errors

Returns [`RegistrationError`] if the component has an invalid class name or prefix,
is already registered, has duplicate properties or events, or targets an unknown runtime.
