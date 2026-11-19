---
id: vizinhas
kind: doc
target_file: src/lib.rs
line_hint: 673
author: 
created: 
title: "vizinhas"
tags: []
signature: "fn vizinhas (raiz : & Path , destino : & Path) -> Result < Vec < String > , Erro >"
---

Returns the content of the other Rust files in the project to check calls.

The other `.rs` files of the project, so that pruning knows if anyone else calls the
handler before removing it. They serve both ends: the pre-pass
([`vinculos_vencidos`]) and pruning ([`rxp::podar`]) must answer the same
thing, otherwise the binding leaves and the `fn` stays.

**The `.designer.rs` files are left out, and it is not an optimization.** They are rewritten
from the document on every generation, and what they call is **exactly what
the document points to** — that is, the binding that the pre-pass is
about to remove. Counting that call as "someone uses it" would be the document
vetoing itself: no bound handler would ever leave, which is the old contract
returning through the back door. And there is the worse disk case: during
`escrever` the designer may still be the one from the last generation, calling a handler
that is no longer even in the document — a dead call blocking all removal,
forever, without anything explaining why.

Nothing is lost in this: the designer is nobody's code, and its call
disappears on its own when the document loses the binding, because it is born from the
document.

A neighbor read error **is not an error here**: a file that cannot
be read is a file whose content is unknown, and not knowing the
content is a reason to be conservative — which `podar` already is by finding
nothing. Refusing the entire generation because some `.rs` in `src/` is
locked by another process would be trading a cleanup for a standstill.
