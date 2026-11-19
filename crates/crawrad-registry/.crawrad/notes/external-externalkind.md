---
id: external-externalkind
kind: doc
target_file: src/external.rs
line_hint: 98
author: 
created: 
title: "ExternalKind"
tags: []
signature: "pub enum ExternalKind"
---

The mirror of [`PropKind`] carrying owned data.

The mirror of [`PropKind`], and it mirrors it **including the payload** even
though `ExternalProp` also carries `options` and `ref_class` separately.

The duplication is the catalogue's, not ours: `PropDesc::with_options` writes the
options into both the field and the kind, and `PropDesc::references` does the
same with the class. Carrying only the discriminant here and rebuilding the
payload from the neighbouring field would be a second place deciding they
agree — and it would be right for the twenty components of today and wrong
for the first one that sets a kind without its field.

# Examples

```rust
use crawrad_registry::external::ExternalKind;

let kind = ExternalKind::Text;
assert_eq!(kind, ExternalKind::Text);
```
