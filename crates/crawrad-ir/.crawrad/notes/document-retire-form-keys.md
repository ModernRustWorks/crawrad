---
id: document-retire-form-keys
kind: doc
target_file: src/lib.rs
line_hint: 280
author: 
created: 
title: "Document::retire_form_keys"
tags: []
signature: "fn retire_form_keys (& mut self)"
---

Drops retired keys from the root form node.

Drops the keys the form no longer declares ([`FORM_RETIRED_KEYS`])
from the root, so a form saved before #667 still opens and still
generates: the generators refuse a property the class does not
declare (#660). It runs on every load, not by format version: a key
that is gone stays gone however the file was written.
