---
id: agent-profile-profiles
kind: doc
target_file: src/agent_profile.rs
line_hint: 334
author: 
created: 
title: "PROFILES"
tags: []
signature: "pub const PROFILES : & [AgentProfile] = & [AgentProfile { id : \"claude\" , registry_id : \"claude-acp\" , display_name : \"Claude Agent\" , default_binary : \"claude\" , default_endpoint : \"stdio:claude-agent\" , session : SessionSupport :: Pinned , prompt_delivery : PromptDelivery :: Stdin , prompt_via_stdin : true , model_source : ModelSource :: Static (CLAUDE_MODELS) , } , AgentProfile { id : \"gemini\" , registry_id : \"antigravity-acp\" , display_name : \"Gemini (Antigravity)\" , default_binary : \"agy\" , default_endpoint : \"stdio:agy\" , session : SessionSupport :: Captured , prompt_delivery : PromptDelivery :: BareStdin , prompt_via_stdin : true , model_source : ModelSource :: AgentCommand { subcommand : \"models\" , fallback : GEMINI_AGY_MODELS , } , } , AgentProfile { id : \"codex\" , registry_id : \"codex-acp\" , display_name : \"OpenAI / Codex ACP Bridge\" , default_binary : \"codex\" , default_endpoint : \"stdio:codex\" , session : SessionSupport :: Captured , prompt_delivery : PromptDelivery :: Stdin , prompt_via_stdin : true , model_source : ModelSource :: Static (OPENAI_MODELS) , } , AgentProfile { id : \"opencode\" , registry_id : \"opencode\" , display_name : \"OpenCode / Local (Ollama)\" , default_binary : \"opencode\" , default_endpoint : \"stdio:opencode\" , session : SessionSupport :: None , prompt_delivery : PromptDelivery :: Stdin , prompt_via_stdin : true , model_source : ModelSource :: Static (OPENCODE_MODELS) , } , AgentProfile { id : \"custom\" , registry_id : \"\" , display_name : \"Custom ACP Server\" , default_binary : \"\" , default_endpoint : \"\" , session : SessionSupport :: None , prompt_delivery : PromptDelivery :: Stdin , prompt_via_stdin : true , model_source : ModelSource :: Static (CLAUDE_MODELS) , } ,]"
---

Registered canonical agent profiles.

# Examples

```rust
use crawrad_agent::PROFILES;

assert!(!PROFILES.is_empty());
```
