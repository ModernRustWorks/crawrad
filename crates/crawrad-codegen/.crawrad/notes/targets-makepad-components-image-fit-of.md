---
id: targets-makepad-components-image-fit-of
kind: doc
target_file: src/targets/makepad/components/image.rs
line_hint: 124
author: 
created: 
title: "fit_of"
tags: []
signature: "pub fn fit_of (node : & Node) -> (& 'static str , & 'static str , & 'static str)"
---

Determines Makepad fit, image size, and alignment from classic RAD properties.

The makepad fit, the image's size and the control's alignment for the
classic RAD `Stretch` / `Proportional` / `Center` (#438):

| Stretch | Proportional | the picture |
|---|---|---|
| no | no | its own size (`Size`) |
| yes | no | stretched to the control (`Stretch`) |
| either | yes | as big as fits, aspect kept (`Smallest`) |

`Center` centres it in the control; without it, it sits top-left.

# Examples

```rust
use crawrad_codegen::targets::makepad::components::image::fit_of;
use crawrad_ir::Node;

let node = Node::new("img", "Image");
let (fit, size, align) = fit_of(&node);
assert_eq!(fit, "Size");
assert!(size.contains("Fit"));
assert!(align.contains("0.0"));
```
