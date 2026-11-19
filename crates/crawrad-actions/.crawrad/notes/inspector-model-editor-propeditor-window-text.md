---
id: inspector-model-editor-propeditor-window-text
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 156
author: 
created: 
title: "PropEditor::window_text"
tags: []
signature: "fn window_text (& self) -> Option < String >"
---

The text the window behind this row opens with.

`None` for the nine editors that have no window, and that `None` is
what the shell checks before it opens anything — so a row whose button
was pressed by a mis-aimed action cannot bring up an empty pad.

A **String** and not the value itself, because all three windows of this
panel are the same window: a text pad. `Multiline` opens with its own
text, `Collection` opens with one item per line, and the align picker
has no pad at all. Whoever converts is the editor, right below.
