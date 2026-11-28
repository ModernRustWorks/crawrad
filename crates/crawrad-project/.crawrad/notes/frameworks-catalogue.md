---
id: frameworks-catalogue
kind: doc
target_file: src/frameworks.rs
line_hint: 36
author: 
created: 
title: "CATALOGUE"
tags: []
signature: "pub const CATALOGUE : & [Framework] = & [Framework { name : \"Makepad\" , target : Some (AlvoGerador :: Makepad) , summary : \"GPU-drawn, live-reload of the design, shaders and effects.\" , platforms : \"⊞ Windows   🐧 Linux   🍎 macOS   🌐 Web   📱 Mobile\" , crate_dep : \"makepad-widgets\" , availability : Availability :: Ready , web : true , mobile : true , } , Framework { name : \"egui\" , target : Some (AlvoGerador :: Egui) , summary : \"Immediate mode, small and quick to build.\" , platforms : \"⊞ Windows   🐧 Linux   🍎 macOS   🌐 Web\" , crate_dep : \"eframe\" , availability : Availability :: Ready , web : true , mobile : false , } , Framework { name : \"Slint\" , target : Some (AlvoGerador :: Slint) , summary : \"Declarative markup, one description of the form, native look.\" , platforms : \"⊞ Windows   🐧 Linux   🍎 macOS\" , crate_dep : \"slint\" , availability : Availability :: Ready , web : false , mobile : false , } ,]"
---

Every framework, in the order the dialog lists them.

# Examples

```rust
use crawrad_project::frameworks::CATALOGUE;

assert!(!CATALOGUE.is_empty());
```
