---
id: targets-makepad-binding-bindings
kind: doc
target_file: src/targets/makepad/binding.rs
line_hint: 100
author: 
created: 
title: "BINDINGS"
tags: []
signature: "pub const BINDINGS : & [(& str , & str , Binding)] = & [(\"Button\" , \"on_click\" , Binding { accessor : \"button\" , part : \"\" , query : \"clicked\" , shape : Shape :: Plain , writeback : None , } ,) , (\"MenuItem\" , \"on_click\" , Binding { accessor : \"button\" , part : \"\" , query : \"clicked\" , shape : Shape :: Plain , writeback : None , } ,) , (\"Toggle\" , \"on_click\" , Binding { accessor : \"check_box\" , part : \"\" , query : \"changed\" , shape : Shape :: Value , writeback : Some (\"checked = _val\") , } ,) , (\"Toggle\" , \"on_change\" , Binding { accessor : \"check_box\" , part : \"\" , query : \"changed\" , shape : Shape :: Value , writeback : Some (\"checked = _val\") , } ,) , (\"BitBtn\" , \"on_click\" , Binding { accessor : \"button\" , part : \"\" , query : \"clicked\" , shape : Shape :: Plain , writeback : None , } ,) , (\"SpeedButton\" , \"on_click\" , Binding { accessor : \"button\" , part : \"\" , query : \"clicked\" , shape : Shape :: Plain , writeback : None , } ,) , (\"CheckBox\" , \"on_click\" , Binding { accessor : \"check_box\" , part : \"_box\" , query : \"changed\" , shape : Shape :: Value , writeback : Some (\"checked = next == \\\"cbChecked\\\"\") , } ,) , (\"RadioButton\" , \"on_click\" , Binding { accessor : \"radio_button\" , part : \"\" , query : \"RadioButtonAction::Clicked\" , shape : Shape :: ActionCast , writeback : Some (\"checked = true\") , } ,) , (\"ListBox\" , \"on_change\" , Binding { accessor : \"crawrad_list_box\" , part : \"\" , query : \"selected\" , shape : Shape :: Value , writeback : Some (\"selected = Some(_val)\") , } ,) , (\"ListBox\" , \"on_click\" , Binding { accessor : \"crawrad_list_box\" , part : \"\" , query : \"selected\" , shape : Shape :: Value , writeback : Some (\"selected = Some(_val)\") , } ,) , (\"TrackBar\" , \"on_change\" , Binding { accessor : \"slider\" , part : \"\" , query : \"slided\" , shape : Shape :: Value , writeback : Some (\"position = _val.round() as i64\") , } ,) , (\"ComboBox\" , \"on_change\" , Binding { accessor : \"crawrad_combo_box\" , part : \"\" , query : \"selected\" , shape : Shape :: Value , writeback : Some (\"selected = Some(_val)\") , } ,) , (\"Edit\" , \"on_change\" , TEXT_BOX_CHANGED) , (\"Edit\" , \"on_return\" , TEXT_BOX_RETURNED) , (\"MaskEdit\" , \"on_change\" , TEXT_BOX_CHANGED) , (\"MaskEdit\" , \"on_return\" , TEXT_BOX_RETURNED) , (\"Memo\" , \"on_change\" , Binding { accessor : \"text_input\" , part : \"\" , query : \"changed\" , shape : Shape :: Value , writeback : Some (\"lines = Strings::from_text(&_val)\") , } ,) ,]"
---

Table of bindings between catalog events and the framework.

**THE TABLE.** One `(classe, evento)` pair per line, and nothing more.

It is DATA. A third-party component declares its line in the manifest, and
does not write Rust text — which is the option that ADR 0036 rejected after
measuring that, of the eight precedents, **zero** ask the component author to
write target-language code.

# Examples

```rust
use crawrad_codegen::targets::makepad::binding::BINDINGS;

assert!(!BINDINGS.is_empty());
```
