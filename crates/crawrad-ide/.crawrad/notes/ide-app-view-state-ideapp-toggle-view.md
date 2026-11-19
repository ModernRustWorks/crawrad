---
id: ide-app-view-state-ideapp-toggle-view
kind: doc
target_file: src/ide_app/view_state.rs
line_hint: 151
author: 
created: 
title: "IdeApp::toggle_view"
tags: []
signature: "pub fn toggle_view (& mut self , cx : & mut Cx)"
---

`F12`: form ⇄ unit, and the unit is now a TAB.

Going to code OPENS the form unit in the strip, instead of
lighting up a fixed `Code` tab; returning to the design leaves the tab open,
like any editor. Reopening the same unit only brings it to the front —
the rule belongs to [`crate::view_models::editor_tabs::EditorTabs::open`].

# Examples

```rust,ignore
// binary crate: doctests cannot reach main.rs items
let prev = app.host.center();
app.toggle_view(cx);
assert_ne!(app.host.center(), prev);
```

#### At `match self.host.center().toggle_code() {`

THE KEY RULE remains that of [`Center::toggle_code`], pure and
with its own gate: `FormSource` goes to the unit and not back
to the design, because `F12` means *the other half of the pair*. This
shell only OBEYS what it decides — writing the decision here would be a
second copy of it, and copies become stale.

## Inside `toggle_view`: `self.show_center(cx, Center::Code);`

WITH NO OPEN FORM there is no unit to open, and the code
view continues to be the one explaining what is missing — and
it carries `NOT_GENERATED_YET`.
