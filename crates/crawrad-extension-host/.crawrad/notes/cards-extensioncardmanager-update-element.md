---
id: cards-extensioncardmanager-update-element
kind: doc
target_file: src/cards.rs
line_hint: 72
author: 
created: 
title: "ExtensionCardManager::update_element"
tags: []
signature: "pub fn update_element (& mut self , card_id : & str , element_id : & str , patch : UiElementPatch ,) -> Result < () , ExtensionError >"
---

Apply a partial patch to an element inside a card without rebuilding the DOM.

# Examples

```rust
use crawrad_extension_api::contribution::{CardArea, CardContribution};
use crawrad_extension_api::manifest::{ExtensionContributions, ExtensionManifest};
use crawrad_extension_api::ui::{UiElementPatch, UiNode};
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
mgr.render_card("card1", UiNode::label("lbl", "Hi")).unwrap();
let patch = UiElementPatch { text: Some("New".into()), ..Default::default() };
assert!(mgr.update_element("card1", "lbl", patch).is_ok());
```

# Errors

Returns [`ExtensionError::NotFound`](crawrad_extension_api::error::ExtensionError::NotFound) if the card or element ID is not found.
