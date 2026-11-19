---
id: explorer-member-body
kind: doc
target_file: src/explorer.rs
line_hint: 404
author: 
created: 
title: "member_body"
tags: []
signature: "fn member_body (out : & mut Vec < Row > , m : & model :: MemberNode , view : & View , lines : & [model :: Line])"
---

Populates explorer rows for the body of a member node.

The body of one member: its forms, and — when it is the active one — the
files of the plan with the owner badge.
