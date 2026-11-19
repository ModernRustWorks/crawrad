---
id: note-anchorvisitor-record
kind: doc
target_file: src/note.rs
line_hint: 1627
author: 
created: 
title: "AnchorVisitor::record"
tags: []
signature: "fn record (& mut self , attrs : & [syn :: Attribute] , item : & impl ToTokens)"
---

Records all `#[note]` attributes found on an item.

Records every `#[note("id")]` among `attrs`. The signature is read from
the whole item: `item_signature` skips every leading attribute, the
note itself included, exactly as the macro sees it.
