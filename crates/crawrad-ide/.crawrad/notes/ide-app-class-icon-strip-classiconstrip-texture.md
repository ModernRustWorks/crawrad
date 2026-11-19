---
id: ide-app-class-icon-strip-classiconstrip-texture
kind: doc
target_file: src/ide_app/class_icon_strip.rs
line_hint: 19
author: 
created: 
title: "ClassIconStrip::texture"
tags: []
signature: "pub (crate) fn texture (& mut self , cx : & mut Cx , dpi : f64 , extra : Vec < (String , Vec < IconStep >) > ,) -> Texture"
---

Returns the class icon strip texture for this DPI.

The strip for this DPI, rebuilt only when the pixel side of a cell changes or
the set of extra icons differs from the cached strip.
