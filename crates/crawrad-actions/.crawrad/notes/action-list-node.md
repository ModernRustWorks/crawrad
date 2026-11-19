---
id: action-list-node
kind: doc
target_file: src/lib.rs
line_hint: 1373
author: 
created: 
title: "action_list_node"
tags: []
signature: "fn action_list_node (c : & Context) -> Option < NodeId >"
---

Finds the action list node associated with the current selection.

The action list a new action would enter: the selected one, or the one
containing the selected action — clicking on an action and requesting another is natural.
