---
id: components-page-control-tab-sheet
kind: doc
target_file: src/components/page_control.rs
line_hint: 101
author: 
created: 
title: "tab_sheet"
tags: []
signature: "pub fn tab_sheet () -> ComponentDesc"
---

An individual page inside a `PageControl`.

`TabSheet`: a page. The classic `TTabSheet`.

**Out of the palette**, on purpose — an empty palette string hides the component. In the
classic RAD IDE `TTabSheet` is not there either: pages are born via *New Page* on
`TPageControl`, because a page outside a page control is nothing at all.

The default size is only what `instantiate` needs: the real rectangle comes
from normalization, which runs alongside `Align`.

# Examples

```rust
use crawrad_registry::tab_sheet;

let desc = tab_sheet();
assert_eq!(desc.class, "TabSheet");
```
