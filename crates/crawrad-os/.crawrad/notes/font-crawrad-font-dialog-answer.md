---
id: font-crawrad-font-dialog-answer
kind: doc
target_file: src/font.rs
line_hint: 25
author: 
created: 
title: "CRAWRAD_FONT_DIALOG_ANSWER"
tags: []
signature: "pub const CRAWRAD_FONT_DIALOG_ANSWER : & str = \"CRAWRAD_FONT_DIALOG_ANSWER\""
---

Environment variable providing a test answer to the font dialog.

The answer a test gives the font dialog instead of a person, as the file
dialogs' `CRAWRAD_FILE_DIALOG_ANSWER` does: empty is Cancel, anything else is
a font written the way the Object Inspector writes one (`Name, 9.0pt, Bold`).

# Examples

```rust
use crawrad_os::font::CRAWRAD_FONT_DIALOG_ANSWER;

assert_eq!(CRAWRAD_FONT_DIALOG_ANSWER, "CRAWRAD_FONT_DIALOG_ANSWER");
```
