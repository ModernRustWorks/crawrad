---
id: ide-app-ui-zoom-ideapp-dpi-da-area-de-trabalho
kind: doc
target_file: src/ide_app/ui_zoom.rs
line_hint: 72
author: 
created: 
title: "IdeApp::dpi_da_area_de_trabalho"
tags: []
signature: "fn dpi_da_area_de_trabalho (& mut self , cx : & Cx) -> f64"
---

Retrieves and caches the native desktop DPI factor.

**The DPI the desktop asked for, captured ONCE and kept.**

# Why it is not `CxWindow::os_dpi_factor`

Because that field is never filled on this platform. Measured: the only
assignment to it in the whole framework is
`os/apple/macos/macos.rs:310` — `os/windows/windows.rs` and
`os/linux/x11/linux_x11.rs` mention it **zero** times. On Windows it is
`None` forever, and everything that reads it silently answers the
default.

That is not a detail: it is what broke the pointer. See
[`IdeApp::com_ponteiro_na_escala`].

# Why it cannot be read fresh either

`window_geom.dpi_factor` is where the override LANDS
(`os/windows/windows.rs:115-117`), so after the first step it is our own
answer read back — and multiplying by it again compounds on every press.
The classic drift of a scale that reads its own output.

So it is captured on the first step, when `zoom_da_ide` is still `1.0`
and no override has been written, and never again.
