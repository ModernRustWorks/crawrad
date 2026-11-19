---
id: form-retired-keys
kind: doc
target_file: src/lib.rs
line_hint: 57
author: 
created: 
title: "FORM_RETIRED_KEYS"
tags: []
signature: "pub const FORM_RETIRED_KEYS : & [& str] = & [\"enabled\" , \"visible\" , \"anchors\" , \"align\" , \"margins\" , \"align_with_margins\" , \"tab_order\" , \"tab_stop\" ,]"
---

Keys no longer declared by the root form node.

Keys the form (the root node) no longer declares (#667): layout and Tab
order belong to the controls inside a window, not to the window itself.
The registry leaves them out of the form's Inspector and
[`Document::from_json`] drops them from files saved before.

# Examples

```rust
use crawrad_ir::FORM_RETIRED_KEYS;

assert!(FORM_RETIRED_KEYS.contains(&"enabled"));
assert!(FORM_RETIRED_KEYS.contains(&"visible"));
```
