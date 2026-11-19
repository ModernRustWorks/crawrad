//! The MANIFEST gate — the standard that every component follows from here on.
//!
//! Two families, and the first is what proves the format:
//!
//! 1. **the roundtrip over the twenty catalogue components** — each one
//!    becomes TOML text and returns, and must come out identical;
//! 2. **the refusals**, one at a time, each with the phrase the IDE will show.

use crawrad_registry::Registry;
use crawrad_registry::external::ExternalComponent;

use crate::manifest::{ComponentEntry, Manifest, ManifestError, PackageId, SDK};

/// A package with the given components, for the test to assemble files.
fn package(components: Vec<ComponentEntry>) -> Manifest {
    Manifest {
        sdk: SDK,
        doc: String::new(),
        package: PackageId {
            name: "portadb-crawrad".to_string(),
            version: "0.1.0".to_string(),
            description: "A driver-agnostic database engine".to_string(),
        },
        components,
    }
}

// ---------------------------------------------------------------------------
// 1 — THE ROUNDTRIP
// ---------------------------------------------------------------------------

/// **Every catalogue component fits in the manifest, and returns whole.**
///
/// # Why this is the test that decides whether the format serves
///
/// It passes through TEXT: the native descriptor becomes `ExternalComponent`, becomes TOML,
/// is read back, and must produce a descriptor identical to where it came from.
///
/// It is proof that the format can express everything a component of this
/// codebase has — **before** someone from the outside writes the first file. A field
/// that the format does not carry appears here as a difference, and not months later
/// as a property that vanished from the Object Inspector of an installed
/// component.
///
/// The oracle is `Registry::builtin()`, so the twenty-first component is
/// charged on the day it is born.
#[test]
fn every_catalogue_component_fits_in_manifest() {
    let reg = Registry::builtin();
    let entries: Vec<ComponentEntry> = reg
        .all()
        .map(|d| ComponentEntry::from_model(&ExternalComponent::from(d)))
        .collect();
    assert!(
        entries.len() >= 16,
        "the catalogue shrank to {} classes; this gate measures nothing",
        entries.len()
    );

    let text = package(entries).to_json();
    let read = Manifest::parse(&text).expect("the manifest we wrote ourselves must parse");

    let returned = read
        .to_components()
        .expect("every catalogue default fits the file format");
    assert_eq!(
        returned.len(),
        reg.all().count(),
        "the number of components changed in roundtrip through the file"
    );
    for (native, roundtrip) in reg.all().zip(returned) {
        // **`crate_module` is the only field that the roundtrip CANNOT return
        // identical, and that is the format being right.**
        //
        // It is not stored in the file: it is DERIVED from `[package] name`,
        // with hyphens becoming underscores, which is what Cargo does with the name
        // of a crate when making it a module path. A handwritten `crate_module`
        // field could disagree with the package carrying it, and then the
        // user's project would output `other_crate::Grid` and not
        // compile.
        //
        // The native one has `""` because its type is EMITTED inside the
        // user's project, not imported from any crate. Passed through a file, it
        // ceases to be native: it begins to live in the package that wrote it.
        //
        // Here the expected value is constructed with this **explicit** derivation,
        // not skipping the field in comparison: skipping would hide the day when
        // the roundtrip brought the wrong module.
        let expected = crawrad_registry::ComponentDesc {
            crate_module: "portadb_crawrad",
            ..native.clone()
        };
        assert_eq!(
            roundtrip.clone().into_catalogue(),
            expected,
            "`{}` did not return completely from the file",
            native.class
        );
    }
}

/// **The generated text is real TOML, and readable.**
///
/// This is not fussiness: the file is what a person opens to understand what was installed,
/// and what `New Package` writes for them to edit.
#[test]
fn generated_text_is_readable() {
    let reg = Registry::builtin();
    let button = reg.get("Button").expect("`Button` is in the catalogue");
    let text = package(vec![ComponentEntry::from_model(&ExternalComponent::from(
        button,
    ))])
    .to_json();

    for piece in [
        "\"sdk\": 2",
        "\"package\"",
        "\"components\"",
        "\"class\": \"Button\"",
    ] {
        assert!(
            text.contains(piece),
            "the manifest does not contain `{piece}`:\n{text}"
        );
    }
}

