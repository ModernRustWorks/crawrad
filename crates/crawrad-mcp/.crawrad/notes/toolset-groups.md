---
id: toolset-groups
kind: doc
target_file: src/toolset.rs
line_hint: 11
author: 
created: 
title: "GROUPS"
tags: []
signature: "pub const GROUPS : & [(& str , & [& str])] = & [(\"design\" , & [\"designer\" , \"component\" , \"collection\" , \"align\" , \"order\" , \"layout\" , \"inspector\" , \"form\" , \"strings\" , \"page\" , \"event\" , \"binding\" , \"action\" , \"edit\" , \"view\" , \"ide\" , \"dialog\" ,] ,) , (\"code\" , & [\"editor\" , \"code\" , \"lsp\" , \"file\" , \"build\" , \"edit\" , \"view\" , \"ide\" , \"dialog\" ,] ,) , (\"debug\" , & [\"debug\" , \"run\" , \"build\" , \"editor\" , \"view\" , \"ide\" , \"dialog\"] ,) , (\"project\" , & [\"project\" , \"explorer\" , \"file\" , \"build\" , \"tools\" , \"help\" , \"view\" , \"ide\" , \"dialog\" ,] ,) , (\"chat\" , & [\"chat\" , \"acp\" , \"ai\" , \"view\" , \"ide\" , \"dialog\"]) , (EXTENSIONS , & [\"view\"]) ,]"
---

Each group and the tool prefixes it carries.

# Examples

```rust
use crawrad_mcp::toolset::GROUPS;

assert!(!GROUPS.is_empty());
```
