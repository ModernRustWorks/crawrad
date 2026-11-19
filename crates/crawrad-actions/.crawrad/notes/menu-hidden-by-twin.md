---
id: menu-hidden-by-twin
kind: doc
target_file: src/menu.rs
line_hint: 188
author: 
created: 
title: "hidden_by_twin"
tags: []
signature: "fn hidden_by_twin (a : & crate :: Action , group : & [crate :: Action] , ctx : & Context) -> bool"
---

Checks if an action is hidden by its view twin.

Two actions with one label, one per view (Copy of the components, Copy of
the text, #257): the menu lists the one of the view in front. An action of
any view is never hidden, and one without a twin neither.
