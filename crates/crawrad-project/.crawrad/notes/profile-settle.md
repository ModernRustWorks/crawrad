---
id: profile-settle
kind: doc
target_file: src/profile.rs
line_hint: 82
author: 
created: 
title: "settle"
tags: []
signature: "pub fn settle (command : & ProfileCommand , done : Result < () , String >) -> String"
---

Formats what a profile command tells the user upon completion.

What a profile command that ended in `done` tells the user. Only a command
that worked changes the active profile: one applied or saved becomes it,
and deleting it leaves [`DEFAULT`] active.

# Examples

```rust,no_run
use crawrad_actions::ProfileCommand;
use crawrad_project::profile::settle;

// Avoid writing preferences file during doctests.
let msg = settle(&ProfileCommand::Apply("Custom".into()), Ok(()));
assert_eq!(msg, "profile `Custom` applied");
```
