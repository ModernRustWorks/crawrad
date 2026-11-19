---
id: targets-egui-components-codegen-eguicomponentcodegen-envolve-filhos
kind: doc
target_file: src/targets/egui/components/codegen.rs
line_hint: 20
author: 
created: 
title: "EguiComponentCodegen::envolve_filhos"
tags: []
signature: "fn envolve_filhos (& self) -> bool"
---

**This component wraps its children**, instead of being drawn beside
them.

# What the default answer costs, and why it is still the default

Fifteen of the sixteen classes answer `false`, and for most of them it
is right: a `Button` has no children, and a `Panel` only paints a
background under components that the designer positioned in FORM
coordinates — drawing them beside it lands them in the same place.

The one that cannot live with it is the `ScrollBox`. Its children have
to be inside its `egui::ScrollArea`, or the container never overflows,
never scrolls and never shows a bar (issue #157). It was a
`ScrollArea` with an empty body and the children drawn as siblings —
a container that does not contain.
