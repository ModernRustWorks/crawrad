---
id: is-inside-store-package
kind: doc
target_file: src/lib.rs
line_hint: 900
author: 
created: 
title: "is_inside_store_package"
tags: []
signature: "pub fn is_inside_store_package (path : & std :: path :: Path) -> bool"
---

Whether this path lies inside the Store's protected package folder.

An installed Store app lives under `C:\Program Files\WindowsApps\`, a folder
Windows locks down: the package itself reads it, so an `is_file()` check answers
yes, and every other process is refused.

Two places need this fact for different reasons - where components may be
installed, and which command may be written into a project's `.mcp.json` - so it
lives here, with the rest of what this house knows about the operating system,
instead of being stated twice and drifting apart.

# Examples

The folder is recognised by a path COMPONENT named `WindowsApps`, whatever its
case, so the check reads the same on every platform - which is how this example
runs on Linux and macOS too, where a backslash separates nothing and the real
Windows spelling would be a single component.

```rust
use crawrad_os::is_inside_store_package;
use std::path::Path;

assert!(is_inside_store_package(Path::new("/var/lib/WindowsApps/crawrad-ide")));
assert!(is_inside_store_package(Path::new("/var/lib/windowsapps/crawrad-ide")));
assert!(!is_inside_store_package(Path::new("/usr/local/bin/crawrad-ide")));
```
