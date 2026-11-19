---
id: ide-app-chat-teacher-ideapp-teacher-grounding
kind: doc
target_file: src/ide_app/chat_teacher.rs
line_hint: 26
author: 
created: 
title: "IdeApp::teacher_grounding"
tags: []
signature: "fn teacher_grounding (& self , cx : & Cx) -> Option < crawrad_agent :: teacher :: Grounding >"
---

Collects grounding info for teacher commands.

What `/why` and `/fix-explain` ground on: the caret's diagnostic, the
lines around it and the compiler's own explanation of its code.
