---
id: client-answer-server-request
kind: doc
target_file: src/client.rs
line_hint: 1130
author: 
created: 
title: "answer_server_request"
tags: []
signature: "pub (crate) fn answer_server_request (payload : & str , reply : & Sender < String > , inlay_refresh : & AtomicBool , code_lens_refresh : & AtomicBool ,) -> bool"
---

Answers requests originating from the language server with null.

A REQUEST from the server (it has both `id` and `method`) gets an answer:
`null`, which is what every request rust-analyzer sends accepts
(`window/workDoneProgress/create`, `client/registerCapability`,
`workspace/*/refresh`). Before this they were read as responses to a
request of ours, found no one waiting, and were dropped unanswered.
`workspace/inlayHint/refresh` and `workspace/codeLens/refresh` also raise the flags the IDE polls.
