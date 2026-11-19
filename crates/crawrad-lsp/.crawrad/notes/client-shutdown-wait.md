---
id: client-shutdown-wait
kind: doc
target_file: src/client.rs
line_hint: 122
author: 
created: 
title: "SHUTDOWN_WAIT"
tags: []
signature: "const SHUTDOWN_WAIT : Duration = Duration :: from_secs (2)"
---

How long `shutdown` waits for the server to reply and exit.

How long `shutdown` waits for the server's answer, and then for its
process to leave after `exit`. rust-analyzer takes milliseconds; the
limits only bound a server that hangs.
