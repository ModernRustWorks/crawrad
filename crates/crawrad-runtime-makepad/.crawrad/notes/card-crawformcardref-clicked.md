---
id: card-crawformcardref-clicked
kind: doc
target_file: src/card.rs
line_hint: 190
author: 
created: 
title: "CrawradFormCardRef::clicked"
tags: []
signature: "pub fn clicked (& self , actions : & Actions) -> bool"
---

Whether the card's surface was tapped in `actions`.

The inner `View` is the `#[deref]` field, at the card's address, so its
own actions (`FingerDown`, `FingerUp`…) carry the SAME uid as the card's
`Clicked`. `find_widget_action` returns the first action of a uid — the
view's — and the click was never seen (#604). So every action of the uid
is looked at, for the card's own type.

# Examples

```rust,no_run
// Card actions require the Makepad event loop.
use makepad_widgets::Actions;
use crawrad_runtime_makepad::card::CrawradFormCardRef;

fn example(card: &CrawradFormCardRef, actions: &Actions) {
    let _was_clicked = card.clicked(actions);
}
```
