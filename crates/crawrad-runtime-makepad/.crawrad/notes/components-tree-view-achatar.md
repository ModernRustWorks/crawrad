---
id: components-tree-view-achatar
kind: doc
target_file: src/components/tree_view.rs
line_hint: 67
author: 
created: 
title: "achatar"
tags: []
signature: "fn achatar (itens : & [crawrad_ir :: Item] , symbols : & [String] , nivel : usize , fora : & mut Vec < ContentLine >)"
---

Flattens the item tree into a list with the indentation level of each item.

The depth is limited because the document comes from a file: a tree
that is too deep cannot become a stack overflow in the middle of a frame.
