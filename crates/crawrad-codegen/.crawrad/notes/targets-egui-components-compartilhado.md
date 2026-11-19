---
id: targets-egui-components-compartilhado
kind: doc
target_file: src/targets/egui/components/mod.rs
line_hint: 82
author: 
created: 
title: "Compartilhado"
tags: []
signature: "pub struct Compartilhado"
---

Shared sections required for egui rendering.

What a component's body needs to come along from `desenho_comum.rs`
and `shared_geometry.rs`.

# Why it belongs to the RESULT, and is not a query to the descriptor

Because whoever emitted knows what was emitted. A verb whose property
the class does not declare is **skipped** — the descriptor would say there is dashed drawing, and
the body would have no call at all. Emitting the function there would be `dead_code` in
the user's project, and they cannot fix it: `.designer.rs` is rewritten.

It is the same grammar as `precisa_timer`/`precisa_item` that were already here.

# Examples

```rust
use crawrad_codegen::targets::egui::components::Compartilhado;

let c = Compartilhado::default();
assert!(!c.tracejado);
```
