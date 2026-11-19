---
id: components-declarado-area-de
kind: doc
target_file: src/components/declarado.rs
line_hint: 101
author: 
created: 
title: "area_de"
tags: []
signature: "fn area_de (a : & Area , c : & MakepadCtl , r : Rect) -> Rect"
---

The rectangle that this verb paints.

**The rectangle that this verb paints.**

The fraction is clamped to `0..=1` before turning into size: an out-of-range value
does not make the component paint outside its own box, which is the rule that
native components already follow (`clips_to_own_box`, issue #129).
