---
id: inspector-model-editor-propeditor-children
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 151
author: 
created: 
title: "PropEditor::children"
tags: []
signature: "fn children (& self) -> Vec < (String , Editor) >"
---

The rows that appear under this one, each with the name it shows.

**Empty when the property is folded away**, and that is why there is no
second method asking whether it is open: the editor was built knowing,
and a caller that had to ask twice would be a caller that can ask the
two questions in the wrong order.
