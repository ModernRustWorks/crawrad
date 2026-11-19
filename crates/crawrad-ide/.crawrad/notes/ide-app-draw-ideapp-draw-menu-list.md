---
id: ide-app-draw-ideapp-draw-menu-list
kind: doc
target_file: src/ide_app/draw.rs
line_hint: 823
author: 
created: 
title: "IdeApp::draw_menu_list"
tags: []
signature: "fn draw_menu_list (& self , cx : & mut Cx2d , lista : & mut PortalList , rows : & [menu :: Row] , highlight : Option < usize > , arranjo : & std :: collections :: HashMap < LiveId , DockItem > ,)"
---

Draws one menu panel's list — the bar menu's and its submenu's (#558).

## Inside `draw_menu_list`: `let tick = match linha.panel_state {`

The VISIBILITY CHECKMARK comes from the dock (`painel_a_vista`,
the same check that lights up the rail): `crawrad-actions` has no
window.
