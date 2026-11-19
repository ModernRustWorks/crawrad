---
id: targets-egui-components-codegen-eguicomponentcodegen-form-strip
kind: doc
target_file: src/targets/egui/components/codegen.rs
line_hint: 35
author: 
created: 
title: "EguiComponentCodegen::form_strip"
tags: []
signature: "fn form_strip (& self) -> bool"
---

Paints a strip along the form instead of a control in its rectangle.

**This component paints a strip along the form** (the `MainMenu` bar,
#455) instead of a control in its own rectangle: its body goes out
after the controls, with no pointer events and no tab stop (#538).
