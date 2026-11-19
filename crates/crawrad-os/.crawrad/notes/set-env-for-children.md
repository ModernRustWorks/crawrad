---
id: set-env-for-children
kind: doc
target_file: src/lib.rs
line_hint: 317
author: 
created: 
title: "set_env_for_children"
tags: []
signature: "pub fn set_env_for_children (key : & str , value : & str)"
---

Sets an environment variable inherited by every child process.

Sets a variable of this process's environment, which every child it starts
inherits.

Edition 2024 marks `std::env::set_var` unsafe: on POSIX, another thread
reading the environment at that moment is undefined behaviour. The IDE calls
this once, at start-up, on the UI thread, before the agent and terminal
threads that read the variable exist.

# Examples

```rust
use crawrad_os::set_env_for_children;

set_env_for_children("CRAWRAD_TEST_ENV", "1");
assert_eq!(std::env::var("CRAWRAD_TEST_ENV").unwrap(), "1");
```
