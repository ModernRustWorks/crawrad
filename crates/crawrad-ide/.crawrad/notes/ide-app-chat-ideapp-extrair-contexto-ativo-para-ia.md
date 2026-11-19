---
id: ide-app-chat-ideapp-extrair-contexto-ativo-para-ia
kind: doc
target_file: src/ide_app/chat.rs
line_hint: 514
author: 
created: 
title: "IdeApp::extrair_contexto_ativo_para_ia"
tags: []
signature: "pub fn extrair_contexto_ativo_para_ia (& self) -> crawrad_agent :: ProjectContext"
---

Extracts the live project context for the AI.

Extracts the live project context (from the editor's in-memory buffer or from the designer).

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let app = IdeApp::default();
assert!(matches!(app.extrair_contexto_ativo_para_ia(), crawrad_agent::ProjectContext::None));
```
