---
id: targets-makepad-components-edit-font-points
kind: doc
target_file: src/targets/makepad/components/edit.rs
line_hint: 223
author: 
created: 
title: "font_points"
tags: []
signature: "fn font_points (node : & Node) -> f32"
---

Returns the Edit control font size in points.

The Edit's font size in points: its own when it does not follow the form's,
otherwise the theme's paragraph size (`THEME_FONT_SIZE_P`, 10 pt).
