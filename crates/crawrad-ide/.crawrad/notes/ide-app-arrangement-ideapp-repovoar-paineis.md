---
id: ide-app-arrangement-ideapp-repovoar-paineis
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 291
author: 
created: 
title: "IdeApp::repovoar_paineis"
tags: []
signature: "pub (super) fn repovoar_paineis (& mut self , cx : & mut Cx)"
---

Restores the MODEL to every panel that the dock has just (re)created.

# The regression that this function exists to avoid repeating

`Dock::load_state` calls `self.items.clear()` and recreates everything
(`dock.rs:989-993`). Each panel becomes a **new** widget, and whatever lived
inside the old one dies with it. For almost all of them this costs nothing — they
rewrite themselves from the document every frame. `DesignCanvas` is the opposite
case: the document lives INSIDE it, in `MakepadRuntime`.

Measured on 2026-09-16, by pointer, with the IDE open:

```text
reabrir_no_canvas -> DesignCanvas em 0x185be1e5650   (ctls = 15)
draw_walk         -> DesignCanvas em 0x185be3328c0   (ctls =  0)
```

On screen: the form paper and grid drew — they come from
`form_rect` and `grid`, which have defaults — and **none of the 19 components**.
Nothing crashed, nothing logged, and the adapter suite stayed at 89 greens: the
drawing logic was correct the whole time, drawing an empty
document.

The `Package Manager` had already paid the same toll by hand, right below, in
the course of another front. This function is where that stops being a
memory burden on whoever writes the next `load_state`.

## Inside `repovoar_paineis`: `self.write_panel_chrome(cx);`

AND THE CHROME INSIDE THE PANELS, for the same reason as the canvas and by the
same measurement.

It used to be written in `montar_cromo`, BEFORE the `load_arrangement` that
closes that method — and `load_state` recreates the widget of each dock
item, erasing everything. On screen: the `Properties | Events` strip of the
Object Inspector came out EMPTY (the bar existed, both buttons
responded to clicks, and neither had text), and the `X` of
each dockable panel came out without a glyph. Seen in a screenshot, not deduced.

Nothing complained: `set_text` on a `WidgetRef` that `Dock` already
discarded is not an error, and the suite remained green — the same family of
defect described above for `DesignCanvas`.

## Inside `repovoar_paineis`: `self.reabrir_no_canvas(cx);`

The CANVAS last and always: it is the only panel whose content is
not rewritten every frame from the document.
