---
id: installation-verdict-into-parts
kind: doc
target_file: src/installation.rs
line_hint: 57
author: 
created: 
title: "Verdict::into_parts"
tags: []
signature: "pub fn into_parts (self) -> Result < (String , Vec < String > , PathBuf) , String >"
---

The verdict as a `Result`: the package, its classes and where it landed.

# Errors

The refusal's reason when the package did not enter.

# Examples

```
use crawrad_project::installation::Verdict;

let refused = Verdict::Refused("no manifest.json".into());
assert_eq!(refused.into_parts(), Err("no manifest.json".to_string()));
```
