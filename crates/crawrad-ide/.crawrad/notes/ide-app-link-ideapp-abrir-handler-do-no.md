---
id: ide-app-link-ideapp-abrir-handler-do-no
kind: doc
target_file: src/ide_app/link.rs
line_hint: 62
author: 
created: 
title: "IdeApp::abrir_handler_do_no"
tags: []
signature: "pub (super) fn abrir_handler_do_no (& mut self , cx : & mut Cx , no : & NodeId)"
---

**The link through the form**: the handler a component double-click opens.

THE RULE DOES NOT LIVE HERE: the answer comes from
[`crawrad_codegen::elo::elo_do_duplo_clique`], the SAME function that the egui
shell queries — it moved houses on the day it gained its second
caller, and `codigo.rs` over there today only re-exports it. *An already bound event wins; otherwise, the
first one declared by the catalogue* — and no "default event by
class" table is written in this shell, which would be a second truth about the
same component.

`Label`, `Panel` and `GroupBox` declare no event: double-clicking
them says so on the bar and does nothing else, as in a classic RAD IDE.

#### At `if classe == CLASSE_IMAGE_LIST {`

IMAGELIST COMES BEFORE THE COLLECTION CHECK, and the order is the
fix: it HAS a collection property (`images_data`), and
because of that fell into the Items Editor — which opened as `ImageList Items
Editor`, with `New SubItem` on a list of icons. The owner saw it on
screen on 09/17/2026: *"ImageList is not this dlg neither here nor in China"*.

Whatever has a CUSTOM editor wins over the generic editor, and that is what
a classic RAD IDE does: the component's default verb is its own.

#### At `if let Some(classe) = self`

**THE COMPONENT'S DEFAULT VERB WINS OVER THE DEFAULT EVENT**, and that is what
a classic RAD IDE does: double clicking a `TActionList` opens the list editor, and
not the `.pas`. Whatever has its own editor exits here.

The COLLECTION question belongs to the catalogue — *does this component have a
collection property?* —, and therefore a new component that declares
one opens the Items Editor on double click without a single line changing here.

The other two are by CLASS, and it is the same list that the egui shell
has (`main.rs`, the `PointerDblClick` arm). They are by class
because *having a component editor* is not yet a catalogue fact; the
day it is, both shells will change together.
