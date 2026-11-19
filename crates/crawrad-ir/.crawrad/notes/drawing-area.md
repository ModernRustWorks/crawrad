---
id: drawing-area
kind: doc
target_file: src/drawing.rs
line_hint: 94
author: 
created: 
title: "Area"
tags: []
signature: "pub enum Area"
---

**WHERE painting occurs** — and it is orthogonal to WHAT is painted.

# Examples

```rust
use crawrad_ir::drawing::Area;

assert_eq!(Area::default(), Area::All);
```

# It was born from a failure, and the failure was provoked on purpose

The first version of this vocabulary had five verbs and **all painted the
entire rectangle**. It was enough to write the second third-party component — a
gauge that shows a VALUE — for it not to fit: there was no way to paint a
fraction of the box.

The owner's rule decided the rest: *"if `PageControl` does not fit in the format, the
format is wrong — not `PageControl`"*
(ADR 0035).

# Why it did NOT become a sixth verb

`bar` would be the easy way and the wrong answer: the next verb would ask for
`vertical_bar`, and the other `half_box`. A vocabulary that grows by cases is a
list.

The area is a **separate** question, and separating it multiplied the reach of the
five without adding any: now one can put the check in a corner, the text
in a strip, and the bar in the fraction — with the same five.
