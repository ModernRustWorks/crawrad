---
id: workspace-config-is-inside-package
kind: doc
target_file: src/workspace_config.rs
line_hint: 32
author: 
created: 
title: "is_inside_package"
tags: []
signature: "pub fn is_inside_package (exe : & std :: path :: Path) -> bool"
---

Whether this path lies inside the Store's protected package folder.

**A path inside `WindowsApps` is unusable to everyone but the package.** An
installed Store app lives under `C:\Program Files\WindowsApps\`, a folder Windows
locks down: the package itself reads it, so `is_file()` answers yes, and every
other process is refused. Writing that path into `.mcp.json` sent the agent's CLI
- a separate process - to `fork/exec ... Access is denied`, and it started with no
MCP server at all.

So a path in there is never written. The bare name is, and the manifest's
execution alias is what makes it resolve.

# Examples

The folder is recognised by a path COMPONENT named `WindowsApps`, so the check
reads the same on every platform - which is how this example runs on Linux and
macOS too, where a backslash separates nothing and the real Windows spelling
would be a single component.

```rust
use crawrad_mcp::workspace_config::is_inside_package;
use std::path::Path;

assert!(is_inside_package(Path::new("/var/lib/WindowsApps/crawrad-mcp-bridge")));
assert!(is_inside_package(Path::new("/var/lib/windowsapps/crawrad-mcp-bridge")));
assert!(!is_inside_package(Path::new("/usr/local/bin/crawrad-mcp-bridge")));
```
