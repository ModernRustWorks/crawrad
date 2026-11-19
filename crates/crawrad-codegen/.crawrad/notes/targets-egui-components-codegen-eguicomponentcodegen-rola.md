---
id: targets-egui-components-codegen-eguicomponentcodegen-rola
kind: doc
target_file: src/targets/egui/components/codegen.rs
line_hint: 25
author: 
created: 
title: "EguiComponentCodegen::rola"
tags: []
signature: "fn rola (& self) -> bool"
---

**This component opens an `egui::ScrollArea`** in the generated code.

The generated application sets the scroll-bar theme ONCE per form, and
it has to know whether any component asks for one. The question is
asked of each class, and not of a list written somewhere central: a
list is the second place to maintain, and the class added tomorrow is
the one that gets forgotten there.
