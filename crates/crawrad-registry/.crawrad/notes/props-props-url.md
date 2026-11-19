---
id: props-props-url
kind: doc
target_file: src/props.rs
line_hint: 1071
author: 
created: 
title: "Props::url"
tags: []
signature: "pub fn url (default : & str) -> PropDesc"
---

The web or mail address a `LinkLabel` opens (#500).

The address a `LinkLabel` opens (#500): web or mail only — the
generated app refuses any other scheme (`crawrad_runtime::shell`).

# Examples

```rust
use crawrad_registry::Props;

let p = Props::url("https://example.com");
assert_eq!(p.key, "url");
```
