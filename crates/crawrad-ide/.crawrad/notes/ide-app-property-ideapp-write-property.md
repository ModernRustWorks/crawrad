---
id: ide-app-property-ideapp-write-property
kind: doc
target_file: src/ide_app/property.rs
line_hint: 14
author: 
created: 
title: "IdeApp::write_property"
tags: []
signature: "pub (super) fn write_property (& mut self , cx : & mut Cx , alvos : Vec < NodeId > , key : & str , valor : Value ,)"
---

Writes a property — and it is **always** through the action catalog.

No transaction originates here, and this is not pedantry:
`component.set_property` already delivers the entire batch in a single
undo step (`crates/crawrad-actions/src/lib.rs:1396`), and it is the same
path that an RTA agent has (ADR 0005). The Inspector does not get a
shortcut that the protocol does not have.

`name` is the declared exception, and it comes from ADR 0008: changing the name is
`component.rename`, which checks uniqueness and rejects an invalid name with its
own message. A `set_property` with the `name` key would silently bypass
both guards. It is the same detour that the egui shell makes
(`crates/crawrad-ide/src/inspector.rs:531-537`).

#### At `self.redraw_panel_list(cx, Panel::Inspector, id!(inspector_panel.inspe`

#554: a summary row reads the value it was just given (`Rect`
folded shows `24, 96, 140 × 32`), and only a list redraw reaches a
`PortalList` inside a dock item; without it the typed `Width` moved
the canvas while the summary still said 96.
