---
id: targets-makepad-look-look
kind: doc
target_file: src/targets/makepad/look.rs
line_hint: 13
author: 
created: 
title: "LOOK"
tags: []
signature: "const LOOK : & [(& str , & str , Option < bool > , & str)] = & [(\"Button\" , \"\" , Some (false) , \"Pressable\") , (\"Label\" , \"{f}_text\" , Some (true) , \"Plain\") , (\"Edit\" , \"\" , Some (false) , \"Pressable\") , (\"MaskEdit\" , \"\" , Some (false) , \"Pressable\") , (\"DateTimePicker\" , \"\" , Some (false) , \"Pressable\") , (\"Memo\" , \"\" , Some (false) , \"Pressable\") , (\"CheckBox\" , \"{f}_box,{f}_lead:Plain\" , None , \"Toggle\") , (\"RadioButton\" , \"\" , None , \"Toggle\") , (\"SpinEdit\" , \"{f}_edit\" , None , \"Pressable\") , (\"Toggle\" , \"\" , None , \"Toggle\") , (\"ColorBox\" , \"{f}_list\" , None , \"Pressable\") , (\"LinkLabel\" , \"\" , None , \"Pressable\") , (\"Badge\" , \"label\" , None , \"Plain\") , (\"Card\" , \"header.title\" , Some (false) , \"Plain\") , (\"Drawer\" , \"header.title\" , Some (false) , \"Plain\") , (\"CategoryPanel\" , \"header\" , None , \"Pressable\") , (\"Panel\" , \"label\" , Some (false) , \"Plain\") , (\"GroupBox\" , \"title\" , Some (false) , \"Plain\") , (\"StatusBar\" , \"*status\" , Some (false) , \"Plain\") , (\"PageControl\" , \"*tabs\" , Some (false) , \"Pressable\") , (\"ScrollBox\" , \"-\" , Some (false) , \"\") ,]"
---

Styling specifications mapping component classes to text and background paths.

`(class, text, background, states)`: `text` is the path under the
control's field where the text is drawn (`""` the widget itself, `"{f}"`
stands for the field's name; several texts are `a,b`, and `b:Plain` gives
one its own states); `background` is `Some(show_bg)` when the
widget has one; `states` names the state colours its text mixes
(`crawrad_runtime_makepad::look::TextStates`).
