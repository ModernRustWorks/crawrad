---
id: ide-app-constants-novo-campos
kind: doc
target_file: src/ide_app/constants.rs
line_hint: 53
author: 
created: 
title: "NOVO_CAMPOS"
tags: []
signature: "pub (super) const NOVO_CAMPOS : [(ZonePath , & str) ; 3] = [(| | id ! (novo_nome) , \"Name\") , (| | id ! (novo_simbolo) , \"Symbol\") , (| | id ! (novo_categoria) , \"Category\") ,]"
---

The three fields for the new icon, with the placeholder text for each one.

The placeholder text ACTS as the label here, and it is a space choice:
three labels on top of three fields would consume an entire line in a window
360 in height. In a field that starts empty and disappears upon typing, it says what
needs to be said when it is needed.
