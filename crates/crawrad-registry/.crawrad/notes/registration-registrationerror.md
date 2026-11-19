---
id: registration-registrationerror
kind: doc
target_file: src/registration.rs
line_hint: 34
author: 
created: 
title: "RegistrationError"
tags: []
signature: "pub enum RegistrationError"
---

Why a component from outside was refused.

Every variant carries what it needs to name the offender, because the
sentence ends up in front of a person installing something.

# Examples

```rust
use crawrad_registry::registration::RegistrationError;

let err = RegistrationError::BadClass("123bad".into());
assert!(err.to_string().contains("123bad"));
```
