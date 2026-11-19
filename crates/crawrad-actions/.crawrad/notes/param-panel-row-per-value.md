---
id: param-panel-row-per-value
kind: doc
target_file: src/lib.rs
line_hint: 4282
author: 
created: 
title: "PARAM_PANEL_ROW_PER_VALUE"
tags: []
signature: "const PARAM_PANEL_ROW_PER_VALUE : Param = Param"
---

The same parameter, for the one action a menu can draw whole.

Written out instead of updating [`PARAM_PANEL`] because functional record
update is not allowed in a `const`. What must not drift between the two is
the NAME and the DOMAIN, and both are charged by
`o_parametro_do_toggle_e_o_mesmo_dos_outros`.
