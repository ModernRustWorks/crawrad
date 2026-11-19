---
id: reservadas
kind: doc
target_file: src/lib.rs
line_hint: 812
author: 
created: 
title: "RESERVADAS"
tags: []
signature: "const RESERVADAS : & [& str] = & [\"as\" , \"break\" , \"const\" , \"continue\" , \"crate\" , \"else\" , \"enum\" , \"extern\" , \"false\" , \"fn\" , \"for\" , \"if\" , \"impl\" , \"in\" , \"let\" , \"loop\" , \"match\" , \"mod\" , \"move\" , \"mut\" , \"pub\" , \"ref\" , \"return\" , \"self\" , \"Self\" , \"static\" , \"struct\" , \"super\" , \"trait\" , \"true\" , \"type\" , \"unsafe\" , \"use\" , \"where\" , \"while\" , \"async\" , \"await\" , \"dyn\" , \"abstract\" , \"become\" , \"box\" , \"do\" , \"final\" , \"macro\" , \"override\" , \"priv\" , \"try\" , \"typeof\" , \"unsized\" , \"virtual\" , \"yield\" , \"gen\" , \"new\" , \"show\" ,]"
---

Words reserved by Rust that cannot be identifiers.

The words that Rust reserves. A component named `type` would generate a
field `type`, which does not compile.

**Two families, and the second was missing.** In addition to keywords currently in use,
Rust reserves a handful *for future use* — `box`, `do`, `macro`, `priv`,
`typeof`, `virtual`, `yield`, and the like. They do nothing today and yet
**cannot be identifiers**, which is why they are not guessable: anyone
writing the list from memory remembers `type` and not `box`.

The loophole was reachable via the Object Inspector: a `Name` of `box` passed
validation, became a field in `.designer.rs`, and the generated project failed to compile
— with the error pointing to a file that the user never wrote.

The list is maintained data, and the trigger is concrete: **a new Rust
edition**. 2018 added `async`, `await`, `dyn`, and `try`; 2024,
`gen`.
