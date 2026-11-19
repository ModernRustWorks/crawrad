---
id: explorer-folds-rebuild-if-needed
kind: doc
target_file: src/explorer.rs
line_hint: 216
author: 
created: 
title: "Folds::rebuild_if_needed"
tags: []
signature: "fn rebuild_if_needed (& mut self , document : Option < & Path > , active : Option < & Path >)"
---

The guard itself: a different document starts the state over.

Every method above goes through here, and none of them touches a field
before calling it. That is what makes the rebuild impossible to forget
— there is no second call to remember.
