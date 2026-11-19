---
id: options-tree
kind: doc
target_file: src/options.rs
line_hint: 163
author: 
created: 
title: "TREE"
tags: []
signature: "pub const TREE : & [Group] = & [Group { name : \"Preferences\" , items : & [Category :: CommonlyUsed] , open_by_default : true , } , Group { name : \"IDE\" , items : & [Category :: IdeDefaultFolders , Category :: IdeEnvironmentVariables , Category :: IdeFileHistory , Category :: IdeSavingRecovering ,] , open_by_default : true , } , Group { name : \"User Interface\" , items : & [Category :: UiFormDesigner , Category :: UiAppearance] , open_by_default : true , } , Group { name : \"AI Assistant\" , items : & [Category :: AiAcp] , open_by_default : true , } , Group { name : \"Editor\" , items : & [Category :: Editor , Category :: EditorColorScheme] , open_by_default : true , } , Group { name : \"Keyboard\" , items : & [Category :: Keyboard] , open_by_default : false , } ,]"
---

THE ENTIRE TREE — with Commonly Used group on top, modern-editor style.

# Examples

```rust
use crawrad_project::options::TREE;

assert!(!TREE.is_empty());
```
