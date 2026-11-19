//! Runtime state management and event dispatcher for extension-contributed UI cards.

use crawrad_extension_api::contribution::{CardArea, CardContribution};
use crawrad_extension_api::error::ExtensionError;
use crawrad_extension_api::manifest::ExtensionManifest;
use crawrad_extension_api::ui::{UiElementKind, UiElementPatch, UiEventNotification, UiNode};
use crawrad_note::note;
use std::collections::HashMap;

#[note("cards-uicardstate")]
#[derive(Debug, Clone, PartialEq)]
pub struct UiCardState {
    pub id: String,
    pub extension_id: String,
    pub title: String,
    pub area: CardArea,
    pub closeable: bool,
    pub open: bool,
    pub root: Option<UiNode>,
}

impl UiCardState {
    pub fn new(extension_id: &str, contribution: &CardContribution) -> Self {
        Self {
            id: contribution.id.clone(),
            extension_id: extension_id.to_string(),
            title: contribution.title.clone(),
            area: contribution.area,
            closeable: contribution.closeable,
            open: false,
            root: None,
        }
    }
}

#[note("cards-extensioncardmanager")]
#[derive(Debug, Default, Clone)]
pub struct ExtensionCardManager {
    cards: HashMap<String, UiCardState>,
    events_log: Vec<UiEventNotification>,
}

impl ExtensionCardManager {
    pub fn new() -> Self {
        Self::default()
    }

    #[note("cards-extensioncardmanager-register-manifest-cards")]
    pub fn register_manifest_cards(&mut self, manifest: &ExtensionManifest) {
        for card_contrib in &manifest.contributions.cards {
            let state = UiCardState::new(&manifest.id, card_contrib);
            self.cards.insert(card_contrib.id.clone(), state);
        }
    }

    #[note("cards-extensioncardmanager-forget")]
    pub fn forget(&mut self, extension_id: &str) {
        self.cards.retain(|_, c| c.extension_id != extension_id);
    }

    #[note("cards-extensioncardmanager-render-card")]
    pub fn render_card(&mut self, card_id: &str, root: UiNode) -> Result<(), ExtensionError> {
        let card = self
            .cards
            .get_mut(card_id)
            .ok_or_else(|| ExtensionError::NotFound(format!("Card '{card_id}' not found")))?;
        card.root = Some(root);
        card.open = true;
        Ok(())
    }

    #[note("cards-extensioncardmanager-update-element")]
    pub fn update_element(
        &mut self,
        card_id: &str,
        element_id: &str,
        patch: UiElementPatch,
    ) -> Result<(), ExtensionError> {
        let card = self
            .cards
            .get_mut(card_id)
            .ok_or_else(|| ExtensionError::NotFound(format!("Card '{card_id}' not found")))?;

        let root = card.root.as_mut().ok_or_else(|| {
            ExtensionError::NotFound(format!("Card '{card_id}' has no rendered root"))
        })?;

        let target_node = Self::find_node_mut(root, element_id).ok_or_else(|| {
            ExtensionError::NotFound(format!(
                "Element '{element_id}' not found in card '{card_id}'"
            ))
        })?;

        Self::apply_patch(target_node, patch);
        Ok(())
    }

    #[note("cards-extensioncardmanager-close-card")]
    pub fn close_card(&mut self, card_id: &str) -> Result<(), ExtensionError> {
        let card = self
            .cards
            .get_mut(card_id)
            .ok_or_else(|| ExtensionError::NotFound(format!("Card '{card_id}' not found")))?;
        card.open = false;
        Ok(())
    }

    #[note("cards-extensioncardmanager-get-card")]
    pub fn get_card(&self, card_id: &str) -> Option<&UiCardState> {
        self.cards.get(card_id)
    }

    #[note("cards-extensioncardmanager-get-card-mut")]
    pub fn get_card_mut(&mut self, card_id: &str) -> Option<&mut UiCardState> {
        self.cards.get_mut(card_id)
    }

    #[note("cards-extensioncardmanager-list-cards")]
    pub fn list_cards(&self) -> Vec<&UiCardState> {
        self.cards.values().collect()
    }

    #[note("cards-extensioncardmanager-record-event")]
    pub fn record_event(&mut self, notif: UiEventNotification) {
        self.events_log.push(notif);
    }

    #[note("cards-extensioncardmanager-take-events")]
    pub fn take_events(&mut self) -> Vec<UiEventNotification> {
        std::mem::take(&mut self.events_log)
    }

