---
id: components-tab-sheet-classe
kind: doc
target_file: src/components/tab_sheet.rs
line_hint: 10
author: 
created: 
title: "CLASSE"
tags: []
signature: "pub const CLASSE : & str = \"TabSheet\""
---

The class of this component in the `crawrad-registry` catalog.

`PageControl` reads this constant to find its pages: whoever knows the
class name is its own file, and never the parent.

# Examples

```rust
use crawrad_runtime_makepad::components::tab_sheet::CLASSE;

assert_eq!(CLASSE, "TabSheet");
```
