---
id: agent-profile-opencode-models
kind: doc
target_file: src/agent_profile.rs
line_hint: 312
author: 
created: 
title: "OPENCODE_MODELS"
tags: []
signature: "pub const OPENCODE_MODELS : & [StaticModelSpec] = & [StaticModelSpec { id : \"qwen2.5-coder:7b\" , display_name : \"Qwen 2.5 Coder 7B\" , description : \"Local Code Specialist (Alibaba)\" , supports_reasoning : false , } , StaticModelSpec { id : \"deepseek-coder-v2\" , display_name : \"DeepSeek Coder v2\" , description : \"Local MoE Architecture (DeepSeek)\" , supports_reasoning : true , } , StaticModelSpec { id : \"llama-3.3-70b\" , display_name : \"Llama 3.3 70B\" , description : \"Local Flagship (Meta)\" , supports_reasoning : true , } ,]"
---

Verified static model catalog for OpenCode and Ollama.

OpenCode / Ollama local models.
Source: Ollama library documentation (September 2026).

# Examples

```rust
use crawrad_agent::OPENCODE_MODELS;

assert!(!OPENCODE_MODELS.is_empty());
```
