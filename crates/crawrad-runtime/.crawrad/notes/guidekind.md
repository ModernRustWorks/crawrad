---
id: guidekind
kind: doc
target_file: src/lib.rs
line_hint: 95
author: 
created: 
title: "GuideKind"
tags: []
signature: "pub enum GuideKind"
---

What the guide is aligning — the classic RAD IDE uses different colors for each.

# Examples

```rust
use crawrad_runtime::GuideKind;

let kind = GuideKind::Component;
assert_eq!(kind, GuideKind::Component);
```
