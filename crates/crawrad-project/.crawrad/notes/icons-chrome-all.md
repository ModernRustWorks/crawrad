---
id: icons-chrome-all
kind: doc
target_file: src/icons.rs
line_hint: 215
author: 
created: 
title: "Chrome::ALL"
tags: []
signature: "pub const ALL : & 'static [Self] = & [Self :: Undo , Self :: Redo , Self :: Save , Self :: SaveAll , Self :: Build , Self :: Run , Self :: Preview , Self :: Explorer , Self :: Components , Self :: Inspector , Self :: Packages , Self :: ExtensionViews , Self :: Tests , Self :: Settings , Self :: New , Self :: ClearChat , Self :: Close , Self :: Unsaved , Self :: Check , Self :: Error , Self :: Warning , Self :: Zoom , Self :: Attach , Self :: Memory , Self :: Mcp , Self :: History , Self :: Agent , Self :: Sparkle , Self :: SplitHorizontal , Self :: More , Self :: ChevronDown , Self :: ChevronRight , Self :: ArrowRight , Self :: Edit , Self :: Queue , Self :: NewFile , Self :: NewFolder , Self :: NewForm , Self :: Refresh , Self :: CollapseAll , Self :: Lock , Self :: Debug , Self :: DebugContinue , Self :: DebugPause , Self :: StepOver , Self :: StepInto , Self :: StepOut , Self :: DebugStop , Self :: Start ,]"
---

Every role, so a test can walk them without a second list.

# Examples

```rust
use crawrad_project::icons::Chrome;

assert!(!Chrome::ALL.is_empty());
```
