---
id: layout-margins
kind: doc
target_file: src/layout.rs
line_hint: 57
author: 
created: 
title: "Margins"
tags: []
signature: "pub struct Margins"
---

Manages control margins for alignment.

The four margins of the classic component library (`TControl.Margins`), and the switch that
tells `Align` to honour them (`TControl.AlignWithMargins`).

**One read, two questions.** [`Margins::from_value`] answers the raw question
— *how much space does this control want around itself?* — and always holds, with the
same default 3 of the classic component library; it is what the intelligent guide of dragging asks.
[`Margins::effective`] answers the alignment question — *should `Align`
honour this?* — and is the first passed through the switch. One of them is the
other with `AlignWithMargins` applied, and never a second table.

The switch emerging from inside the read is what prevents an `if` in the cascade:
turned off, [`Margins::effective`] returns four zeros and the alignment
calculation is that from before margins existed, byte for byte. And since the default
of the classic component library is `AlignWithMargins = False`, every `.crawform` that already exists
continues opening exactly where it was.

# Examples

```rust
use crawrad_designer::layout::Margins;

assert_eq!(Margins::DEFAULT.left, 3.0);
```
