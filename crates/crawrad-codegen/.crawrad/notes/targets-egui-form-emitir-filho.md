---
id: targets-egui-form-emitir-filho
kind: doc
target_file: src/targets/egui/form.rs
line_hint: 678
author: 
created: 
title: "emitir_filho"
tags: []
signature: "fn emitir_filho (doc : & Document , reg : & crawrad_registry :: Registry , n : & Node , radios : & [(crawrad_ir :: NodeId , String)] , control_events : Option < & super :: control_events :: ControlEvents > , tab_order : Option < & super :: tab_order :: TabOrder > ,) -> Result < ResultadoComponente , Erro >"
---

A child inside a wrapping container: its `let r` and the body.

The position is the one **relative to the parent** (`n.rect()`), and not
`rect_no_formulario`: inside the container block, `origin` is already the corner
of its content. It is the same trade-off that the Makepad target made in reverse — there
`abs_pos` is absolute and asks for the form's; here `anchor` starts from an
origin, and the origin changed.
