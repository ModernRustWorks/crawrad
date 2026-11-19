---
id: canvas-brush-layers
kind: doc
target_file: src/canvas/brush.rs
line_hint: 206
author: 
created: 
title: "LAYERS"
tags: []
signature: "pub const LAYERS : [Layer ; 16] = [Layer :: Background , Layer :: GridDot , Layer :: Box , Layer :: Picture , Layer :: Text , Layer :: Stroke , Layer :: Arrow , Layer :: Check , Layer :: Badge , Layer :: BadgeText , Layer :: Dashed , Layer :: Chrome , Layer :: ChromeArrow , Layer :: Handle , Layer :: Ruler , Layer :: RulerText ,]"
---

The canvas stack, **from bottom to top**.

Touching this list changes the screen. What each neighbourhood guarantees is
in the `a_pilha_de_camadas_protege_o_que_ja_quebrou` test, and every one of
its assertions names the defect it prevents.

# Examples

```rust
use crawrad_runtime_makepad::canvas::LAYERS;

assert_eq!(LAYERS.len(), 16);
```
