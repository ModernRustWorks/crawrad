---
id: observable-subscriptionid
kind: doc
target_file: src/observable.rs
line_hint: 7
author: 
created: 
title: "SubscriptionId"
tags: []
signature: "pub struct SubscriptionId (u64)"
---

Unique identifier for an active subscription listener.

# Examples

```rust
use crawrad_binding::SubscriptionId;

let id = SubscriptionId::next();
assert_ne!(id, SubscriptionId::next());
```
