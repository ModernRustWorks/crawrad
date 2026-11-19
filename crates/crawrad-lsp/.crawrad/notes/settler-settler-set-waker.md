---
id: settler-settler-set-waker
kind: doc
target_file: src/settler.rs
line_hint: 48
author: 
created: 
title: "Settler::set_waker"
tags: []
signature: "pub (crate) fn set_waker (& self , wake : impl Fn () + Send + 'static)"
---

What wakes the window when an answer arrives.

Measured on 01/10/2026: an IDE with no input sleeps, and the hints waited
for the next mouse move.
