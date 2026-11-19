---
id: dock-decode
kind: doc
target_file: src/dock.rs
line_hint: 54
author: 
created: 
title: "decode"
tags: []
signature: "pub fn decode (text : & str , is_valid_tab : impl Fn (LiveId) -> bool ,) -> Option < HashMap < LiveId , DockItem > >"
---

The arrangement a text carries, or `None` when it cannot be trusted whole.

Returns `None` when:
1. the version marker is missing or does not match [`FORMAT_VERSION`];
2. the body does not parse;
3. there is no `root`;
4. a `Tab` fails the `is_valid_tab` predicate.

# Examples

```rust
use crawrad_runtime_makepad::dock::decode;

assert!(decode("invalid", |_| true).is_none());
```
