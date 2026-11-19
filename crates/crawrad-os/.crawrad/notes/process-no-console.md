---
id: process-no-console
kind: doc
target_file: src/process.rs
line_hint: 45
author: 
created: 
title: "no_console"
tags: []
signature: "pub fn no_console (program : impl AsRef < std :: ffi :: OsStr >) -> Command"
---

Configures a command to execute without creating a black console window.

Every child process of the product is born here, and the reason is a black
window.

On Windows the release binary has no console (`windows_subsystem` at the top
of `main.rs`) — and it is *because* the IDE has no console that the system
**creates one** for each child: a black rectangle that opens and closes in
the face of someone who only opened the program. `CREATE_NO_WINDOW`
(`0x0800_0000`) is what tells Windows not to.

**The defect is invisible during development, by construction.** In `debug`
the attribute does not apply, the IDE *has* a console, and the child
inherits it instead of getting a new one — nothing blinks. Only whoever runs
the release binary sees it, and that is how it survived until the owner
noticed it on screen. That is why the guard on this function reads the
*source*, not the behaviour: a test that only runs in `debug` would never
see what we are trying to prevent.

It is not only `cargo`: the `code.cmd` of the editor launchers is a batch
file, and a batch file without a console is a new console all the same.

# Examples

```rust
use crawrad_os::process::no_console;

let cmd = no_console("cargo");
assert_eq!(cmd.get_program(), "cargo");
```
