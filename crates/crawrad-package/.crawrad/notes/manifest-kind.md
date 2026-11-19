---
id: manifest-kind
kind: doc
target_file: src/manifest.rs
line_hint: 791
author: 
created: 
title: "Kind"
tags: []
signature: "pub enum Kind"
---

What the property IS, in the file format.

# Here it is JUST THE NAME, and the payload lives in the field next to it

The model ([`ExternalKind`]) carries the payload inside the type — `Enum(options)`,
`Reference(class)` —, and must carry it: the catalogue's `PropKind` carries it, and
losing that would lose information the model has.

**In the file that would be the same list written twice.** The first
draft of this format produced exactly that:

```toml
options = ["None", "Top", "Bottom"]

[component.prop.kind]
enum = ["None", "Top", "Bottom"]
```

Whoever wrote by hand would fill one and forget the other, and the reader
would pick one of the two in silence. The format is written by humans; it
must have each thing **once**.

So here `kind` is the name, and the payload comes from [`PropEntry::options`],
[`PropEntry::ref_class`], [`PropEntry::min`], [`PropEntry::max`],
[`PropEntry::step`] and [`PropEntry::tree_collection`] — fields that the model
**already has**, and that the catalogue keeps up to date by construction
(`PropDesc::options`, `PropDesc::refers_to`, `PropDesc::hierarchical`).

**The limit, declared:** the format cannot express a property
whose type payload disagrees with the neighboring field. The catalogue also cannot
produce one, and the conversion refuses instead of picking one of the two.

# Examples

```rust
use crawrad_package::manifest::Kind;

let k = Kind::Text;
assert_eq!(k, Kind::Text);
```
