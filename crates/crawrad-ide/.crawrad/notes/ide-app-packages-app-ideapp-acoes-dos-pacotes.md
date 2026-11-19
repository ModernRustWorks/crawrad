---
id: ide-app-packages-app-ideapp-acoes-dos-pacotes
kind: doc
target_file: src/ide_app/packages_app.rs
line_hint: 17
author: 
created: 
title: "IdeApp::acoes_dos_pacotes"
tags: []
signature: "pub (super) fn acoes_dos_pacotes (& mut self , cx : & mut Cx , actions : & Actions)"
---

The Package Manager gestures: two tabs, the search field, two buttons.

A dedicated method because [`IdeApp::handle_actions`] is already long, and
because everything here shares the same prerequisite — the panel existing in the
dock. Without it, `Dock::item` returns empty and every query would be a
`WidgetRef::empty()` answering "I was not clicked" forever.

**`cargo add` and `cargo remove` do NOT pass through the catalogue, and this is a
declared gap.** The owner's rule states that everything in the IDE must have an action
— *"so that the list can later be used as tools for MCP"* —, and
today neither `packages.add` nor `packages.remove` exists in
`crawrad_actions::catalogo()`: both are pure UI of the two
renderers. This shell does what egui does, through the same model door,
and the missing action is in the report of this round instead of being invented
here.

#### At `self.packages.tab = packages::model::Tab::Browse;`

SEARCHING BRINGS THE TAB ALONG. The field is visible in both — it is the
panel header, not the tab's —, and without this line an `Enter`
pressed in `Installed` would fill a list the user is not
seeing: the search would appear not to have happened.

#### At `let campo = painel.text_input(id!(packages_search.packages_query));`

THE SEARCH FIELD. `Return` triggers; each letter only stores the text.

Searching on every letter would hit the crates.io API once per keystroke —
and the panel's own phrase promises the opposite: *"type a crate name
above and press Enter to search"*.

#### At `if nova == packages::model::Tab::Installed {`

Switching to `Installed` RE-READS the manifest, which is what the
egui shell does on the same click: the list may have changed due to
a `cargo add` done outside the IDE, and a tab showing the
previous state is worse than an empty tab.

#### At `let mut abas = WidgetSet::empty();`

THE TWO TABS. The set is built from `Tab::ALL`, and not from
two paths written here: this way the enum ORDER and the button
order have no way to diverge. What unchecks the other is `RadioButtonSet`
itself (`radio_button.rs:1420-1433`).

## Inside `acoes_dos_pacotes`: `let lista = painel.portal_list(id!(packages_body.packages_list));`

THE TWO BUTTONS, by the same rule as the Explorer: the package comes from the list
that drew the row, by the index returned by `PortalList`.
