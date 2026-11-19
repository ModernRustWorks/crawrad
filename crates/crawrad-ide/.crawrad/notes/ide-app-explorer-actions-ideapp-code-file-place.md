---
id: ide-app-explorer-actions-ideapp-code-file-place
kind: doc
target_file: src/ide_app/explorer_actions.rs
line_hint: 1
author:
created:
title: "IdeApp::code_file_place"
tags: []
signature: "fn code_file_place (& self) -> CodePlace"
---

The member and group root of the code file in front, when no form is (D9).

A console or library project has no form, and the Explorer, the chat's
folder and the LSP root used to read "No folder open". Measuring reads
Cargo.toml and `sincronizar` asks every pass, so the answer is kept by
(file, desktop root) and measured again only when one of them changes —
a disk read per pass made Enter accept a completion (#910).
