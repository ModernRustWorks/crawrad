---
id: inspector-model-editor-propeditor-window-value
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 161
author: 
created: 
title: "PropEditor::window_value"
tags: []
signature: "fn window_value (& self , _text : & str) -> Outcome"
---

What the text of that window becomes when it is confirmed.

The inverse of [`Self::window_text`], and it lives on the same object
so that the round trip can never be split between two files: the editor
that decides one item per line is the editor that reads one item per
line back.
