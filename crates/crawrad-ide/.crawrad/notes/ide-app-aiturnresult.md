---
id: ide-app-aiturnresult
kind: doc
target_file: src/ide_app.rs
line_hint: 356
author: 
created: 
title: "AiTurnResult"
tags: []
signature: "pub type AiTurnResult = crawrad_agent :: acp_live :: TurnResult"
---

Turn result of the AI assistant.

Resultado de um turno do assistente de IA: resposta de texto, registros de ferramentas invocadas e aviso opcional de fallback.

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let _res: Option<AiTurnResult> = None;
assert!(_res.is_none());
```
