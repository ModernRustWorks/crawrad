---
id: contract-common
kind: doc
target_file: src/contract.rs
line_hint: 24
author: 
created: 
title: "COMMON"
tags: []
signature: "pub const COMMON : & [& str] = & [\"name\" , \"text\" , \"rect\" , \"enabled\" , \"visible\" , \"font\" , \"tab_order\" , \"tab_stop\" , \"anchors\" , \"align\" , \"hint\" , \"tag\" , \"action\" , \"images\" , \"image_index\" , \"shortcut\" , \"category\" , \"image_width\" , \"image_height\" , \"images_data\" , \"items\" , \"interval\" , \"item_index\" , \"checked\" , \"password_char\" , \"readonly\" , \"active_page\" , \"border_width\" , \"margins\" , \"align_with_margins\" , \"scroll_bars\" , \"kind\" , \"min\" , \"max\" , \"position\" , \"small_change\" , \"large_change\" , \"auto_scroll\" , \"horz_scroll_bar\" , \"vert_scroll_bar\" ,]"
---

The common core: properties that **every** runtime honors.

# Examples

```rust
use crawrad_ir::contract::COMMON;

assert!(COMMON.contains(&"name"));
assert!(COMMON.contains(&"visible"));
```

Growing this list is an architectural decision, not a matter of convenience: only
what EVERY target can implement with the same
semantics enters. When in doubt, the property is born namespaced and moves up later.
