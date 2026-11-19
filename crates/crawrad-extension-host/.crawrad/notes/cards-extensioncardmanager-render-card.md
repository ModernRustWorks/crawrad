---
id: cards-extensioncardmanager-render-card
kind: doc
target_file: src/cards.rs
line_hint: 61
author: 
created: 
title: "ExtensionCardManager::render_card"
tags: []
signature: "pub fn render_card (& mut self , card_id : & str , root : UiNode) -> Result < () , ExtensionError >"
---

Set or replace the entire declarative UI tree of a card.

# Examples

```rust
use crawrad_extension_api::contribution::{CardArea, CardContribution};
use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};
use crawrad_extension_api::ui::UiNode;
use crawrad_extension_host::ExtensionCardManager;

let mut mgr = ExtensionCardManager::new();
let manifest = ExtensionManifest {
    id: "ext1".to_string(),
    name: "Ext".to_string(),
    version: "0.1.0".to_string(),
    publisher: "Acme".to_string(),
    description: String::new(),
    entrypoint: None,
    activation_events: Vec::new(),
    contributions: ExtensionContributions {
        cards: vec![CardContribution {
            id: "card1".to_string(),
            title: "Card".to_string(),
            area: CardArea::Bottom,
            icon: None,
            closeable: true,
        }],
        ..Default::default()
    },
};
mgr.register_manifest_cards(&manifest);
assert!(mgr.render_card("card1", UiNode::label("lbl", "Hi")).is_ok());
```

# Errors

Returns [`ExtensionError::NotFound`](crawrad_extension_api::error::ExtensionError::NotFound) if the card ID has not been registered.