/// **A component with `inherits` survives round trip through manifest.**
#[test]
fn manifest_roundtrip_with_inherits() {
    let reg = Registry::builtin();
    let spin = reg.get("SpinEdit").expect("`SpinEdit` is in the catalogue");
    let mut entry = ComponentEntry::from_model(&ExternalComponent::from(spin));
    entry.class = "PriceEdit".to_string();
    entry.inherits = Some("SpinEdit".to_string());

    let text = package(vec![entry]).to_json();
    assert!(text.contains("\"inherits\": \"SpinEdit\""));

    let parsed = Manifest::parse(&text).expect("manifest with inherits must parse");
    let components = parsed
        .to_components()
        .expect("converts to external components");
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].class, "PriceEdit");
    assert_eq!(components[0].inherits.as_deref(), Some("SpinEdit"));

    let entry_back = ComponentEntry::from_model(&components[0]);
    assert_eq!(entry_back.inherits.as_deref(), Some("SpinEdit"));
}

/// **A component without `inherits` omits the key in manifest.**
#[test]
fn manifest_roundtrip_without_inherits() {
    let reg = Registry::builtin();
    let button = reg.get("Button").expect("`Button` is in the catalogue");
    let entry = ComponentEntry::from_model(&ExternalComponent::from(button));
    assert!(entry.inherits.is_none());

    let text = package(vec![entry]).to_json();
    assert!(!text.contains("\"inherits\""));

    let parsed = Manifest::parse(&text).expect("manifest without inherits must parse");
    let components = parsed
        .to_components()
        .expect("converts to external components");
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].inherits, None);
}

// ---------------------------------------------------------------------------
// 2 — THE REFUSALS
// ---------------------------------------------------------------------------

