---
id: ide-app-arrangement-ideapp-warm-up-lists
kind: doc
target_file: src/ide_app/arrangement.rs
line_hint: 334
author: 
created: 
title: "IdeApp::WARM_UP_LISTS"
tags: []
signature: "pub (crate) const WARM_UP_LISTS : & 'static [(& 'static [LiveId] , & 'static [& 'static str])] = & [(id ! (toolbox_palette . toolbox_list) , & [\"Secao\" , \"SecaoPrimeira\" , \"Item\" , \"ItemOff\" , \"Vazia\"] ,) , (id ! (structure_tree . structure_list) , & [\"Linha\" , \"Raiz\"]) ,]"
---

Every template of every `PortalList` in the Components panel, by list.

Written as text because the gate `warm_up_covers_every_template` reads
the DSL of `paineis.rs` and compares: a template added there and
forgotten here would bring the first-click delay back without a word,
and a template renamed there would make `item` build nothing.
