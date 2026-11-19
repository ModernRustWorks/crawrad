---
id: inspector-model-editors-kinds
kind: doc
target_file: src/inspector_model/editors.rs
line_hint: 138
author: 
created: 
title: "KINDS"
tags: []
signature: "pub const KINDS : & [Kind] = & [collection :: KIND , lines :: KIND , mute :: KIND , anchors :: KIND , edges :: KIND , align :: KIND , choice :: KIND , when :: KIND , text :: KIND , check :: KIND , integer :: KIND , decimal :: KIND , color :: KIND , rect :: KIND , reference :: KIND , date_time :: KIND , file_path :: KIND , cursor :: KIND , modal_result :: KIND , shortcut :: KIND , font :: KIND , picture :: KIND ,]"
---

Every kind of editor, in the order the makers are asked.

The order is load-bearing — see the module doc.

# Examples

```rust
use crawrad_actions::inspector_model::editors::KINDS;

assert!(!KINDS.is_empty());
```
