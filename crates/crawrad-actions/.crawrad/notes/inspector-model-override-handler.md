---
id: inspector-model-override-handler
kind: doc
target_file: src/inspector_model.rs
line_hint: 1
author:
created:
title: "override_handler"
tags: []
signature: "pub fn override_handler (base : & str) -> String"
---

The name this form's handler gets when it overrides the base's `base`.

A name of its own, and not the base's: the form's file keeps only what
differs from the base, and an override under the same name would leave it
on the next save. The owner chose the suffix (07/11/2026).

# Examples

```
use crawrad_actions::inspector_model::override_handler;

assert_eq!(override_handler("btn_save_click"), "btn_save_click_override");
```
