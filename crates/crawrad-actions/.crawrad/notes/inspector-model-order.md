---
id: inspector-model-order
kind: doc
target_file: src/inspector_model.rs
line_hint: 178
author: 
created: 
title: "ORDER"
tags: []
signature: "const ORDER : [Category ; 6] = [Category :: Identity , Category :: Appearance , Category :: Data , Category :: Layout , Category :: Behavior , Category :: Advanced ,]"
---

The order of the categories on screen.

The order of the categories on screen. It is the egui shell's
(`crates/crawrad-ide/src/inspector.rs:171-177`), and it is deliberately not
alphabetical: `Identity` opens because it is the name of the component, and
`Advanced` closes because it is what gets touched least.
