---
id: components-page-control-desenho-draw
kind: doc
target_file: src/components/page_control.rs
line_hint: 52
author: 
created: 
title: "Desenho::draw"
tags: []
signature: "fn draw (& self , p : & mut Brush , c : & MakepadCtl , r : Rect , scene : & Scene)"
---

The tab strip and the BODY, which are two things.

The body begins BELOW the strip, and not at the top of the component — that is what the
RXP does (`crawrad-runtime-egui/src/lib.rs:2819-2821`), and that is what makes the strip
announce itself even in a newly dropped `PageControl`, which does not yet have
any page. While the body covered the entire component, the owner saw
an *"empty rectangle with the `+` in the corner"* and did not recognize the component.
