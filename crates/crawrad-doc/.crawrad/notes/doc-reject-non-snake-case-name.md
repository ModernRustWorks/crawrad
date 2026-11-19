---
id: doc-reject-non-snake-case-name
kind: doc
target_file: src/lib.rs
line_hint: 293
author: 
created: 
title: "Doc::reject_non_snake_case_name"
tags: []
signature: "fn reject_non_snake_case_name (& self , tx : & Transaction) -> Result < () , DocError >"
---

Rejects non-snake_case names in rename commands.

**The entryway for `snake_case`**: a new `Name` is a lowercase Rust
identifier, or it does not get in.

# Why it lives in `apply`, and not in `Command::Rename`

Because `Command::apply` is the two-way path. Undoing
restores the previous name via a `Command::Rename` identical to this one, and
in a project saved before 16/09/2026 the previous name is precisely
`Button1`. Enforcing it in there, Ctrl+Z on a legitimate rename — the
user fixing `Button1` to `btn_ok` — broke with a refusal,
**and the step fell out of the stack**: the IDE stopped undoing.

Measured: `renomear_a_acao_nao_desliga_quem_aponta_para_ela` was the one that
caught it. The rule belongs to the GESTURE, not the history — undoing does not introduce
any name, it restores one the document already had.

# Why it is not in `Doc::validate`

Because a document saved with a name outside the convention must continue
OPENING — the sample app is one of them. Rejecting on load would lock the
user out of their own project, and the IDE is precisely what they
would use to rename.

# Why it is not in the `component.rename` action

Because the action is not the only entryway: double-clicking, the tree, and the
MCP agent build transactions. Here, no one bypasses it — and the rule that
`renomear_para` in `crawrad-ide` enforces remains valid, because what it
forbids is the action having its OWN validation.
