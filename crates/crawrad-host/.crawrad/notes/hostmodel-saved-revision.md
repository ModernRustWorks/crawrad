---
id: hostmodel-saved-revision
kind: doc
target_file: src/lib.rs
line_hint: 325
author: 
created: 
title: "HostModel::saved_revision"
tags: []
signature: "pub fn saved_revision (& self) -> u64"
---

The revision the form in front was last saved at.

It decides that form's own `saved` / `modified` badge.

The shell held one of these beside its one document. With a tab each, two forms
have independent saved states, and a single field would have told the person
their saved form was modified the moment the other one changed.

# Examples

```rust
use crawrad_host::HostModel;

let host = HostModel::new();
assert_eq!(host.saved_revision(), 0, "nothing open, nothing saved");
```
