---
id: process-outcome-report
kind: doc
target_file: src/process.rs
line_hint: 168
author: 
created: 
title: "Outcome::report"
tags: []
signature: "pub fn report (& self) -> Option < String >"
---

One line fit for a status bar, or `None` if genuinely silent.

One line fit for a status bar, and **`None` only when the child was
genuinely silent**.

This is the whole of issue #67 in one function. The old code was
`stderr.lines().find(|l| l.starts_with("error"))`, and everything else
went in the bin — so a cargo that failed while saying something other
than `error` (a broken manifest, a missing linker, a dependency that
will not resolve) left the shell printing `build failed, with nothing to
show` while the reason sat right there in the pipe.

The order of preference, and why:

1. the line cargo itself marked `error` — it is the one written to be
   read;
2. otherwise the **last** non-empty line of `stderr`. Last and not
   first, because cargo's first lines are progress (`Compiling`,
   `Updating`) and the verdict is at the bottom;
3. otherwise the last non-empty line of `stdout`, for the children that
   do not use `stderr` at all.

One line and not the whole text because the caller is a status bar. The
whole text is still in [`Outcome::stderr`] for a panel to show; that is
exactly what discarding it made impossible.

# Examples

```rust
use crawrad_os::process::Outcome;

let outcome = Outcome {
    stderr: "error: failed".to_string(),
    ..Default::default()
};
assert_eq!(outcome.report().as_deref(), Some("error: failed"));
```
