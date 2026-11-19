---
id: targets-makepad-form-theme-link
kind: doc
target_file: src/targets/makepad/form.rs
line_hint: 511
author: 
created: 
title: "theme_link"
tags: []
signature: "fn theme_link (root : & Node) -> & 'static str"
---

The form's `Theme` on Makepad 1.0 (#76).

`makepad_widgets` links `theme_desktop_dark` by default (at
`makepad-widgets-1.0.0/src/lib.rs:174`). When the form specifies `thLight`,
we link `crawrad_theme_light` — makepad's light theme with its fonts named where
they live (`crawrad-runtime-makepad/src/theme_light.rs`; the published one fails
to load its Chinese and emoji fonts).
Makepad 1.0 has no system-theme query: `thSystem` is dark on the desktop
and light in the browser (`target_arch = "wasm32"`), where a web page is
white (#1043); `thDark` stays dark everywhere.
