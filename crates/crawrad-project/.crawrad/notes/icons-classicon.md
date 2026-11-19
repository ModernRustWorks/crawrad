---
id: icons-classicon
kind: doc
target_file: src/icons.rs
line_hint: 367
author: 
created: 
title: "ClassIcon"
tags: []
signature: "pub enum ClassIcon"
---

The icon of a **catalogue component** — the Toolbox row and the Structure row.

# Why this one is drawn and not another glyph

Codicons covers the chrome and does **not** cover the catalogue. Reading its
`cmap` and its `post` table (639 glyphs, `U+EA60`..`U+ECE7`) there is no
`Edit`, no `Memo`, no `PageControl`, no `ComboBox` and no `TabSheet`;
`edit` is a pencil and `primitive-square` is a filled square. Patching five
holes with a second family rebuilds the very problem that put Codicons here
— several families in one column. It is the decision ADR 0026 already took
for the egui shell, and the drawings below are that same set, ported.

# The variant says WHICH, the shader says HOW

[`Self::kind`] is the instance value the one shader of `CrawradClassIcon`
switches on. One shader and N instances is the rule of the house: a
`DrawQuad` only exists when it comes from the DSL
(`makepad-platform-1.0.0/src/draw_vars.rs:609-617`).

# Examples

```rust
use crawrad_project::icons::ClassIcon;

assert_eq!(ClassIcon::Button.class(), "Button");
```
