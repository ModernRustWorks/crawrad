---
id: targets-shared-standard-colors-const
kind: doc
target_file: src/targets/shared.rs
line_hint: 440
author: 
created: 
title: "standard_colors_const"
tags: []
signature: "pub fn standard_colors_const () -> String"
---

Generates standard color array constant for `ColorBox` components.

The standard colours as a constant of the generated project (#500:
a `ColorBox`'s list), `CRAWRAD_STANDARD_COLORS`: `crawrad_ir::Color::STANDARD`,
one list for both targets and for every ColorBox of a form.

# Examples

```rust
use crawrad_codegen::targets::shared::standard_colors_const;

let code = standard_colors_const();
assert!(code.contains("CRAWRAD_STANDARD_COLORS"));
```
