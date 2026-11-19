---
id: inspector-model-editors-anchors-on
kind: doc
target_file: src/inspector_model/editors/anchors.rs
line_hint: 54
author: 
created: 
title: "on"
tags: []
signature: "fn on (map : & BTreeMap < String , Value > , key : & str) -> bool"
---

Returns whether the given side is anchored.

Is this side anchored?

A side the map does not mention is **off**, and that is not a guess: the
document only writes what was set, and `crawrad-designer` reads a missing key
the same way.
