---
id: project-template-write-file
kind: doc
target_file: src/project_template.rs
line_hint: 511
author: 
created: 
title: "write_file"
tags: []
signature: "fn write_file (folder : & Path , name : & str , content : impl AsRef < [u8] >) -> Result < () , String >"
---

Writes `content` at `folder/name`; the error names the file as the user sees it.
