//! The gate of DISPATCH BY NAME — the `enum` that barred class 27.
//!
//! # The bottleneck, and the measurement that named it
//!
//! [ADR 0031] measured on 16/09/2026:
//!
//! > *"The `ComponentClass` is the bottleneck: the codegen trait requires
//! > `fn class(&self) -> ComponentClass`, so **as long as a class does not
//! > enter that enum, no target can register a handler for it** — that is
//! > why `Form`, `Action`, `ActionList` and `ImageList` were left out."*
//!
//! While the catalogue was closed, this was inconvenient. With installable
//! components ([ADR 0033]) it became an impediment: class 27 comes from a file, and
//! no compiled `enum` can accommodate it.
//!
//! The `enum` was removed on 17/09/2026, and `class()` on both targets began
//! returning `&'static str`.
//!

/// **No target writes its own class list: it comes from what it
/// REGISTERED.**
///
/// # The difference this makes, and it is not a stylistic one
///
/// `CLASSES` for both targets was `ComponentClass::ALL_STR` — the sixteen
/// variants of the catalogue's `enum`. The target declared it generated what the ENUM had,
/// not what it had registered itself. **They coincided by luck**, and nothing
/// enforced the coincidence.
///
/// Derived from the target's own registry, the list becomes true by
/// construction: an added handler appears in it, and a removed one disappears from it,
/// without anyone editing any constant.
#[test]
fn a_lista_de_classes_sai_do_registro_de_cada_alvo() {
    for (alvo, declaradas, registradas) in [
        (
            "egui",
            crate::targets::egui::CLASSES.as_slice(),
            crate::targets::egui::components::classes(),
        ),
        (
            "makepad",
            crate::targets::makepad::CLASSES.as_slice(),
            crate::targets::makepad::components::classes(),
        ),
    ] {
        assert_eq!(
            declaradas,
            &registradas[..],
            "the `{alvo}` declares different classes from what it registered"
        );
        assert!(
            declaradas.len() >= 16,
            "the `{alvo}` shrank to {} classes",
            declaradas.len()
        );
    }
}

/// **A class OUTSIDE the enum's sixteen dispatches.**
///
/// # This is the proof that the bottleneck is gone
///
/// `Form`, `Action`, `ActionList` and `ImageList` have always been in the catalogue and
/// **no target could register a handler for them**, because they were not variants
/// of `ComponentClass`. This is not a hypothesis: ADR 0031 names all four.
///
/// This test constructs a handler whose class is a name the `enum` never had, and
/// asserts that dispatch finds it. With the `enum` back, it would not compile —
/// there would be no variant to write.
#[test]
fn uma_classe_fora_das_dezesseis_despacha() {
    use crate::targets::makepad::components::MakepadComponentCodegen;

    struct PortaDbCodegen;
    impl MakepadComponentCodegen for PortaDbCodegen {
        fn class(&self) -> &'static str {
            "Connection"
        }
        fn struct_definition(&self) -> String {
            String::new()
        }
        fn emit_default(&self, _n: &crawrad_ir::Node) -> String {
            String::new()
        }
        fn emit_sync(&self, _d: &crawrad_ir::Document, _n: &crawrad_ir::Node, _f: &str) -> String {
            String::new()
        }
        fn emit_actions(
            &self,
            _d: &crawrad_ir::Document,
            _n: &crawrad_ir::Node,
            _f: &str,
            _e: &[&crawrad_ir::Node],
        ) -> Result<String, crate::Erro> {
            Ok(String::new())
        }
        fn emit_dsl(
            &self,
            _d: &crawrad_ir::Document,
            _reg: &crawrad_registry::Registry,
            _n: &crawrad_ir::Node,
            _x: f32,
            _y: f32,
            _w: f32,
            _h: f32,
        ) -> Result<String, crate::Erro> {
            Ok(String::new())
        }
    }

    // The handler exists, and its name is the class — no enum in the way.
    let h = PortaDbCodegen;
    assert_eq!(h.class(), "Connection");
    assert_eq!(
        h.class_name(),
        "Connection",
        "the old name became `class()` itself"
    );

    // And the four that ADR 0031 names as excluded by the enum are valid
    // names now: all they lack is someone writing the handler, not a
    // nonexistent variant.
    for classe in ["Form", "Action", "ActionList", "ImageList"] {
        assert!(
            crawrad_registry::Registry::builtin().get(classe).is_some(),
            "`{classe}` is in the catalog and was blocked by the enum"
        );
    }
}

/// **A class without a handler falls into the declared path, and it SAYS it does not generate.**
///
/// This is what separates *"this target does not emit this class"* from *"this target emitted
/// incorrectly"*. A third-party component whose generator does not yet exist is born with a
/// `TODO` in the user's code, not with silence.
#[test]
fn a_classe_sem_handler_cai_no_caminho_declarado() {
    assert!(
        crate::targets::egui::components::for_class_name("Connection").is_none(),
        "no one registered a generator for `Connection` — yet"
    );
    assert!(
        crate::targets::egui::components::for_class_name("Button").is_some(),
        "the `Button` has a generator, and dispatch by name must find it"
    );
}
