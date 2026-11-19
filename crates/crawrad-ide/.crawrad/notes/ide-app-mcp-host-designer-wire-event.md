---
id: ide-app-mcp-host-designer-wire-event
kind: doc
target_file: src/ide_app/mcp_host/designer.rs
line_hint: 120
author: 
created: 
title: "wire_event"
tags: []
signature: "fn wire_event (l : & mut Live < '_ > , args : & Value) -> crawrad_mcp :: run_tool :: Answer"
---

Wires an event handler in the designer and generates its handler stub.

`designer.wire_event`: the bind, then the stub the generator writes (#583),
as the IDE's double-click does.
