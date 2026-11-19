---
id: main-window
kind: doc
target_file: src/lib.rs
line_hint: 266
author: 
created: 
title: "main_window"
tags: []
signature: "fn main_window () -> Option < windows_sys :: Win32 :: Foundation :: HWND >"
---

The one top-level window this process owns, if there is exactly one.

Exactly one, and the strictness is deliberate: this crate has no way to
know which of two windows the caller meant, and moving the wrong one is
worse than moving none. The IDE has a single window — its dialogs are
views inside it, not windows of their own — so the ambiguous case means
something unexpected is on screen, and the caller falls back.

`EnumThreadWindows` and not `GetActiveWindow`: a native file dialog is
modal and owned, and it would be the active one while it is up. Owned
windows are filtered out below.
