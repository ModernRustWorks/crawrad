---
id: navigation-pousocodigo
kind: doc
target_file: src/navigation.rs
line_hint: 299
author: 
created: 
title: "PousoCodigo"
tags: []
signature: "pub struct PousoCodigo"
---

Code landing target indicating line and byte offset for cursor navigation.

Where the caret or the selection lands after a jump to code (double click,
Inspector, go to). Moved out of the Makepad shell, which only applies it.

# Examples

```rust
use crawrad_host::navigation::PousoCodigo;

let pouso = PousoCodigo::at(5, 10);
assert_eq!(pouso.linha, 5);
assert_eq!(pouso.start_byte, 10);
```
