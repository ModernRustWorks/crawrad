---
id: inspector-model-editor-propeditor-window-items
kind: doc
target_file: src/inspector_model/editor.rs
line_hint: 171
author: 
created: 
title: "PropEditor::window_items"
tags: []
signature: "fn window_items (& self) -> Option < bool >"
---

Returns whether this row opens the Items Editor.

Does this row open the **Items Editor** instead of the text pad?

`Some(hierarchical)` for a collection, `None` for everything else —
and the `bool` is the same one the catalogue declares
(`PropDesc::colecao_em_arvore`), carried here rather than asked again
by the shell. Asking again is exactly how a `ListBox` once opened the
TREE editor: the question was answered a second time, by a literal
class name.

# Why the editor answers it, and not a `match` in the shell

Because it is the same rule as everything else on this trait: the
property type decides how it is edited. A shell that asked *"is this
key `items`?"* would be the shell knowing about collections again, and
a twelfth property kind would cost a line there instead of a file
here.
