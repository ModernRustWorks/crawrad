---
id: ide-app-dialogs-imagelist-ideapp-draw-imagelist
kind: doc
target_file: src/ide_app/dialogs/imagelist.rs
line_hint: 43
author: 
created: 
title: "IdeApp::draw_imagelist"
tags: []
signature: "pub (in super :: super) fn draw_imagelist (& mut self , cx : & mut Cx)"
---

Draws the ImageList Editor on screen.

## Inside `draw_imagelist`: `for (caminho, espera) in NOVO_CAMPOS {`

THE ADD TO GALLERY ROW. The three fields convey what they are through their
placeholder text — a label on top of each would waste three lines in a
360 window.

## Inside `draw_imagelist`: `let combo = dlg.drop_down(id!(iml_size));`

THE SIZE COMBO: the options are those of the model, and the text is the current
pair. A `16x16` written in the DSL would be the second list of sizes.
