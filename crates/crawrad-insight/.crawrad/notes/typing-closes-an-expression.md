---
id: typing-closes-an-expression
kind: doc
target_file: src/typing.rs
line_hint: 155
author: 
created: 
title: "CLOSES_AN_EXPRESSION"
tags: []
signature: "const CLOSES_AN_EXPRESSION : & [char] = & ['\"' , '\\'' , ')' , ']' , '?']"
---

Characters that terminate an expression before dot member access.

What ends an expression whose `.` is a member access: a string or char
literal, a call, an index, a `?`.
