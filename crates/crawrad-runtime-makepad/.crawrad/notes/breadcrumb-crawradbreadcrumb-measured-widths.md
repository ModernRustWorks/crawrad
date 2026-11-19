---
id: breadcrumb-crawradbreadcrumb-measured-widths
kind: doc
target_file: src/breadcrumb.rs
line_hint: 437
author: 
created: 
title: "CrawradBreadcrumb::measured_widths"
tags: []
signature: "fn measured_widths (& self , cx : & mut Cx) -> (Vec < f32 > , f32)"
---

Measures the pixel width of each segment and the separator.

The REAL width of each segment and of the separator, measured by the
fonts in use: a proportional font ("m" wide, "i" narrow) needs it for
even gaps and for clicks that hit what is drawn.
