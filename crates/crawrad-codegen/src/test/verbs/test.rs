//! **Do the four verbs reproduce what sixteen handwritten handlers
//! produce?**
//!
//! This is the falsification of ADR 0036.
//! The ADR claims that `create`/`set`/`child`/`bind` suffice; this measurement compares,
//! class by class, the DERIVED state from the descriptor with the state that the class handler
//! writes today.
//!
//! # The comparison is by SET, not by text
//!
//! Byte-for-byte would be the wrong yardstick and answer the wrong question. The order of
//! fields in a `struct` with named initialization is not semantic: the handwritten
//! handler places `visible`/`enabled` first because that was how it
//! grew; the descriptor provides them in base order. Enforcing field order would cause the
//! gate to fail over a difference that changes nothing in the application — and a
//! gate that flags what is correct gets turned off the third time.
//!
//! What matters, and what is enforced: **the same set of fields, with the
//! same types.**
//!
//! # What this file does NOT claim
//!
//! It measures the **declarative** half — `Create` and `Set`. `Child` and
//! `Bind` have their own files when they exist. A green result here does not mean the
//! entire format is complete: it means the part the ADR called declarative is
//! indeed derivable, for the twenty classes in the catalogue.

use crawrad_registry::Registry;
use std::collections::BTreeMap;

use crate::targets::makepad::components;
use crate::targets::makepad::state;

/// The fields declared by the handwritten handler, read from the text it emits.
fn campos_escritos_a_mao(classe: &str) -> Option<BTreeMap<String, String>> {
    let h = components::for_class(classe)?;
    let texto = h.struct_definition();
    let needle = format!("struct {classe}");
    let pos_classe = texto.find(&needle)?;
    let texto_classe = &texto[pos_classe..];
    let abre = texto_classe.find('{')? + pos_classe;
    let fecha = texto[abre..].find('}')? + abre;
    let mut out = BTreeMap::new();
    for l in texto[abre + 1..fecha].lines() {
        let l = l.trim();
        let Some(resto) = l.strip_prefix("pub ") else {
            continue;
        };
        let Some((nome, tipo)) = resto.split_once(": ") else {
            continue;
        };
        out.insert(
            nome.trim().to_string(),
            tipo.trim().trim_end_matches(',').to_string(),
        );
    }
    Some(out)
}

/// The fields derived by the verbs from the descriptor.
fn campos_derivados(classe: &str) -> BTreeMap<String, String> {
    let r = Registry::builtin();
    let desc = r.get(classe).expect("the class is in the catalogue");
    state::fields_of(desc).into_iter().collect()
}

/// **The measurement.** For each class with a handler, what the derived version has in addition and
/// what is missing.
fn diferencas(classe: &str) -> (Vec<String>, Vec<String>) {
    let mao = campos_escritos_a_mao(classe).unwrap_or_default();
    let der = campos_derivados(classe);
    let so_no_derivado: Vec<String> = der
        .iter()
        .filter(|(k, v)| mao.get(*k) != Some(*v))
        .map(|(k, v)| format!("{k}: {v}"))
        .collect();
    let so_na_mao: Vec<String> = mao
        .iter()
        .filter(|(k, v)| der.get(*k) != Some(*v))
        .map(|(k, v)| format!("{k}: {v}"))
        .collect();
    (so_no_derivado, so_na_mao)
}

/// **THE PROOF, in a single sentence: the verbs lose NOTHING.**
///
/// For each of the sixteen classes with a handler, the set of fields declared by the
/// handwritten handler is ENTIRELY contained in what the four verbs derive from the
/// descriptor. No class has a field that the format cannot reach.
///
/// # Why the converse is not enforced, and why that is not looseness
///
/// The derived version has ADDITIONAL fields, and they are not bugs — they are the debt of the
/// handwritten side, measured in the report below. `hint` in all sixteen, `tab_order`
/// in ten, `action` on `Button`, `kind` on `ScrollBar`: properties that the
/// user edits in the Object Inspector and that the generated application discards because
/// someone wrote them on a list of names to skip.
///
/// Demanding exact equality would force **copying the debt into the
/// format** just to turn the gate green. That would be the gate demanding degraded
/// code.
#[test]
fn os_verbos_nao_perdem_nenhum_campo() {
    let mut perdidos = Vec::new();
    for classe in components::classes() {
        let (_, a_menos) = diferencas(classe);
        if !a_menos.is_empty() {
            perdidos.push(format!("{classe}: {}", a_menos.join(", ")));
        }
    }
    assert!(
        perdidos.is_empty(),
        "FIELD DECLARED BY HANDLER BUT NOT REACHED BY VERBS:\n  {}\n\n\
         Either it comes from a property missing in the descriptor — which means \
         target state, declared in `state::EXTRA_FIELDS` —, or the name/type \
         diverges and a line is missing in `state::FIELD_MAP`. Both valves are \
         DATA; neither is a per-class handler.",
        perdidos.join("\n  ")
    );
}

