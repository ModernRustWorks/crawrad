---
id: events-payload-table
kind: doc
target_file: src/events.rs
line_hint: 155
author: 
created: 
title: "PAYLOAD_TABLE"
tags: []
signature: "pub const PAYLOAD_TABLE : & [PayloadType] = & [PayloadType { name : \"ShiftState\" , needs : & [] , text : SHIFT_STATE , } , PayloadType { name : \"MouseButton\" , needs : & [] , text : MOUSE_BUTTON , } , PayloadType { name : \"Key\" , needs : & [] , text : KEY , } , PayloadType { name : \"MouseEvent\" , needs : & [\"MouseButton\" , \"ShiftState\"] , text : MOUSE_EVENT , } , PayloadType { name : \"MouseMoveEvent\" , needs : & [\"ShiftState\"] , text : MOUSE_MOVE_EVENT , } , PayloadType { name : \"KeyEvent\" , needs : & [\"Key\" , \"ShiftState\"] , text : KEY_EVENT , } , PayloadType { name : \"MouseActivateEvent\" , needs : & [\"MouseButton\" , \"ShiftState\"] , text : MOUSE_ACTIVATE_EVENT , } , PayloadType { name : \"Canvas\" , needs : & [] , text : CANVAS , } ,]"
---

The full payload vocabulary ordered by dependencies.

**The whole payload vocabulary**, in dependency order: a type only names
types that come before it, so emitting a filtered prefix-respecting subset
always compiles.

# Examples

```rust
use crawrad_codegen::events::PAYLOAD_TABLE;

assert!(!PAYLOAD_TABLE.is_empty());
assert_eq!(PAYLOAD_TABLE[0].name, "ShiftState");
```