    #[note("cards-extensioncardmanager-find-node")]
    pub fn find_node<'a>(root: &'a UiNode, element_id: &str) -> Option<&'a UiNode> {
        if root.id == element_id {
            return Some(root);
        }
        if let UiElementKind::Container { children, .. } = &root.kind {
            for child in children {
                if let Some(found) = Self::find_node(child, element_id) {
                    return Some(found);
                }
            }
        }
        None
    }

    #[note("cards-extensioncardmanager-find-node-mut")]
    pub fn find_node_mut<'a>(root: &'a mut UiNode, element_id: &str) -> Option<&'a mut UiNode> {
        if root.id == element_id {
            return Some(root);
        }
        if let UiElementKind::Container { children, .. } = &mut root.kind {
            for child in children {
                if let Some(found) = Self::find_node_mut(child, element_id) {
                    return Some(found);
                }
            }
        }
        None
    }

    #[note("cards-extensioncardmanager-apply-patch")]
    pub fn apply_patch(node: &mut UiNode, patch: UiElementPatch) {
        if let Some(visible) = patch.visible {
            node.visible = visible;
        }
        if let Some(enabled) = patch.enabled {
            node.enabled = enabled;
        }

        match &mut node.kind {
            UiElementKind::Label { text, .. } => {
                if let Some(t) = patch.text {
                    *text = t;
                }
            }
            UiElementKind::Button { text, .. } => {
                if let Some(t) = patch.text {
                    *text = t;
                }
            }
            UiElementKind::TextInput { text, .. } => {
                if let Some(t) = patch.text {
                    *text = t;
                }
            }
            UiElementKind::Checkbox { checked, label } => {
                if let Some(c) = patch.checked {
                    *checked = c;
                }
                if let Some(t) = patch.text {
                    *label = t;
                }
            }
            UiElementKind::List { items, selected_id } => {
                if let Some(new_items) = patch.items {
                    *items = new_items;
                }
                if let Some(sel) = patch.selected_id {
                    *selected_id = Some(sel);
                }
            }
            UiElementKind::Tree { selected_id, .. } => {
                if let Some(sel) = patch.selected_id {
                    *selected_id = Some(sel);
                }
            }
            UiElementKind::Container { .. } => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crawrad_extension_api::manifest::ExtensionContributions;
    use crawrad_extension_api::ui::{ContainerLayout, UiEventKind};

    fn make_test_manifest() -> ExtensionManifest {
        ExtensionManifest {
            id: "test.ext".into(),
            name: "Test Ext".into(),
            version: "1.0.0".into(),
            publisher: "Test".into(),
            description: "Test".into(),
            entrypoint: None,
            activation_events: Vec::new(),
            contributions: ExtensionContributions {
                actions: Vec::new(),
                menus: Vec::new(),
                cards: vec![CardContribution {
                    id: "panel.test".into(),
                    title: "Test Panel".into(),
                    area: CardArea::Right,
                    icon: None,
                    closeable: true,
                }],
                tools: Vec::new(),
                templates: Vec::new(),
                views: Vec::new(),
            },
        }
    }

    #[test]
    fn test_card_manager_registration_and_rendering() {
        let mut mgr = ExtensionCardManager::new();
        let manifest = make_test_manifest();
        mgr.register_manifest_cards(&manifest);

        let card = mgr
            .get_card("panel.test")
            .expect("card should be registered");
        assert_eq!(card.title, "Test Panel");
        assert_eq!(card.area, CardArea::Right);
        assert!(!card.open);
        assert!(card.root.is_none());

        // Render card
        let root = UiNode::container(
            "root_container",
            ContainerLayout::Vertical,
            vec![
                UiNode::label("lbl_hello", "Welcome"),
                UiNode::button("btn_action", "Click Here"),
            ],
        );

        mgr.render_card("panel.test", root).expect("render card");
        let updated = mgr.get_card("panel.test").unwrap();
        assert!(updated.open);
        assert!(updated.root.is_some());
    }

    #[test]
    fn test_card_manager_patch_element() {
        let mut mgr = ExtensionCardManager::new();
        let manifest = make_test_manifest();
        mgr.register_manifest_cards(&manifest);

        let root = UiNode::container(
            "root_container",
            ContainerLayout::Vertical,
            vec![
                UiNode::label("lbl_status", "Initial"),
                UiNode::button("btn_run", "Start"),
            ],
        );
        mgr.render_card("panel.test", root).expect("render card");

        let patch = UiElementPatch {
            text: Some("Running...".into()),
            enabled: Some(false),
            ..Default::default()
        };

        mgr.update_element("panel.test", "btn_run", patch)
            .expect("update element");

        let card = mgr.get_card("panel.test").unwrap();
        let root = card.root.as_ref().unwrap();
        let btn = ExtensionCardManager::find_node(root, "btn_run").expect("find button");
        assert!(!btn.enabled);
        match &btn.kind {
            UiElementKind::Button { text, .. } => assert_eq!(text, "Running..."),
            _ => panic!("Expected button"),
        }

        // Verify other elements unchanged
        let lbl = ExtensionCardManager::find_node(root, "lbl_status").expect("find label");
        assert!(lbl.enabled);
        match &lbl.kind {
            UiElementKind::Label { text, .. } => assert_eq!(text, "Initial"),
            _ => panic!("Expected label"),
        }
    }

    #[test]
    fn test_card_manager_events_and_close() {
        let mut mgr = ExtensionCardManager::new();
        let manifest = make_test_manifest();
        mgr.register_manifest_cards(&manifest);

        mgr.record_event(UiEventNotification {
            card_id: "panel.test".into(),
            element_id: "btn_run".into(),
            event: UiEventKind::Click,
        });
        let events = mgr.take_events();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].card_id, "panel.test");
        assert_eq!(events[0].element_id, "btn_run");
        assert_eq!(events[0].event, UiEventKind::Click);

        assert!(mgr.take_events().is_empty());

        // Close card
        mgr.close_card("panel.test").expect("close card");
        let card = mgr.get_card("panel.test").unwrap();
        assert!(!card.open);
    }
}
