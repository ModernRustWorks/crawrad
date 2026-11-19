---
id: ide-app-chrome-ideapp-show-menu
kind: doc
target_file: src/ide_app/chrome.rs
line_hint: 317
author: 
created: 
title: "IdeApp::show_menu"
tags: []
signature: "pub (super) fn show_menu (& mut self , cx : & mut Cx)"
---

Opens, closes, and POSITIONS the menu panel.

The position comes from the rectangle of the button that opened it — the same
calculation as any menu bar: aligned to the left of the title, just below the
strip. It goes via `margin` because under `Flow::Overlay` margin is
pure offset (`makepad-draw-1.0.0/src/turtle.rs:279-281`), and
because `margin` is `#[live]` — `apply_over` with `live!{}` is the
official way to change a property at runtime (`widget.rs:733`).

The HEIGHT is also written here, and it is not decorative: the DSL does not
know how many actions the group has, and a `PortalList` needs a height to know
how many items to draw. `Fit` with an inner virtual list does not resolve.

#### At `let janela = self.ui.area().rect(cx);`

And it cannot overflow past the bottom or right edge: a menu
opened 8 px from the footer draws three rows outside the window, and nothing
warns about it. The WINDOW rectangle is measured, as mandated by the rule that
already governs the Y of the top menu — screen numbers are asked of the screen.

#### At `let (x, y) = match aberto {`

The X comes from the BUTTON and the Y from the bottom edge of the BAR, both
measured on screen.

Neither of the two is constant, and Y in particular once was and cost
dearly: written as the strip height (34), the menu appeared OVER
the menu bar. The bar does not start at zero — MEASURED, it is at
`y = 27`, because Makepad's `Window` reserves the top strip before
the body. Screen numbers are asked of the screen.

#### At `let comandos = self.menu_rows();`

The HEIGHT is the sum of the rows, and a SEPARATOR does not measure the same as
a command. Counting everything as a full row left dead space at the foot
of the menu — and the panel is sized to fit exactly, otherwise the
`PortalList` shows a scrollbar.

## Inside `show_menu`: `if let Some((_, row, _)) = &self.open_submenu {`

The SUBMENU opens beside its row (#558): its left edge at the menu's
right edge, its top at the row's top, kept in the window the same way.
