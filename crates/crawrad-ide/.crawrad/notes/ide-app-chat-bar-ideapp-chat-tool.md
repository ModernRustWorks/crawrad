---
id: ide-app-chat-bar-ideapp-chat-tool
kind: doc
target_file: src/ide_app/chat_bar.rs
line_hint: 420
author: 
created: 
title: "IdeApp::chat_tool"
tags: []
signature: "pub (crate) fn chat_tool (& self , name : & str) -> ButtonRef"
---

Finds a button in the chat prompt tool bar by name.

Localiza o container da barra de entrada de prompt do chat de IA.
A button of the prompt's tool bar by name (`btn_prompt_send`, `btn_ai_mic`…),
found under the input bar first, anywhere in the panel otherwise.