/// **The SDK version is checked BEFORE anything else.**
///
/// # Why order matters
///
/// A file of another contract could have fields this IDE would interpret
/// incorrectly, and missing fields. Assembled first and checked later, it would be
/// refused by the **first strange field** — and the phrasing would talk about a field instead
/// of talking about the contract, telling the author to fix the wrong line.
#[test]
fn sdk_version_comes_before_everything() {
    // A file from another contract, and WITHOUT the required fields of this one.
    let e = Manifest::parse(r#"{ "sdk": 99 }"#).unwrap_err();
    assert_eq!(e, ManifestError::WrongSdk { found: 99 });
    // THE PHRASE STATES BOTH VERSIONS and what to do — whoever reads it needs to know whether
    // to update the IDE or switch packages.
    let phrase = e.to_string();
    assert!(
        phrase.contains("99") && phrase.contains(&SDK.to_string()),
        "{phrase}"
    );

    // And the ABSENCE of the field is not "sdk zero", nor an unmatched contract: it is
    // a file that is probably not even ours. See
    // `manifest_of_something_else_is_refused_for_what_it_is`.
    let e = Manifest::parse(r#"{ "package": { "name": "x" } }"#).unwrap_err();
    assert!(matches!(e, ManifestError::NotACrawradManifest), "{e:?}");
}

/// **A package with no components at all is refused.**
///
/// It would install, take up space, and not put a line in the palette — and the
/// user would be left searching for what it brought.
#[test]
fn package_without_components_is_refused() {
    let text = r#"{ "sdk": 2, "package": { "name": "vazio" } }"#;
    assert_eq!(
        Manifest::parse(text).unwrap_err(),
        ManifestError::NoComponent
    );
}

/// **Two classes with the same name in the same package are refused.**
///
/// The second would win silently at registration time, and the author would have
/// no way of knowing which of the two is in the palette.
#[test]
fn duplicate_class_in_same_package_is_refused() {
    let reg = Registry::builtin();
    let button = ComponentEntry::from_model(&ExternalComponent::from(
        reg.get("Button").expect("is in the catalogue"),
    ));
    let text = package(vec![button.clone(), button]).to_json();
    assert_eq!(
        Manifest::parse(&text).unwrap_err(),
        ManifestError::ClassTwice("Button".to_string())
    );
}

/// **A default that is not of the declared kind is refused, and the phrase says both.**
///
/// # The defect this prevents
///
/// `kind` is what the Object Inspector uses to choose the editor, and
/// `default` is what the component is born having. Accepting `kind = "int"` with
/// `default = "abc"` would put a number box over text: the Inspector
/// would draw an editor that cannot read the value that is there.
#[test]
fn default_outside_kind_is_refused() {
    let text = r#"{
      "sdk": 2,
      "package": { "name": "x" },
      "components": [
        {
          "class": "Thing",
          "display": "Thing",
          "palette": "Extras",
          "prefix": "thg",
          "default_size": [10.0, 10.0],
          "props": [
            {
              "key": "tag",
              "display": "Tag",
              "category": "behavior",
              "kind": "int",
              "default": "this is not a number"
            }
          ]
        }
      ]
    }"#;
    let m = Manifest::parse(text).expect("the file is valid JSON and the SDK matches");
    let e = m.to_components().unwrap_err();
    match &e {
        ManifestError::DefaultDoesNotMatchKind { class, key, .. } => {
            assert_eq!(class, "Thing");
            assert_eq!(key, "tag");
        }
        other => panic!("expected default value refusal, got {other:?}"),
    }
    // THE PHRASE STATES THE KIND AND THE VALUE, otherwise whoever wrote it cannot know which of the two
    // to fix.
    let phrase = e.to_string();
    assert!(
        phrase.contains("int") && phrase.contains("this is not a number"),
        "{phrase}"
    );
}

/// **BOTH forms of the default value are read.**
///
/// The short one is normal — the `kind` next to it already states the type. The long one is that of
/// `crawrad_ir::Value` itself, and is the only total one: there are variants no `kind` names.
/// Disambiguation is a single rule, and this test exercises it on both sides.
#[test]
fn both_forms_of_default_are_read() {
    let with_val = |val: &str| {
        format!(
            r#"{{
              "sdk": {sdk},
              "package": {{ "name": "x" }},
              "components": [
                {{
                  "class": "Thing",
                  "display": "Thing",
                  "palette": "Extras",
                  "prefix": "thg",
                  "default_size": [10.0, 10.0],
                  "props": [
                    {{
                      "key": "text",
                      "display": "Text",
                      "category": "appearance",
                      "kind": "text",
                      "default": {val}
                    }}
                  ]
                }}
              ]
            }}"#,
            sdk = SDK
        )
    };

    // THE SHORT ONE.
    let m = Manifest::parse(&with_val(r#""olá""#)).expect("parses");
    let c = m.to_components().expect("converts");
    assert_eq!(
        c[0].props[0].default,
        crawrad_ir::Value::Str("olá".to_string())
    );

    // THE LONG ONE — the same representation that `.crawform` records.
    let m = Manifest::parse(&with_val(r#"{ "t": "Str", "v": "olá" }"#)).expect("parses");
    let c = m.to_components().expect("converts");
    assert_eq!(
        c[0].props[0].default,
        crawrad_ir::Value::Str("olá".to_string())
    );
}

/// **The kind payload lives in the neighboring field, and appears ONCE in the file.**
///
/// # The draft this test buries
///
/// The first version of the format wrote the options list twice — in
/// `options` and inside `kind` —, because the model carries the payload inside the
/// type. Whoever wrote by hand would fill one and forget the other, and the reader
/// would pick one of the two in silence.
#[test]
fn kind_payload_appears_only_once() {
    let reg = Registry::builtin();
    let button = reg.get("Button").expect("is in the catalogue");
    let text = package(vec![ComponentEntry::from_model(&ExternalComponent::from(
        button,
    ))])
    .to_json();

    // The `align` of `Button` is an enum with six options.
    assert!(text.contains("\"kind\": \"enum\""), "{text}");
    assert_eq!(
        text.matches("\"Client\"").count(),
        1,
        "the options list was written more than once:\n{text}"
    );
    // And the type does NOT become its own object — a property is a single object, and
    // `kind` inside it is the NAME of the type, with the payload in neighboring fields.
    let v: serde_json::Value = serde_json::from_str(&text).expect("what we wrote is JSON");
    let prop = v["components"][0]["props"]
        .as_array()
        .expect("props")
        .iter()
        .find(|p| p["kind"] == "enum")
        .expect("the `align` is enum");
    assert!(
        prop["kind"].is_string(),
        "the kind went back to carrying the payload inside itself:\n{prop}"
    );
}

// ---------------------------------------------------------------------------
// 3 — THE FIRST THIRD-PARTY COMPONENT, AND WHAT IT FOUND
// ---------------------------------------------------------------------------

/// The connection package of this codebase: five native providers and the abstraction.
const PORTADB: &str = include_str!("../../../../../components/portadb-crawrad/manifest.json");

/// **A manifest TYPED BY HUMANS is read.**
///
/// This is the test that proves the format is writable by hand: this text did not come from
/// any serializer — it was typed right here. A format that only the serializer itself
/// can produce does not serve what [ADR 0033] wants, which is
/// someone from the outside publishing a component.
///
/// It became like this on 18/09/2026. Before, the oracle was the `manifest.json` of
/// `portadb-crawrad`, which **ceased to be handwritten** when the providers
/// became `struct` with `#[derive(Component)]`. Using a generated file to
/// prove that it can be handwritten would be the guard measuring the opposite of what
/// its doc says.
///
#[test]
fn hand_written_manifest_is_read() {
    let text = format!(
        r#"{{
          "sdk": {sdk},
          "package": {{
            "name": "meu-pacote",
            "version": "0.1.0",
            "description": "escrito na mão, sem ferramenta nenhuma"
          }},
          "components": [
            {{
              "class": "Termometro",
              "display": "Termômetro",
              "palette": "Extras",
              "prefix": "trm",
              "default_size": [120.0, 32.0],
              "visual": false,
              "props": [
                {{
                  "key": "name",
                  "display": "Name",
                  "category": "identity",
                  "kind": "text",
                  "default": ""
                }},
                {{
                  "key": "escala",
                  "display": "Escala",
                  "category": "behavior",
                  "kind": "enum",
                  "default": "Celsius",
                  "options": ["Celsius", "Fahrenheit", "Kelvin"]
                }}
              ],
              "events": [
                {{ "name": "on_change", "display": "OnChange", "signature": "fn(&mut self)" }}
              ]
            }}
          ]
        }}"#,
        sdk = SDK
    );

    let m = Manifest::parse(&text).expect("what a person types must parse");
    assert_eq!(m.package.name, "meu-pacote");
    assert_eq!(m.components.len(), 1);

    let c = &m.components[0];
    assert_eq!(c.class, "Termometro");
    assert!(!c.visual);
    assert_eq!(c.events.len(), 1);

    // The enum brings options ONCE, from the field next to `kind`.
    let scale = c
        .props
        .iter()
        .find(|p| p.key == "escala")
        .expect("`escala` is in the file");
    assert_eq!(scale.kind, crate::manifest::Kind::Enum);
    assert!(scale.options.contains(&"Kelvin".to_string()));
}

/// **The connection package brings the five providers AND the abstraction.**
///
/// It is the design described by the owner on 2026-09-18: the native one exists as
/// a component, the abstraction points to one of them, and both go to the screen.
#[test]
fn connection_package_brings_providers_and_abstraction() {
    let m = Manifest::parse(PORTADB).expect("the package manifest must parse");
    assert_eq!(m.sdk, SDK);
    assert_eq!(m.package.name, "portadb-crawrad");
    assert_eq!(
        m.components.len(),
        7,
        "five providers, the abstraction and the dataset"
    );

    let abstraction = m
        .components
        .iter()
        .find(|c| c.class == "Connection")
        .expect("the abstraction is in the package");
    assert!(
        !abstraction.visual,
        "it is non-visual, like a classic RAD connection"
    );
    assert_eq!(abstraction.events.len(), 2);

    // **The abstraction requests a CAPABILITY, and not a class.**
    let provider = abstraction
        .props
        .iter()
        .find(|p| p.key == "provider")
        .expect("the abstraction has where to receive the provider");
    assert_eq!(provider.kind, crate::manifest::Kind::Reference);
    assert_eq!(provider.ref_class.as_deref(), Some("DbProvider"));

    // And the five offer it.
    let offering = m
        .components
        .iter()
        .filter(|c| c.provides.iter().any(|p| p == "DbProvider"))
        .count();
    assert_eq!(offering, 5, "the five providers offer the capability");

    // **The dataset asks for the CONNECTION the same way, and the abstraction offers it.**
    assert!(abstraction.provides.iter().any(|p| p == "DbConnection"));
    let dataset = m
        .components
        .iter()
        .find(|c| c.class == "Dataset")
        .expect("the dataset is in the package");
    assert!(
        dataset.provides.iter().any(|p| p == "DbDataset"),
        "the dataset must provide DbDataset capability"
    );
    let connection = dataset
        .props
        .iter()
        .find(|p| p.key == "connection")
        .expect("the dataset has where to receive the connection");
    assert_eq!(connection.kind, crate::manifest::Kind::Reference);
    assert_eq!(connection.ref_class.as_deref(), Some("DbConnection"));
}

/// **And it ENTERS — the third-party component brings what belongs to it.**
///
/// # This test previously asserted the opposite, and the date matters
///
/// It used to be named `o_componente_de_terceiro_ainda_nao_tem_onde_por_o_que_e_dele`
/// and demanded the REJECTION of `Connection`, with the name of the first key that hit
/// the wall — `url`. Its doc stated, in so many words: *"on the day the drawer
/// exists, it turns red and asks to be rewritten the opposite way"*.
///
/// That is what happened, on 2026-09-17, through [Amendment 1 of ADR 0031].
///
/// # The gap, and what closed it
///
/// The contract of [ADR 0002] had two drawers: the common core (**every**
/// runtime honors) and the space of a target (**that specific** runtime honors).
/// `Connection.url` was neither — **no runtime honors it**,
/// the COMPONENT is what honors it.
///
/// The way out was not opening a third drawer: it was recognizing that the first **should
/// not be a gatekeeper**. In the words of the owner: *"if we are going to focus on
/// the component, why do we need this common one? That is already duplication"*. The
/// list of forty names was the second copy of what the component already declares.
///
/// What judges the key without a prefix became the CLASS — and it became stronger,
/// not weaker: it catches what the global list never caught, a `Button` with the
/// `interval` property of `Timer`.
///
#[test]
fn third_party_component_brings_what_belongs_to_it() {
    let m = Manifest::parse(PORTADB).expect("reads");
    let components = m.to_components().expect("file format is correct");

    let mut catalogue = Registry::builtin();
    for c in components {
        let class_name = c.class.clone();
        catalogue
            .register_external(c)
            .unwrap_or_else(|e| panic!("`{class_name}` was rejected: {e}"));
    }

    let d = catalogue.get("Connection").expect("is in catalogue");
    for key in [
        "url",
        "provider",
        "max_connections",
        "connect_timeout",
        "connected",
    ] {
        assert!(
            d.prop(key).is_some(),
            "`Connection.{key}` belongs to the component and should be there"
        );
    }

    // AND WHAT IT DECLARES IS ITS OWN, and not everybody's: a `Button` with `url`
    // continues to be rejected, because the question belongs to the CLASS.
    let mut doc = crawrad_ir::Document::new("form_main", "Form");
    let root = doc.root.clone();
    let mut button = crawrad_ir::Node::new("btn_ok", "Button");
    button
        .props
        .insert("url".into(), crawrad_ir::Value::Str("x".into()));
    doc.nodes.insert("btn_ok".into(), button);
    doc.node_mut(&root)
        .expect("root")
        .children
        .push("btn_ok".into());
    assert!(
        crawrad_registry::class_property::check(&doc, &catalogue).is_err(),
        "`url` belongs to `Connection`, and not to `Button`"
    );
}

/// **And `Connection` appears on the palette, in the group it declared.**
///
/// Registering without appearing would be halfway: the component would be in the catalogue
/// and nobody would be able to drag it.
#[test]
fn portadb_appears_on_palette() {
    let m = Manifest::parse(PORTADB).expect("reads");
    let mut catalogue = Registry::builtin();
    for c in m.to_components().expect("converts") {
        catalogue.register_external(c).expect("registers");
    }
    let palette = catalogue.palette("egui");
    let group = palette
        .iter()
        .find(|(name, _)| *name == "Data")
        .expect("group `Data` was created with the component");
    assert!(group.1.iter().any(|d| d.class == "Connection"));
}

// ---------------------------------------------------------------------------
// 4 — THE ENGINE COMPONENTS, WHICH PASS
// ---------------------------------------------------------------------------

/// The components of the connection package — the FIVE providers and the abstraction.
///
/// Until 2026-09-18 this was a handwritten `providers.json`, next to the
/// package manifest, with five components that **the IDE never opened**: it reads
/// one manifest per package. They had no Rust type behind them.
///
/// Now they do, and the fixture disappeared with the duplicate: what this test measures is the
/// file that `#[derive(Component)]` writes, which is what the user installs.
const ENGINES: &str = PORTADB;

/// **The provider components ENTER the catalogue — all five.**
///
/// # Why they are the ones proving the door end-to-end, and not `Connection`
///
/// A provider component declares only `name` and `rect`, which belong to the common
/// core. It does not hit the missing drawer — the one for the property that belongs to the
/// COMPONENT —, and that is why it is the first third-party component traversing the
/// entire door: manifest read, validated, and registered in a catalogue alongside
/// the twenty native ones.
#[test]
fn provider_components_enter_catalogue() {
    let m = Manifest::parse(ENGINES).expect("the providers file must parse");
    let components = m.to_components().expect("format is correct");
    assert_eq!(
        components.len(),
        7,
        "five providers, the abstraction and the dataset"
    );

    let mut catalogue = Registry::builtin();
    let before = catalogue.all().count();
    for c in components {
        let class_name = c.class.clone();
        catalogue
            .register_external(c)
            .unwrap_or_else(|e| panic!("`{class_name}` was rejected: {e}"));
    }
    assert_eq!(
        catalogue.all().count(),
        before + 7,
        "the seven had to be in the catalogue"
    );

    // And they appear on the PALETTE, in the group they declared.
    let palette = catalogue.palette("egui");
    let group = palette
        .iter()
        .find(|(name, _)| *name == "Data")
        .expect("group `Data` was created with the providers");
    assert_eq!(group.1.len(), 7);
}

/// **Each provider enables ITS feature, and that is what it exists to do.**
///
/// # The SQLite *driver link*, measured
///
/// Without `portadb/rusqlite` the driver module **is not even compiled** — the
/// `drivers/mod.rs` of the crate is entirely `#[cfg(feature = …)]`. So the requirement
/// carried by this component is not metadata: it is the difference between the driver
/// existing or not existing in the user binary.
///
/// The oracle is the `Cargo.toml` of `portadb` itself: the feature requested by a
/// component must be one that the crate declares. An invented feature
/// would cause the `cargo build` of the generated project to fail with an error about a crate
/// the user never wrote.
#[test]
fn each_provider_enables_the_feature_declared_by_crate() {
    const CARGO: &str = include_str!("../../../../../components/portadb/Cargo.toml");
    let declared: Vec<&str> = CARGO
        .lines()
        .skip_while(|l| l.trim() != "[features]")
        .skip(1)
        .take_while(|l| !l.trim_start().starts_with('['))
        .filter_map(|l| l.split_once('=').map(|(n, _)| n.trim()))
        .filter(|n| !n.is_empty())
        .collect();
    assert!(
        declared.contains(&"rusqlite"),
        "`Cargo.toml` reading format changed: {declared:?}"
    );

    let m = Manifest::parse(ENGINES).expect("reads");
    let mut with_feature = 0;
    let mut with_requirement = 0;
    for c in m.to_components().expect("converts") {
        // **THE ABSTRACTION HAS NO REQUIREMENT, and this is the design, not a deficiency.**
        //
        // What enables the driver is the ENGINE -- it is the SQLite *driver link*. The
        // abstraction only points to one of them, and forcing it to declare a crate
        // would cause the user project to link a driver nobody chose.
        let Some(r) = c.requires.first() else {
            continue;
        };
        with_requirement += 1;
        assert_eq!(r.crate_name, "portadb");
        for f in &r.features {
            assert!(
                declared.contains(&f.as_str()),
                "`{}` requests feature `{f}`, which `portadb` does not declare: {declared:?}",
                c.class
            );
            with_feature += 1;
        }
    }
    assert_eq!(
        with_requirement, 6,
        "the five providers and the dataset declare the engine crate; the abstraction declares \
         none, and that is how it should be"
    );
    assert!(
        with_feature >= 4,
        "only {with_feature} providers enable a feature; `redis` is the only one that does \
         not enable one, because it is not gated in today's crate"
    );
}

/// **Text that is not JSON returns the parser's complaint, not a panic.**
#[test]
fn non_json_text_returns_complaint() {
    let e = Manifest::parse("this is not json {{{").unwrap_err();
    assert!(matches!(e, ManifestError::NotJson(_)), "{e:?}");
}

/// **A `manifest.json` of SOMETHING ELSE is rejected for what it is.**
///
/// The file name became standard `manifest.json` on 2026-09-18, at the owner's
/// request. The price of this standardization is this: a crate can have a
/// `manifest.json` of something else at its root, and the IDE will open it.
///
/// Without this rejection, the complaint would be about the VERSION — *"this package speaks SDK 0
/// and this IDE speaks 2"* —, and whoever read it would search for an update that does not
/// exist. A guard measuring the wrong thing is worse than none.
#[test]
fn manifest_of_something_else_is_rejected_for_what_it_is() {
    let e = Manifest::parse(r#"{ "name": "meu-app", "icons": [] }"#).unwrap_err();
    assert!(
        matches!(e, ManifestError::NotACrawradManifest),
        "expected rejection for NOT BEING our manifest, and got: {e:?}"
    );
    let said = e.to_string();
    assert!(
        said.contains("CrawRAD"),
        "the phrase must say whose file it is not: {said}"
    );
}

/// **The manifest does NOT register anything on its own.**
///
/// Loading and registering are two distinct actions, and the store needs the first without the
/// second: it shows what a package brings **before** installing. What decides
/// if a component enters is `Registry::register_external`, with its seven
/// rejections.
#[test]
fn loading_is_not_registering() {
    let reg = Registry::builtin();
    let button = ComponentEntry::from_model(&ExternalComponent::from(
        reg.get("Button").expect("is in catalogue"),
    ));
    let text = package(vec![button]).to_json();
    let m = Manifest::parse(&text).expect("reads");

    // The manifest delivers the components, and the catalogue remains the one that
    // rejects: this `Button` collides with the native one.
    let mut catalogue = Registry::builtin();
    let incoming = m.to_components().expect("reads");
    assert_eq!(incoming.len(), 1);
    let e = catalogue
        .register_external(incoming.into_iter().next().expect("one"))
        .unwrap_err();
    assert!(
        matches!(
            e,
            crawrad_registry::registration::RegistrationError::ClassTaken(_)
        ),
        "{e:?}"
    );
}

/// **Typed collection survives the round trip through the file.**
///
/// ADR 0037 establishes that a typed collection (`Collection<T>`) declares
/// `item_class` in the manifest, so the Inspector knows what item type to edit.
#[test]
fn typed_collection_roundtrips_through_file() {
    use crawrad_ir::Value;
    use crawrad_registry::Category;
    use crawrad_registry::external::{ExternalChildren, ExternalKind, ExternalProp};

    let col_prop = ExternalProp {
        key: "columns".to_string(),
        display: "Columns".to_string(),
        default: Value::List(Vec::new()),
        category: Category::Appearance,
        kind: ExternalKind::Collection {
            hierarchical: false,
            item_class: Some("GridColumn".to_string()),
        },
        readonly: false,
        bindable: false,
        options: Vec::new(),
        ref_class: None,
        tree_collection: false,
        item_class: Some("GridColumn".to_string()),
        multiline: false,
        hidden_when: None,
    };

    let comp = ExternalComponent {
        inherits: None,
        class: "CustomGrid".to_string(),
        crate_module: "custom_grid".to_string(),
        desenho: Vec::new(),
        icon: Vec::new(),
        display: "Custom Grid".to_string(),
        base_name: "custom_grid".to_string(),
        palette: "Grids".to_string(),
        prefix: "grd".to_string(),
        default_size: (200.0, 150.0),
        props: vec![col_prop],
        events: Vec::new(),
        targets: vec!["makepad".to_string()],
        children: ExternalChildren::None,
        derived_geometry: false,
        visual: true,
        emits_code: true,
        private_state: false,
        provides: Vec::new(),
        text_line: None,
        requires: Vec::new(),
    };

    let entry = ComponentEntry::from_model(&comp);
    let text = package(vec![entry]).to_json();
    assert!(
        text.contains(r#""item_class": "GridColumn""#),
        "the JSON must contain the item_class key: {text}"
    );

    let parsed = Manifest::parse(&text).expect("parses manifest");
    let components = parsed.to_components().expect("converts to components");
    assert_eq!(components.len(), 1);

    let roundtripped = &components[0];
    let roundtripped_prop = &roundtripped.props[0];
    assert_eq!(
        roundtripped_prop.kind,
        ExternalKind::Collection {
            hierarchical: false,
            item_class: Some("GridColumn".to_string()),
        }
    );
    assert_eq!(roundtripped_prop.item_class.as_deref(), Some("GridColumn"));

    let cat_desc = roundtripped.clone().into_catalogue();
    let cat_prop = &cat_desc.props[0];
    assert_eq!(
        cat_prop.kind,
        crawrad_registry::PropKind::Collection {
            hierarchical: false,
            item_class: Some("GridColumn"),
        }
    );
    assert_eq!(cat_prop.item_class, Some("GridColumn"));
}

// ---------------------------------------------------------------------------
// 3 — THE PALETTE ICON
// ---------------------------------------------------------------------------

#[test]
fn manifest_with_icon_array_parses_into_steps() {
    use crawrad_ir::icon::IconStep;

    let json = r#"{
        "sdk": 2,
        "doc": "",
        "package": { "name": "icon-pkg", "version": "0.1.0", "description": "Icon test" },
        "components": [{
            "class": "IconicButton",
            "display": "Iconic Button",
            "palette": "Buttons",
            "prefix": "ib",
            "default_size": [100.0, 30.0],
            "icon": [
                { "step": "move_to", "x": 2.0, "y": 2.0 },
                { "step": "line_to", "x": 14.0, "y": 14.0 },
                { "step": "stroke" }
            ]
        }]
    }"#;

    let manifest = Manifest::parse(json).expect("manifest with icon parses");
    let comps = manifest
        .to_components()
        .expect("components conversion succeeds");
    assert_eq!(comps.len(), 1);
    assert_eq!(
        comps[0].icon,
        vec![
            IconStep::MoveTo(2.0, 2.0),
            IconStep::LineTo(14.0, 14.0),
            IconStep::Stroke,
        ]
    );
}

#[test]
fn icon_roundtrips_through_from_model_serialize_and_parse() {
    use crawrad_ir::icon::IconStep;
    use crawrad_registry::external::ExternalChildren;

    let comp = ExternalComponent {
        inherits: None,
        class: "CircleBadge".to_string(),
        crate_module: "circle_badge".to_string(),
        desenho: Vec::new(),
        icon: vec![
            IconStep::Circle {
                x: 8.0,
                y: 8.0,
                r: 6.0,
            },
            IconStep::Fill,
        ],
        display: "Circle Badge".to_string(),
        base_name: "circle_badge".to_string(),
        palette: "Badges".to_string(),
        prefix: "cb".to_string(),
        default_size: (50.0, 50.0),
        props: Vec::new(),
        events: Vec::new(),
        targets: vec!["makepad".to_string()],
        children: ExternalChildren::None,
        derived_geometry: false,
        visual: true,
        emits_code: true,
        private_state: false,
        provides: Vec::new(),
        text_line: None,
        requires: Vec::new(),
    };

    let entry = ComponentEntry::from_model(&comp);
    let text = package(vec![entry]).to_json();
    assert!(
        text.contains(r#""icon""#),
        "the JSON must contain the icon array: {text}"
    );

    let parsed = Manifest::parse(&text).expect("parses manifest with icon");
    let components = parsed.to_components().expect("converts to components");
    assert_eq!(components.len(), 1);
    assert_eq!(components[0].icon, comp.icon);
}

#[test]
fn unknown_icon_step_is_refused_naming_class() {
    let json = r#"{
        "sdk": 2,
        "doc": "",
        "package": { "name": "bad-icon", "version": "0.1.0", "description": "Bad icon" },
        "components": [{
            "class": "BadStepWidget",
            "display": "Bad Step",
            "palette": "Test",
            "prefix": "bsw",
            "default_size": [100.0, 30.0],
            "icon": [
                { "step": "polygon", "x": 2.0, "y": 2.0 }
            ]
        }]
    }"#;

    let manifest = Manifest::parse(json).expect("parses JSON");
    let err = manifest.to_components().unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("BadStepWidget"),
        "error must name class BadStepWidget: {msg}"
    );
    assert!(
        msg.contains("polygon"),
        "error must name unknown step polygon: {msg}"
    );
    assert!(
        matches!(err, ManifestError::UnknownIconStep { ref class, ref found } if class == "BadStepWidget" && found == "polygon")
    );
}

#[test]
fn icon_step_missing_field_is_refused_naming_class() {
    let json = r#"{
        "sdk": 2,
        "doc": "",
        "package": { "name": "bad-icon", "version": "0.1.0", "description": "Bad icon" },
        "components": [{
            "class": "IncompleteMove",
            "display": "Incomplete Move",
            "palette": "Test",
            "prefix": "im",
            "default_size": [100.0, 30.0],
            "icon": [
                { "step": "move_to", "x": 2.0 }
            ]
        }]
    }"#;

    let manifest = Manifest::parse(json).expect("parses JSON");
    let err = manifest.to_components().unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("IncompleteMove"),
        "error must name class IncompleteMove: {msg}"
    );
    assert!(
        msg.contains("move_to"),
        "error must name step move_to: {msg}"
    );
    assert!(msg.contains('y'), "error must name missing field y: {msg}");
    assert!(
        matches!(err, ManifestError::IconStepMissingField { ref class, ref step, ref field } if class == "IncompleteMove" && step == "move_to" && field == "y")
    );
}

