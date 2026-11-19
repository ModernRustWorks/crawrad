---
id: adapter-dataset-mirror
kind: doc
target_file: src/adapter/dataset.rs
line_hint: 70
author: 
created: 
title: "Mirror"
tags: []
signature: "struct Mirror"
---

What the store's callbacks touch.

# Why the store is held WEAKLY here

The store keeps the callbacks, and the callbacks keep the mirror. A strong
reference back to the store would close the circle and nothing would ever be
dropped -- a form opened and closed a hundred times would keep a hundred datasets.
