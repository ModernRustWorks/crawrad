---
id: keys-char-keys
kind: doc
target_file: src/keys.rs
line_hint: 84
author: 
created: 
title: "CHAR_KEYS"
tags: []
signature: "pub const CHAR_KEYS : [KeyCode ; 36] = [KeyCode :: KeyA , KeyCode :: KeyB , KeyCode :: KeyC , KeyCode :: KeyD , KeyCode :: KeyE , KeyCode :: KeyF , KeyCode :: KeyG , KeyCode :: KeyH , KeyCode :: KeyI , KeyCode :: KeyJ , KeyCode :: KeyK , KeyCode :: KeyL , KeyCode :: KeyM , KeyCode :: KeyN , KeyCode :: KeyO , KeyCode :: KeyP , KeyCode :: KeyQ , KeyCode :: KeyR , KeyCode :: KeyS , KeyCode :: KeyT , KeyCode :: KeyU , KeyCode :: KeyV , KeyCode :: KeyW , KeyCode :: KeyX , KeyCode :: KeyY , KeyCode :: KeyZ , KeyCode :: Key0 , KeyCode :: Key1 , KeyCode :: Key2 , KeyCode :: Key3 , KeyCode :: Key4 , KeyCode :: Key5 , KeyCode :: Key6 , KeyCode :: Key7 , KeyCode :: Key8 , KeyCode :: Key9 ,]"
---

Alphanumeric keys in KeyCode enum order.

# Examples

```rust
use crawrad_runtime_makepad::keys::CHAR_KEYS;

assert_eq!(CHAR_KEYS.len(), 36);
```
