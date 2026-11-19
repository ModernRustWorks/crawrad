---
id: targets-makepad-components-codegen-makepadcomponentcodegen-default-cursor
kind: doc
target_file: src/targets/makepad/components/codegen.rs
line_hint: 16
author: 
created: 
title: "MakepadComponentCodegen::default_cursor"
tags: []
signature: "fn default_cursor (& self) -> & 'static str"
---

The mouse pointer Cursor::Default shows over this component.

The pointer `Cursor::Default` shows over the component (#667): the
one the classic control has of its own — the text beam of an edit, the
hand Makepad gives a button. Makepad sets a pointer only as the hover
comes in (a text input sets none), so the generated app shows this one
on every move, or the last control's pointer would stay.
