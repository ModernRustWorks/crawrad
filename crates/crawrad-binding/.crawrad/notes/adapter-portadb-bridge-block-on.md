---
id: adapter-portadb-bridge-block-on
kind: doc
target_file: src/adapter/portadb_bridge.rs
line_hint: 262
author: 
created: 
title: "block_on"
tags: []
signature: "fn block_on < T , F : std :: future :: Future < Output = Result < T , portadb :: Error > > > (future : F ,) -> Result < T , String >"
---

Runs one of the dataset's async writes to completion on a private runtime.

# Why blocking is right here, and where it is wrong

The generated apps are not inside a tokio runtime -- the UI thread is the
framework's own loop -- and the local engines answer in microseconds, so a
write that blocks is what a classic RAD `Post` always did. Calling this from
INSIDE a tokio runtime panics ("Cannot start a runtime from within a runtime"):
async code drives the [`Dataset`] directly instead of through this store.
