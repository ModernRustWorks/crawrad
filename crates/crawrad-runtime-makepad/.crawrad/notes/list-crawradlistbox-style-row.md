---
id: list-crawradlistbox-style-row
kind: doc
target_file: src/list.rs
line_hint: 345
author: 
created: 
title: "CrawradListBox::style_row"
tags: []
signature: "fn style_row (& self , cx : & mut Cx , i : usize , row : & WidgetRef , plain : bool)"
---

Configures row padding, font size, and unselected row colors.

Gives `row` the list's look. Only an unselected row takes the text
and background colours: the selected one keeps its highlight.
