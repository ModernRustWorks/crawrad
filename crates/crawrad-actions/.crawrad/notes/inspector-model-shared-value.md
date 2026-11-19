---
id: inspector-model-shared-value
kind: doc
target_file: src/inspector_model.rs
line_hint: 529
author: 
created: 
title: "shared_value"
tags: []
signature: "fn shared_value (doc : & Document , selection : & [NodeId] , p : & PropDesc) -> Option < Value >"
---

The value every selected component shares, or `None` when they differ.

A property ABSENT from the node is worth the catalogue default — that is
what the drawing uses, and it is what the Inspector has to show. egui
compares only what is written (`inspector.rs:551`), and that is why two
freshly created components showed up as differing when one of them had
never been touched.