/// **THE DEBT that the verbs expose** — and it can only shrink.
///
/// Each pair here is a catalogue property discarded by the handwritten
/// handler. It is not an excused exception: it is the record of what remains to be emitted, and the
/// gate checks the EXACT set, so no one can add to it silently.
const DIVIDA_DO_ESCRITO_A_MAO: &[(&str, &str)] = &[
    ("Button", "action"),
    ("Memo", "scroll_bars"),
    // The linked Action is resolved by the generator, as a Button's
    ("MenuItem", "action"),
    ("ScrollBar", "kind"),
    // The generator resolves the icons into each node's symbol
    ("TreeView", "images"),
];

#[test]
fn a_divida_do_escrito_a_mao_so_encolhe() {
    let mut medida: Vec<(String, String)> = Vec::new();
    for classe in components::classes() {
        let (a_mais, _) = diferencas(classe);
        for campo in a_mais {
            let nome = campo.split(':').next().unwrap_or(&campo).trim().to_string();
            medida.push((classe.to_string(), nome));
        }
    }
    medida.sort();
    let registro: Vec<(String, String)> = DIVIDA_DO_ESCRITO_A_MAO
        .iter()
        .map(|(c, p)| (c.to_string(), p.to_string()))
        .collect();
    assert_eq!(
        medida, registro,
        "handwritten side debt changed. NEW discarded property \
         fails; property now emitted fails until the line is removed \
         from `DIVIDA_DO_ESCRITO_A_MAO`. Seven today, zero when each handler \
         yields to the verbs."
    );
}

/// **THE REPORT.** It does not fail: it SHOWS, and this is how the valve
/// size is discovered rather than guessed.
///
/// Failing here today would create a chronically red gate, and the project has already measured
/// the cost of that — it loses its value. What causes failure are the two above:
/// [`os_verbos_nao_perdem_nenhum_campo`], which is the proof, and
/// [`a_divida_do_escrito_a_mao_so_encolhe`], which is the ratchet.
#[test]
fn quanto_os_verbos_ja_cobrem() {
    let mut linhas = Vec::new();
    let mut iguais = 0usize;
    for classe in components::classes() {
        let (a_mais, a_menos) = diferencas(classe);
        if a_mais.is_empty() && a_menos.is_empty() {
            iguais += 1;
            linhas.push(format!("{classe:<14} EQUAL"));
        } else {
            linhas.push(format!(
                "{classe:<14} derived only: [{}] | handwritten only: [{}]",
                a_mais.join(", "),
                a_menos.join(", ")
            ));
        }
    }
    println!(
        "\n=== THE FOUR VERBS, declarative half ===\n{}\n{iguais} of {} classes identical\n",
        linhas.join("\n"),
        components::classes().len()
    );
    assert!(iguais > 0, "no classes matched — derivation is broken");
}

#[test]
fn a_target_prefixed_key_is_target_only() {
    use crate::emission::{NotSet, why_not_set};
    use crawrad_registry::Category;

    assert_eq!(
        why_not_set("makepad.shadow", Category::Appearance),
        Some(NotSet::TargetOnly)
    );
    assert_eq!(
        why_not_set("egui.rounding", Category::Appearance),
        Some(NotSet::TargetOnly)
    );
    assert_eq!(why_not_set("text", Category::Appearance), None);
}

#[test]
fn emissions_skip_a_target_prefixed_property() {
    use crate::emission::{Emission, emissions_of};
    use crawrad_ir::{Document, Node, Value};
    use crawrad_registry::{Category, PropDesc, Registry};

    let r = Registry::builtin();
    let mut desc = r.get("Button").expect("Button is built in").clone();
    desc.props.push(PropDesc::new(
        "makepad.shadow",
        "Shadow",
        Value::Bool(false),
        Category::Appearance,
    ));

    let mut doc = Document::new("form_main", "Form");
    let mut node = Node::new("btn1", "Button");
    node.props
        .insert("makepad.shadow".to_string(), Value::Bool(true));
    node.props
        .insert("text".to_string(), Value::Str("OK".to_string()));
    doc.add_child("form_main", node.clone()).expect("add child");

    let emissions = emissions_of(&doc, &node, &desc);
    assert!(
        !emissions
            .iter()
            .any(|e| matches!(e, Emission::Set { key, .. } if key == "makepad.shadow")),
        "target-prefixed property must not produce Emission::Set: {emissions:?}"
    );
    assert!(
        emissions
            .iter()
            .any(|e| matches!(e, Emission::Set { key, .. } if key == "text")),
        "common property must produce Emission::Set: {emissions:?}"
    );
}
