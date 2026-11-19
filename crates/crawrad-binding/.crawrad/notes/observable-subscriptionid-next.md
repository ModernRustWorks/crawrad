---
id: observable-subscriptionid-next
kind: doc
target_file: src/observable.rs
line_hint: 14
author: 
created: 
title: "SubscriptionId::next"
tags: []
signature: "pub fn next () -> Self"
---

Generate a unique sequential subscription identifier.

# Examples

```rust
use crawrad_binding::SubscriptionId;

let id1 = SubscriptionId::next();
let id2 = SubscriptionId::next();
assert_ne!(id1, id2);
```
