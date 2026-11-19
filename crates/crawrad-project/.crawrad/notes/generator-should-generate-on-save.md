---
id: generator-should-generate-on-save
kind: doc
target_file: src/generator.rs
line_hint: 417
author: 
created: 
title: "should_generate_on_save"
tags: []
signature: "pub fn should_generate_on_save (write_failed : bool , has_document : bool) -> bool"
---

Returns whether `Ctrl+S` should run generation after saving.

After saving, does `Ctrl+S` **generate**?

Pure, and therefore provable — the lesson of the ruler: written as an `if`
inside the shell's save, the rule would only be measurable by opening the
IDE, which is how the ruler once lost its guard and a reviewer deleted the
condition with the suite green.

Two conditions, and both are about correctness:

- **did it save?** If the form or the unit did not reach the disk, the
  sentence on the bar is the error one, and generating on top would trade
  the cause for the symptom — the user needs to see what was left unsaved,
  not the report of a generation he did not ask for;
- **is there a form?** With no document there is nothing to generate, and
  the generation would answer `generate: no form open` glued to a
  `nothing to save`.

# Examples

```rust
use crawrad_project::generator::should_generate_on_save;

assert!(should_generate_on_save(false, true));
assert!(!should_generate_on_save(true, true));
```
