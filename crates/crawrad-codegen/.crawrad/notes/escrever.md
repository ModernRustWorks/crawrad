---
id: escrever
kind: doc
target_file: src/lib.rs
line_hint: 275
author: 
created: 
title: "escrever"
tags: []
signature: "pub fn escrever (raiz : & Path , arquivos : & [Arquivo]) -> Result < Relatorio , Erro >"
---

Escreve o que o gerador decidiu, respeitando o dono de cada arquivo.

# Examples

```rust,no_run
// Does not run during doc tests to avoid writing files to disk.
use std::path::Path;
use crawrad_codegen::{escrever, Arquivo};

let relatorio = escrever(Path::new("meu_projeto"), &[]).unwrap();
assert!(relatorio.criados.is_empty());
```

# Errors

Retorna erro se ocorrer falha de E/S ao criar pastas ou gravar arquivos no disco.
