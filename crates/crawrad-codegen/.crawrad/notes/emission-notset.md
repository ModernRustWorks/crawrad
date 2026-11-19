---
id: emission-notset
kind: doc
target_file: src/emission.rs
line_hint: 88
author: 
created: 
title: "NotSet"
tags: []
signature: "pub enum NotSet"
---

Reasons why a catalog property does not become a `Set`.

**Why a catalog property DOES NOT become a `Set`.**

Three reasons, and none of them is "this class is special" — which is what separates a
rule from a list of exceptions.

# Examples

```rust
use crawrad_codegen::emission::NotSet;

let not_set = NotSet::Identity;
assert_eq!(not_set, NotSet::Identity);
```
