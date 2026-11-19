---
id: ide-app-constants-items-botoes
kind: doc
target_file: src/ide_app/constants.rs
line_hint: 87
author: 
created: 
title: "ITEMS_BOTOES"
tags: []
signature: "pub (super) const ITEMS_BOTOES : [(ZonePath , & str , ItemsCommand) ; 5] = [(| | id ! (btn_new_item) , \"New Item\" , ItemsCommand :: NewItem) , (| | id ! (btn_new_subitem) , \"New SubItem\" , ItemsCommand :: NewSubItem ,) , (| | id ! (btn_delete) , \"Delete\" , ItemsCommand :: Delete) , (| | id ! (btn_move_up) , \"Move Up\" , ItemsCommand :: MoveUp) , (| | id ! (btn_move_down) , \"Move Down\" , ItemsCommand :: MoveDown) ,]"
---

The five commands in the Items Editor's middle column, in egui's order.

One LIST, like `ALIGN_ZONES` above: the drawing order and the words cannot
diverge. Each button carries the command `items_editor::run` executes.
