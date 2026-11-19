---
id: icons-classicon-all
kind: doc
target_file: src/icons.rs
line_hint: 428
author: 
created: 
title: "ClassIcon::ALL"
tags: []
signature: "pub const ALL : & 'static [Self] = & [Self :: Action , Self :: ActionList , Self :: Button , Self :: CheckBox , Self :: ComboBox , Self :: Edit , Self :: Form , Self :: GroupBox , Self :: ImageList , Self :: Label , Self :: ListBox , Self :: Memo , Self :: PageControl , Self :: Panel , Self :: RadioButton , Self :: ScrollBar , Self :: ScrollBox , Self :: TabSheet , Self :: Timer , Self :: TreeView , Self :: Image , Self :: Grid , Self :: BitBtn , Self :: SpeedButton , Self :: MainMenu , Self :: PopupMenu , Self :: ToolBar , Self :: StatusBar , Self :: ProgressBar , Self :: TrackBar , Self :: PaintBox , Self :: Shape , Self :: OpenDialog , Self :: SaveDialog , Self :: Splitter , Self :: RadioGroup , Self :: SpinEdit , Self :: Toggle , Self :: ColorBox , Self :: LinkLabel , Self :: Bevel , Self :: CategoryPanel , Self :: ListView , Self :: MessageDialog , Self :: Card , Self :: Badge , Self :: Chips , Self :: Breadcrumb , Self :: Pagination , Self :: Drawer , Self :: MultiSelect , Self :: CheckListBox , Self :: MaskEdit , Self :: DateTimePicker ,]"
---

All of them, in order of kind.

Two of them are NOT in the egui set, and both for the same reason: the
Structure tree shows nodes the Toolbox never offers. `Form` is the root
of the document — nobody drops a form onto a form — and `Action` is a
child of an `ActionList`, created by its editor and not by the palette.
Without them the tree of the reference form had seven rows with a hole
where the icon goes.

# Examples

```rust
use crawrad_project::icons::ClassIcon;

assert!(!ClassIcon::ALL.is_empty());
```
