---
id: slash-commands-review-prompt
kind: doc
target_file: src/slash_commands.rs
line_hint: 218
author: 
created: 
title: "REVIEW_PROMPT"
tags: []
signature: "const REVIEW_PROMPT : & str = \"Review the attached code (or the selected code / active file). Point out correctness problems, risks and missing tests, citing line numbers. This is a read-only review: do NOT edit, create or delete any file; only report.\""
---

`/review` is read-only: it reports, it never edits (#259).
