---
id: process-outcome-merged
kind: doc
target_file: src/process.rs
line_hint: 155
author: 
created: 
title: "Outcome::merged"
tags: []
signature: "pub fn merged (& self) -> String"
---

Both streams, `stderr` first.

`stderr` first because that is where cargo narrates: `Creating`,
`Adding` and `error` all come out there, and depending on which of the
two carries an informative line would be depending on a detail that has
already changed once.

# Examples

```rust
use crawrad_os::process::Outcome;

let outcome = Outcome {
    stdout: "out".to_string(),
    stderr: "err".to_string(),
    ..Default::default()
};
assert_eq!(outcome.merged(), "errout");
```
