---
id: ide-app-chat-bar-ideapp-history-json
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 134
author: 
created: 
title: "IdeApp::history_json"
tags: []
signature: "pub (super) fn history_json (& self , cx : & Cx) -> serde_json :: Value"
---

Returns JSON state for the chat history popup.

`view.get_chat.history_list`: whether it is open, its search box and each row's rect.
