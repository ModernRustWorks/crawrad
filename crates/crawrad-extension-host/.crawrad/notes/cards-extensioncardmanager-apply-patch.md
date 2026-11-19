---
id: cards-extensioncardmanager-apply-patch
kind: doc
target_file: src/cards.rs
line_hint: 163
author: 
created: 
title: "ExtensionCardManager::apply_patch"
tags: []
signature: "pub fn apply_patch (node : & mut UiNode , patch : UiElementPatch)"
---

Apply property patches to an individual UI node.

# Examples

```rust
use crawrad_extension_api::ui::{UiElementPatch, UiNode};
use crawrad_extension_host::ExtensionCardManager;

let mut node = UiNode::label("lbl1", "Hi");
ExtensionCardManager::apply_patch(&mut node, UiElementPatch { visible: Some(false), ..Default::default() });
assert!(!node.visible);
```
