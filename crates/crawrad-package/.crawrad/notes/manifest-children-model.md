---
id: manifest-children-model
kind: doc
target_file: src/manifest.rs
line_hint: 1263
author: 
created: 
title: "Children::model"
tags: []
signature: "fn model (self , item_class : Option < String > , class : & str ,) -> Result < ExternalChildren , ManifestError >"
---

The model, with the item class that came from the neighboring field.

The two refusals are symmetric and exist for the same reason: a collection
without an item class accepts anything inside, and an item class
without a collection is a declaration nobody reads — and a declaration nobody
reads is one that ages incorrectly without warning.
