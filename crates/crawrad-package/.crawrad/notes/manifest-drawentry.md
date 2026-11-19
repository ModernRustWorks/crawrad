---
id: manifest-drawentry
kind: doc
target_file: src/manifest.rs
line_hint: 210
author: 
created: 
title: "DrawEntry"
tags: []
signature: "pub struct DrawEntry"
---

A drawing verb in the manifest file format.

**A DRAWING VERB, in the file format.**

The grammar is the same as the property `kind` and `children`: **bare name, and
the payload in neighboring fields.** Whoever writes the manifest already knows it from
two other sections, and learning a third form for the same idea would be
cost without benefit.

```toml
[[component.draw]]
verb = "box"
fill = "surface"
border = "border"
radius = 3.0

[[component.draw]]
verb = "text"
prop = "text"        # what the user typed in the Object Inspector
color = "text"
centered = true
```

**There is no literal color field, and that is on purpose** — see
[`crawrad_ir::drawing::Ink`]. A component published on crates.io with
`#ff0000` inside would be immune to the theme, and would be outside our reach to
fix.

# Examples

```rust
use crawrad_package::manifest::DrawEntry;

let draw = DrawEntry {
    verb: "box".into(),
    fill: Some("surface".into()),
    border: None,
    color: None,
    radius: None,
    thickness: None,
    text: None,
    prop: None,
    centered: false,
    area: None,
};
assert_eq!(draw.verb, "box");
```
