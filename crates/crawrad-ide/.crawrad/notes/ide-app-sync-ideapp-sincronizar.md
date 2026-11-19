---
id: ide-app-sync-ideapp-sincronizar
kind: doc
target_file: src/ide_app/sync.rs
line_hint: 14
author: 
created: 
title: "IdeApp::sincronizar"
tags: []
signature: "pub (crate) fn sincronizar (& mut self , cx : & mut Cx)"
---

Synchronizes derived document state across title, status bar, and panels.

Writes to the interface everything DERIVED from the document: the two status
bar labels, the title, the `saved` badge, and the active node of the AI
panel.

Comparing with what is already on screen is not a matter of taste:
a `Label`'s `set_text` requests a redraw when the text changes
(`label.rs:247-249`), and rewriting every frame would make the window never
sleep.

#### At `if sel != self.selecao_na_arvore {`

**THE STRUCTURE TREE REDRAWS WHEN THE SELECTION CHANGES.**

#### The defect this comparison closes, and it looked like something else

The owner, with the checkmark one line below the one he clicked: *"I clicked the
one above, it selected the one below... it's a side effect of zoom"*. It was not
zoom, and the proof was on his own screen: the status bar said
`edt_nome_ selected` — the RIGHT node, the one he had clicked. What
was stale was the DRAWING of the tree, stuck on the previous checkmark.

The cause has been documented here since the Package Manager, in the doc of
[`IdeApp::redraw_panel_list`]: `self.ui.redraw(cx)` **does not
reach** a `PortalList` that lives inside a dock item.
`select_node` only called it, and the list retained the screen of the
previous frame while the underlying state had already changed.

#### Why HERE, and not in `select_node`

Because the selection changes through many doors — canvas click,
`Del`, `Ctrl+Z`, `Ctrl+A`, the effect of a catalog action — and
a line in each would be one to forget. `sincronizar`
runs after all of them, and already has the selection in hand two lines above.

#### At `self.colher_do_editor(cx);`

WHAT WAS TYPED IS READ BACK, and it is here because this is the only
point that runs every frame with the editor in front. `CodeView`
notifies no one when text changes — see
[`IdeApp::colher_do_editor`].

#### At `self.draw_pad(cx);`

THE TWO INSPECTOR WINDOWS, for the same reason as the header: they
depend on a field that no status bar gesture touches, and
placing them after the `return` down there would make them appear only in the frame
where the bar changed.

#### At `let pode_add = self.collection_selection().is_some();`

The Structure's `+` lights up only when the selection is a collection.

**Disabled, not hidden.** A classic RAD IDE disables it, and a button that disappears
takes away the clue that it exists — it is the rule that this codebase has already
applied to the status bar and the menu.

#### At `let sel: Vec<NodeId> = self`

THE INSPECTOR HEADER comes before the shortcut below, and it is on
purpose: it depends on the SELECTION, and there are gestures that change selection without
changing a single letter on the status bar. If placed after the `return`, the
panel would show the previous component's name.

## Inside `sincronizar`: `let title = match self.host.active_file().and_then(|p| p.file_name(...`

The title carries the file NAME even when the bar is occupied
by a message — that way the identity of what is open never vanishes
from the screen.

## Inside `sincronizar`: `self.escrever_rodape_do_editor(cx);`

THE EDITOR FOOTER MOVES ON ITS OWN: the cursor moves without the center
changing view, and `show_center` only runs when it changes.

## Inside `sincronizar`: `self.draw_items(cx);`

The Items Editor through the same door and for the same reason as the pad: it
depends on a field that no status bar gesture touches.
