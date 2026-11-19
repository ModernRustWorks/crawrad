---
id: ide-app-effects-ideapp-refusal
kind: doc
target_file: src/ide_app/effects.rs
line_hint: 1
author: 
created: 
title: "IdeApp::refusal"
tags: []
signature: "pub (super) fn refusal (& self , action_id : & str , args : & crawrad_actions :: Args) -> Option < String >"
---

Why the action would be refused now, or `None` when it may run.

The MCP asks before running, so `ide.run_action` can answer
`action-disabled` with the reason instead of *"executed"*. The reason is
the action's unmet `Need` (*"Generate needs a form in the project"*); a
need that is met but still refused says *"not available now"*. An unknown id is
not refused here: `execute_with` reports it.
