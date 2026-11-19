---
id: agent-profile-claude-models
kind: doc
target_file: src/agent_profile.rs
line_hint: 210
author: 
created: 
title: "CLAUDE_MODELS"
tags: []
signature: "pub const CLAUDE_MODELS : & [StaticModelSpec] = & [StaticModelSpec { id : \"sonnet\" , display_name : \"Sonnet (Claude Default)\" , description : \"Standard Frontier & Hybrid Reasoning (Anthropic)\" , supports_reasoning : true , } , StaticModelSpec { id : \"opus\" , display_name : \"Opus (Deep Thinking)\" , description : \"Frontier Deep Analysis (Anthropic)\" , supports_reasoning : true , } , StaticModelSpec { id : \"haiku\" , display_name : \"Haiku (Fast & Compact)\" , description : \"Lightweight High Speed (Anthropic)\" , supports_reasoning : false , } ,]"
---

Verified static model catalog for Anthropic Claude.

Anthropic Claude models: aliases always resolve to latest versions in Claude Code.
Source: Anthropic Claude Code CLI documentation (September 2026).

# Examples

```rust
use crawrad_agent::CLAUDE_MODELS;

assert!(!CLAUDE_MODELS.is_empty());
```
