---
id: targets-makepad-form-text-box-enter-call
kind: doc
target_file: src/targets/makepad/form.rs
line_hint: 521
author: 
created: 
title: "text_box_enter_call"
tags: []
signature: "fn text_box_enter_call (emitted : & [& Node]) -> Result < String , Erro >"
---

Handles Enter key behavior in text boxes following classic RAD IDE rules.

Enter in the form's text boxes as in a classic RAD IDE (issue #295): a Memo's Enter
breaks the line, an Edit never gets one. The key is rewritten before any
widget sees it — see `crawrad_runtime_makepad::keys`. Empty for a form with
neither.
