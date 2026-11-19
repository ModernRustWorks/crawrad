---
id: agent-profile-gemini-agy-models
kind: doc
target_file: src/agent_profile.rs
line_hint: 232
author: 
created: 
title: "GEMINI_AGY_MODELS"
tags: []
signature: "pub const GEMINI_AGY_MODELS : & [StaticModelSpec] = & [StaticModelSpec { id : \"gemini-3.8-flash-high\" , display_name : \"Gemini 3.8 Flash (High)\" , description : \"Next-Gen 1M High Reasoning (Google)\" , supports_reasoning : true , } , StaticModelSpec { id : \"gemini-3.8-flash-medium\" , display_name : \"Gemini 3.8 Flash (Medium)\" , description : \"Next-Gen 1M Balanced Reasoning (Google)\" , supports_reasoning : true , } , StaticModelSpec { id : \"gemini-3.7-flash-high\" , display_name : \"Gemini 3.7 Flash (High)\" , description : \"Hybrid Reasoning 1M (Google)\" , supports_reasoning : true , } , StaticModelSpec { id : \"gemini-3.1-pro-high\" , display_name : \"Gemini 3.1 Pro (High)\" , description : \"Frontier 2M Reasoning (Google)\" , supports_reasoning : true , } , StaticModelSpec { id : \"claude-sonnet-4-6\" , display_name : \"Claude Sonnet 4.6 (via agy)\" , description : \"Anthropic Frontier Hosted via Antigravity\" , supports_reasoning : false , } , StaticModelSpec { id : \"claude-opus-4-6-thinking\" , display_name : \"Claude Opus 4.6 Thinking (via agy)\" , description : \"Anthropic Thinking Hosted via Antigravity\" , supports_reasoning : true , } , StaticModelSpec { id : \"gpt-oss-120b-medium\" , display_name : \"GPT-OSS 120B (via agy)\" , description : \"Open-Weights MoE Balanced Reasoning\" , supports_reasoning : true , } ,]"
---

Verified static model catalog for Google Gemini and Antigravity.

Real models supported by Google Gemini / Antigravity CLI (`agy models`).
Source: Verified output of `agy models` (September 2026).

# Examples

```rust
use crawrad_agent::GEMINI_AGY_MODELS;

assert!(!GEMINI_AGY_MODELS.is_empty());
```
