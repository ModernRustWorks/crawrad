---
id: canvas-adornments
kind: doc
target_file: src/canvas.rs
line_hint: 709
author: 
created: 
title: "Adornments"
tags: []
signature: "struct Adornments"
---

What the model reports there is to adorn in this frame.

Everything in DESIGN units: conversion belongs to the brush, and happens once, at
paint time (ADR 0019). Reading first and painting later is not style — it is
what keeps the drawing and the runtime's `rect_da_alca` talking about the same frame.

Each adornment originating from a component carries ITS clip
([`Clipped`]), and there is no second place where this answer is calculated:
where the component does not appear, there is nothing to grab.