#[test]
fn icon_validation_failure_is_refused_naming_class() {
    use crawrad_ir::icon::IconError;

    let json = r#"{
        "sdk": 2,
        "doc": "",
        "package": { "name": "bad-icon", "version": "0.1.0", "description": "Bad icon" },
        "components": [{
            "class": "UnpaintedWidget",
            "display": "Unpainted",
            "palette": "Test",
            "prefix": "uw",
            "default_size": [100.0, 30.0],
            "icon": [
                { "step": "move_to", "x": 2.0, "y": 2.0 },
                { "step": "line_to", "x": 14.0, "y": 14.0 }
            ]
        }]
    }"#;

    let manifest = Manifest::parse(json).expect("parses JSON");
    let err = manifest.to_components().unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("UnpaintedWidget"),
        "error must name class UnpaintedWidget: {msg}"
    );
    assert!(
        matches!(err, ManifestError::InvalidIcon { ref class, error: IconError::NothingPainted } if class == "UnpaintedWidget")
    );
}

/// A step that takes no number refuses one: `close_path` with an `x` is a
/// typo in the manifest, and reading it as a plain `close_path` would hide it.
#[test]
fn icon_step_with_an_extra_field_is_refused_naming_class() {
    let json = r#"{
        "sdk": 2,
        "doc": "",
        "package": { "name": "bad-icon", "version": "0.1.0", "description": "Bad icon" },
        "components": [{
            "class": "ExtraClose",
            "display": "Extra Close",
            "palette": "Test",
            "prefix": "ec",
            "default_size": [100.0, 30.0],
            "icon": [
                { "step": "move_to", "x": 2.0, "y": 2.0 },
                { "step": "line_to", "x": 8.0, "y": 2.0 },
                { "step": "close_path", "x": 1.0 },
                { "step": "fill" }
            ]
        }]
    }"#;
    let manifest = Manifest::parse(json).expect("parses JSON");
    let err = manifest.to_components().unwrap_err();
    assert!(
        matches!(err, ManifestError::IconStepUnexpectedField { ref class, ref step, ref field } if class == "ExtraClose" && step == "close_path" && field == "x"),
        "expected the extra `x` of close_path to be refused, got: {err}"
    );
}
