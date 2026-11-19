---
id: agent-profile-openai-models
kind: doc
target_file: src/agent_profile.rs
line_hint: 278
author: 
created: 
title: "OPENAI_MODELS"
tags: []
signature: "pub const OPENAI_MODELS : & [StaticModelSpec] = & [StaticModelSpec { id : \"gpt-5.6-sol\" , display_name : \"GPT-5.6 Sol (Codex Default)\" , description : \"OpenAI Flagship Coding Model\" , supports_reasoning : true , } , StaticModelSpec { id : \"o3-mini\" , display_name : \"o3-mini\" , description : \"Fast Reasoning Engine (OpenAI)\" , supports_reasoning : true , } , StaticModelSpec { id : \"o1\" , display_name : \"o1\" , description : \"Deep Reasoning Flagship (OpenAI)\" , supports_reasoning : true , } , StaticModelSpec { id : \"gpt-4o\" , display_name : \"GPT-4o\" , description : \"Omni Multimodal Flagship (OpenAI)\" , supports_reasoning : false , } , StaticModelSpec { id : \"gpt-4o-mini\" , display_name : \"GPT-4o Mini\" , description : \"Fast Affordable (OpenAI)\" , supports_reasoning : false , } ,]"
---

Verified static model catalog for OpenAI and Codex.

OpenAI models.
Source: OpenAI API documentation (September 2026).

# Examples

```rust
use crawrad_agent::OPENAI_MODELS;

assert!(!OPENAI_MODELS.is_empty());
```
