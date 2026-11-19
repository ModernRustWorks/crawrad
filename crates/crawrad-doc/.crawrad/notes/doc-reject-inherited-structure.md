---
id: doc-reject-inherited-structure
kind: doc
target_file: src/lib.rs
line_hint: 330
author:
created:
title: "Doc::reject_inherited_structure"
tags: []
signature: "fn reject_inherited_structure (& self , tx : & Transaction) -> Result < () , DocError >"
---

Refuses to change where an inherited component sits.

An inherited component's PLACE in the form belongs to the base: removing,
renaming, moving it to another parent or among its siblings is refused here,
because the derived file could not keep any of it. Its properties and events
are free, and stay in the derived file as overrides.
