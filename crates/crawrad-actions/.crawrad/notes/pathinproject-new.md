---
id: pathinproject-new
kind: doc
target_file: src/lib.rs
line_hint: 273
author: 
created: 
title: "PathInProject::new"
tags: []
signature: "pub fn new (raw : & str) -> Option < Self >"
---

Validates and normalizes a path relative to the generated project.

Returns `None` for everything that **may** escape the root. The check is on
the structure of the path, not on the form it was written — that is
the difference that matters, because the first version of this validation
enumerated forms (`is_absolute()` plus `..`) and let through three
whole families:

| input | `is_absolute()` | what `join` did |
|---|---|---|
| `\Windows\System32\...` | **false** | `F:\Windows\System32\...` |
| `/Windows/System32/...` | **false** | `F:/Windows/System32/...` |
| `C:algo.txt` | **false** | `C:algo.txt` — the root **discarded** |

On Windows, `is_absolute()` requires prefix **and** root; a path with only
one of the two escapes the check and still makes `PathBuf::join`
**discard** the root the shell passed. The subsequent `exists()`
defended nothing: `C:\Windows\System32\drivers\etc\hosts` exists.

The rule here is a single one, not a list of forbidden forms: traverse the
components and refuse any that **re-anchors** the path — `Prefix`
(`C:`, `\\server\share`, `\\?\`) and `RootDir` (`\x`, `/x`) — counting the
depth so that `..` does not ascend above the start. A path that only
has normal components cannot escape any root.

`.` and a `..` that does not ascend too high are accepted and **vanish** in
normalization: `src/../src/main.rs` is a legitimate path, and returning the
clean form is what makes the shell compare apples to apples later.

# Examples

```rust
use crawrad_actions::PathInProject;

assert!(PathInProject::new("src/lib.rs").is_some());
assert!(PathInProject::new("../escape.rs").is_none());
```
