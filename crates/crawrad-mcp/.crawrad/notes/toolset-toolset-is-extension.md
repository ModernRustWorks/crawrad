---
id: toolset-toolset-is-extension
kind: doc
target_file: src/toolset.rs
line_hint: 124
author: 
created: 
title: "Toolset::is_extension"
tags: []
signature: "fn is_extension (& self , name : & str) -> bool"
---

Checks whether a tool name originated from an extension.

Whether `name` came from an extension: the IDE listed it so, or it has
the `<id>.tool.<name>` form, or no IDE group knows its prefix (a call
made before any `tools/list`, like `todos.list`).
