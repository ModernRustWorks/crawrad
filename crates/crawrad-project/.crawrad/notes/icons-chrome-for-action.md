---
id: icons-chrome-for-action
kind: doc
target_file: src/icons.rs
line_hint: 1
author: 
created: 
title: "Chrome::for_action"
tags: []
signature: "pub fn for_action (action : & str) -> Option < Self >"
---

The icon a menu row shows beside an action, when it has one.

The Run menu shows the filled triangle for Run (under the debugger) and the
outline one for Run Without Debugging, as the approved mockup draws them.

# Examples

```rust
use crawrad_project::icons::Chrome;

assert_eq!(Chrome::for_action("project.run"), Some(Chrome::Start));
assert_eq!(Chrome::for_action("project.run_without_debugging"), Some(Chrome::Run));
assert_eq!(Chrome::for_action("file.save"), None);
```
