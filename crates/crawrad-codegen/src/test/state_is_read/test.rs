//! **Every field of a component's state is read by the drawing — in EVERY target.**
//!
//! This is the ADR 0010 promise stated as an arithmetic fact: a component is a
//! field of its own type, `visible`/`enabled`/caption are **state**, and state
//! that nothing reads is a property the user edits in the Object Inspector
//! while the application ignores it.
//!
//! # Why this file exists, and what it costs
//!
//! Until 16/09/2026 the only thing measuring this was `rustc` itself, through
//! `dead_code` — and the generated designer carried a `#![allow(dead_code)]`
//! at the top, so it measured nothing. Removing that attribute uncovered
//! **seven** properties in this exact state: `Panel.enabled`,
//! `GroupBox.enabled`, `PageControl.enabled`, `ScrollBox.enabled` and
//! `auto_scroll`, `ScrollBar.small_change` and `large_change`, plus the
//! captions of `CheckBox` and `RadioButton`, which were literals pasted into
//! the drawing instead of reads of the field beside them.
//!
//! With the attribute gone, `rustc` is the net again — but it only fires in
//! `gate_sem_avisos`, which is `#[ignore]` and calls `cargo` from inside
//! `cargo`. **A net you cannot run in `cargo test` is a net nobody runs.**
//!
//! So this file asks the same question of the **emitted text**, which costs
//! microseconds and no `cargo`: for every component on the form and every
//! field of its state type, does the drawing mention that field?
//!
//! # It measured ONE target, and that was the hole
//!
//! Until 18/09/2026 it called `GeradorRxp` and nothing else. The egui arm was
//! held to the promise; **the Makepad arm — the one the IDE actually ships
//! (ADR 0032) —
//! was not measured at all.**
//!
//! What was hiding there: **`enabled` is dead state in every generated Makepad
//! application.** The value is baked into the `live_design!` text as a literal
//! at generation time, and the field beside it is never read — so
//! `self.form.btn.enabled = false` in a running application does nothing. The
//! egui arm reads it in all sixteen classes.
//!
//! A guard that measures one of two targets reports green for a promise it
//! checked half of. Both arms now run the same sweep, and the target is a
//! parameter — the third target added tomorrow is charged by adding a row.
//!
//! # What makes it honest rather than a spell-check
//!
//! It reads **nothing written here**: the struct types and their fields are
//! parsed out of the designer the generator just produced, and the component
//! list comes from `crawrad_registry::Registry::builtin()`. A class added to the
//! catalogue tomorrow is charged tomorrow, and a field added to a state struct
//! is charged the moment it is added — which is the moment it is cheapest to
//! wire up.
//!
//! # What it is NOT
//!
//! It is a **text** measurement, so it proves the field is mentioned, not that
//! it is mentioned correctly: a `let _ = self.pnl.enabled;` would satisfy it
//! and change nothing on screen. That dodge is the kind the house forbids by
//! rule rather than by test, and the drawing gate
//! (`tests/gate_desenho_comparado.rs`) is where a wrong drawing gets caught.
//! The complement is deliberate: this file catches **forgetting**, which is
//! what actually happened seven times, and then an eighth in a whole target.

use crate::Gerador;
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;
use std::collections::BTreeMap;

/// **A target, as this sweep needs to see it.**
///
/// Three questions, and every target answers them differently enough that a
/// single hard-coded path would only ever fit one:
///
/// - which generator produces the designer;
/// - how the drawing REACHES a field (`self.btn.text` in egui,
///   `self.form.btn.text` in Makepad, because there the form is a field of the
///   `App`);
/// - WHICH parts of the designer count as drawing — the egui target has one
///   `show()`, the Makepad target has `sync_ui` and `handle_actions` with
///   `Default` impls in between.
struct Alvo {
    nome: &'static str,
    designer: fn(&Document) -> String,
    /// Prefix that leads to the component: `self.` or `self.form.`.
    acesso: &'static str,
    /// Snippets of the designer that READ state, already sliced.
    leitura: fn(&str) -> String,
}

/// The egui target's designer.
fn designer_egui(doc: &Document) -> String {
    designer_de(crate::GeradorRxp, doc)
}

/// The Makepad target's designer.
fn designer_makepad(doc: &Document) -> String {
    designer_de(crate::GeradorMakepad, doc)
}

fn designer_de(g: impl Gerador, doc: &Document) -> String {
    g.gerar(doc, "app", &crawrad_registry::Registry::builtin())
        .expect("generating")
        .into_iter()
        .find(|a| a.caminho.to_string_lossy().contains("designer"))
        .expect("the designer file")
        .conteudo
}

/// Body of egui's `show()` — from it to the end of the file.
///
/// `impl Default` is left out **on purpose**: it WRITES every field, and
/// including it would make this test pass for a field the drawing never inspects,
/// which is precisely the defect being measured.
fn leitura_egui(fonte: &str) -> String {
    let i = fonte
        .find("pub fn show(&mut self")
        .expect("the egui designer always has `show()`");
    fonte[i..].to_string()
}

/// The TWO Makepad snippets that read state: `sync_ui` and
/// `handle_actions`.
///
/// Between them live `impl Default for FormMain`, `impl Default for App`, and
/// `LiveRegister` — and all three are trimmed for the same reason as egui's `Default`:
/// writing a field does not prove anyone reads it.
fn leitura_makepad(fonte: &str) -> String {
    let i = fonte
        .find("pub fn sync_ui(&mut self")
        .expect("the Makepad designer always has `sync_ui()`");
    let fim_do_sync = fonte[i..]
        .find("pub struct FormMain")
        .map(|j| i + j)
        .expect("the `sync_ui` is followed by the form struct");
    let acoes = fonte
        .find("fn handle_actions(&mut self")
        .expect("the Makepad designer always has `handle_actions()`");
    format!("{}\n{}", &fonte[i..fim_do_sync], &fonte[acoes..])
}

/// **One of every class in the catalogue**, plus a `PageControl` with two real
/// pages — a `TabSheet` only exists inside one, so the sweep would never see
/// its fields otherwise.
fn form_with_everything(target: &str) -> Document {
    let r = Registry::builtin();
    let mut d = Document::new("form_main", "Form");
    {
        let root = d.node_mut("form_main").expect("the brand new root");
        root.set_rect(Rect::new(0.0, 0.0, 640.0, 420.0));
        root.set_name("form_main");
        root.set_text("Form1");
    }
    let mut y = 8.0;
    let mut descs: Vec<&crawrad_registry::ComponentDesc> = r.all().collect();
    // Collection items must be instantiated after their owners exist.
    descs.sort_by_key(|d| r.all().any(|o| o.item_class() == Some(d.class)));
    for desc in descs {
        if desc.class == "Form" || desc.class == "TabSheet" {
            continue;
        }
        // Only place components that the target under test supports.
        if !desc.supports(target) {
            continue;
        }
        let id = r.suggest_id(desc.class, |i| d.node(i).is_some());
        let (w, h) = desc.default_size;
        let n = r
            .instantiate(desc.class, id, Rect::new(8.0, y, w, h))
            .expect("the catalogue instantiates what it lists");
        // A collection item goes inside its owner, as on a real form (
        // a StatusPanel is read by the StatusBar that draws it); the owner is
        // whoever declares `Filhos::NaColecao { item }` for this class.
        let owner = d
            .nodes
            .values()
            .find(|o| {
                r.get(&o.class)
                    .and_then(|od| od.item_class())
                    .is_some_and(|item| item == desc.class)
            })
            .map(|o| o.id.clone());
        match owner {
            Some(owner) => d.add_child(&owner, n).expect("it goes in its collection"),
            None => d.add_child("form_main", n).expect("it goes on the form"),
        };
        // A Splitter sits between two controls, as on a real form (
        // it moves them, and its MinSize bounds that)
        if desc.class == "Splitter" {
            for (side, x) in [("left", 8.0 - 60.0), ("right", 8.0 + w)] {
                let pid = format!("pnl_split_{side}");
                let p = r
                    .instantiate("Panel", pid, Rect::new(x, y, 60.0, h))
                    .expect("instantiates");
                d.add_child("form_main", p).expect("it goes on the form");
            }
        }
        y += h + 4.0;
    }

    // A PopupMenu with an item, and a MainMenu item with a sub-menu, as on a
    // real form: a control's `PopupMenu` is read by the
    // secondary click that opens it, an item's state by the list it is in
    let owners: Vec<String> = d
        .nodes
        .values()
        .filter(|n| {
            n.class == "PopupMenu"
                || (n.class == "MenuItem"
                    && d.parent_of(&n.id)
                        .is_some_and(|p| d.node(p).is_some_and(|p| p.class == "MainMenu")))
        })
        .map(|n| n.id.clone())
        .collect();
    for owner in owners {
        let id = r.suggest_id("MenuItem", |i| d.node(i).is_some());
        let n = r
            .instantiate("MenuItem", id, Rect::new(0.0, 0.0, 28.0, 28.0))
            .expect("instantiates");
        d.add_child(&owner, n).expect("it goes in the menu");
    }

    // A SpeedButton on the ToolBar, as on a real form (the bar's Flat
    // and ShowCaptions are read by the buttons on it)
    if let Some(bar) = d
        .nodes
        .values()
        .find(|n| n.class == "ToolBar")
        .map(|n| n.id.clone())
    {
        let id = r.suggest_id("SpeedButton", |i| d.node(i).is_some());
        let b = r
            .instantiate("SpeedButton", id, Rect::new(0.0, 0.0, 24.0, 24.0))
            .expect("instantiates");
        d.add_child(&bar, b).expect("it goes on the bar");
    }

    let pgc = r
        .suggest_id("PageControl", |i| d.node(i).is_some())
        .to_string();
    let n = r
        .instantiate("PageControl", pgc.clone(), Rect::new(8.0, y, 320.0, 200.0))
        .expect("instantiates");
    d.add_child("form_main", n).expect("it goes on the form");
    for _ in 0..2 {
        let id = r.suggest_id("TabSheet", |i| d.node(i).is_some());
        let t = r
            .instantiate("TabSheet", id, Rect::new(0.0, 24.0, 320.0, 176.0))
            .expect("instantiates");
        d.add_child(&pgc, t).expect("the page goes in the control");
    }

    d.validate().expect("valid");
    d
}

/// The fields of every `pub struct` in `fonte`, by type name.
///
/// A plain brace scan is enough and is on purpose: the designer is text this
/// crate wrote three lines ago, not source of unknown shape. A parser here
/// would be a second thing to keep right.
fn structs(fonte: &str) -> BTreeMap<String, Vec<String>> {
    let mut out = BTreeMap::new();
    let mut resto = fonte;
    while let Some(i) = resto.find("pub struct ") {
        let depois = &resto[i + "pub struct ".len()..];
        let Some(abre) = depois.find('{') else { break };
        let nome = depois[..abre].trim().to_string();
        let Some(fecha) = depois[abre..].find('}') else {
            break;
        };
        let campos = depois[abre + 1..abre + fecha]
            .lines()
            .filter_map(|l| l.trim().strip_prefix("pub "))
            .filter_map(|l| l.split(':').next())
            .map(|c| c.trim().to_string())
            .filter(|c| !c.is_empty())
            .collect::<Vec<_>>();
        if !nome.is_empty() && !nome.contains(' ') {
            out.insert(nome, campos);
        }
        resto = &depois[abre + fecha..];
    }
    out
}

/// **The verification for a target.** Returns what that target's drawing ignores.
///
/// The sweep is identical for all targets, which is why target is parameterized: a
/// second copy of this function would be a second place to maintain, and a target
/// added tomorrow would be left out just as Makepad was.
fn campos_que_o_desenho_ignora(alvo: &Alvo) -> std::collections::BTreeSet<String> {
    let doc = form_with_everything(alvo.nome);
    let g = (alvo.designer)(&doc);
    let tipos = structs(&g);
    let corpo = (alvo.leitura)(&g);

    let campos_do_form = tipos.get("FormMain").unwrap_or_else(|| {
        panic!(
            "the designer for target `{}` defines the form struct",
            alvo.nome
        )
    });
    let instancias: Vec<(String, String)> = g
        .lines()
        .map(str::trim)
        .filter_map(|l| l.strip_prefix("pub ")?.strip_suffix(','))
        .filter_map(|l| l.split_once(": "))
        .map(|(n, t)| (n.to_string(), t.to_string()))
        .filter(|(n, _)| n != "base" && campos_do_form.contains(n))
        .collect();

    assert!(
        instancias.len() >= 15,
        "in target `{}` the sweep found only {} component(s) on the form — \
         it stopped measuring: {instancias:?}",
        alvo.nome,
        instancias.len()
    );

    // `Type.field`, not `instance.field`: honoring a property belongs to the
    // COMPONENT, so two `PageControl`s on the same form represent a single
    // debt. The instance name remains in the error message, where it
    // serves a purpose.
    let mut esquecidos = std::collections::BTreeSet::new();
    let mut medidos = 0usize;
    for (nome, tipo) in &instancias {
        let campos = tipos.get(tipo).unwrap_or_else(|| {
            panic!(
                "the designer for target `{}` does not define the type `{tipo}` of `{nome}`",
                alvo.nome
            )
        });
        // A component's own method (`impl MessageDialog { fn execute }`)
        // reads its state too: the user's code calls it. Only an inherent
        // `impl Tipo {` counts — never `impl Default for`, which only writes.
        let own_methods = inherent_impl(&g, tipo);
        for campo in campos {
            medidos += 1;
            let in_body = corpo.contains(&format!("{}{nome}.{campo}", alvo.acesso));
            let in_method = own_methods.contains(&format!("self.{campo}"));
            if !in_body && !in_method {
                esquecidos.insert(format!("{tipo}.{campo}"));
            }
        }
    }

    assert!(
        medidos >= 40,
        "in target `{}` only {medidos} field(s) measured — the sweep became blind",
        alvo.nome
    );
    esquecidos
}

/// The text of `impl <tipo> {` … `}` (the type's own methods), or empty.
fn inherent_impl(source: &str, tipo: &str) -> String {
    let head = format!("impl {tipo} {{");
    let Some(start) = source.find(&head) else {
        return String::new();
    };
    let rest = &source[start..];
    let end = rest.find("\n}").map_or(rest.len(), |i| i + 2);
    rest[..end].to_string()
}

/// The message the owner reads when a target leaves a property with no effect.
fn cobrar(alvo: &Alvo) {
    let esquecidos = campos_que_o_desenho_ignora(alvo);
    assert!(
        esquecidos.is_empty(),
        "TARGET `{}` — PROPERTY IGNORED BY APPLICATION: user edits in \
         Object Inspector and drawing does not read it:\n  {}\n\n\
         Each of these lines breaks an ADR 0010 promise: state is \
         state, not inspector decoration. This is how `Panel.enabled`, \
         `GroupBox.enabled`, `PageControl.enabled`, `ScrollBox.auto_scroll`, and \
         the two `change` fields of `ScrollBar` spent years with no effect — \
         silenced by a `#![allow(dead_code)]` that no longer exists.",
        alvo.nome,
        esquecidos.into_iter().collect::<Vec<_>>().join("\n  ")
    );
}

/// **THE MAKEPAD TARGET DEBT, measured on 18/09/2026 — and it can only SHRINK.**
///
/// Ten fields (previously 38; paid `border_width` of
/// `ScrollBox`, `Edit.password_char` and
/// `Edit.readonly`, `enabled` across fourteen classes, `enabled` and `interval_ms` of `Timer`, `items` of `ComboBox`, those of `ListBox`, those of `TreeView`, `tab_order` of three classes) that Makepad drawing does not read. Not an exemption
/// list: it is the **record** of debt, and the test below enforces that the measured
/// set matches this EXACTLY. A new field in this situation fails; a fixed field
/// also fails, until its line is removed here. The list cannot
/// rot in silence, the flaw of all exception lists.
///
/// # Why it exists instead of eighteen ad-hoc fixes
///
/// Because these are two different issues and neither is fixed one by one:
///
/// 1. **inert** — `Button.enabled` is neither read **nor** baked in: `Button`'s
///    `emit_dsl` emits position, size, and text, nothing else. `Timer` is the
///    extreme case: `emit_dsl` and `emit_actions` return empty strings, so it
///    does absolutely nothing in the generated application;
/// 2. **baked in at generation** — `ComboBox.enabled` and company become literals in
///    `live_design!` (`format!("enabled: {en}")`). Design-time value is
///    honored; runtime changes are not.
///
/// The second is a consequence of the target being RETAINED mode with static DSL, and
/// ADR 0036
/// decided that class behavioral emission trends to zero — behavior
/// shifts to the runtime crate. **Fixing eighteen
/// fields within an architecture being phased out means paying twice.**
///
/// So the number stays visible as the metric: **10 today, zero in the end.**
const DIVIDA_MAKEPAD: &[&str] = &[
    "GroupBox.text",
    "Panel.border_width",
    "Panel.text",
    "RadioButton.text",
    "ScrollBar.large_change",
    "ScrollBox.auto_scroll",
    "ScrollBox.horz_scroll_bar",
    "ScrollBox.vert_scroll_bar",
    "TabSheet.enabled",
    "TabSheet.visible",
];

const EGUI: Alvo = Alvo {
    nome: "egui",
    designer: designer_egui,
    acesso: "self.",
    leitura: leitura_egui,
};

const MAKEPAD: Alvo = Alvo {
    nome: "makepad",
    designer: designer_makepad,
    acesso: "self.form.",
    leitura: leitura_makepad,
};

/// **Verification of the egui target.** Every field of every component is read.
///
/// The mutation this test catches is the coordinator one: making `Panel` ignore
/// `enabled` again. Without this file it passed with all 180 tests green,
/// and only `gate_sem_avisos` — which is `--ignored`, so nobody runs it —
/// would have had a word to say.
#[test]
fn every_state_field_is_read_by_the_drawing() {
    cobrar(&EGUI);
}

/// **Verification of the Makepad target** — what the IDE actually delivers, which until
/// 18/09/2026 was not measured.
///
/// It acts as a **ratchet**: the measured set must match
/// [`DIVIDA_MAKEPAD`] exactly. An unread new field fails; a fixed field fails
/// until its line leaves the record. The list cannot grow on its own or
/// lie due to age.
#[test]
fn every_state_field_is_read_by_the_drawing_makepad() {
    let medida = campos_que_o_desenho_ignora(&MAKEPAD);
    let registro: std::collections::BTreeSet<String> =
        DIVIDA_MAKEPAD.iter().map(|s| s.to_string()).collect();

    let novos: Vec<&String> = medida.difference(&registro).collect();
    assert!(
        novos.is_empty(),
        "NEW DEBT in makepad target — drawing does not read:\n  {}\n\n\
         Each of these lines breaks an ADR 0010 promise: user edits \
         the property in Object Inspector and generated application does not honor it. \
         Adding to `DIVIDA_MAKEPAD` is NOT the fix — the list is a debt \
         record, and it only shrinks.",
        novos
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );

    let pagos: Vec<&String> = registro.difference(&medida).collect();
    assert!(
        pagos.is_empty(),
        "PAID DEBT still recorded — drawing now reads it:\n  {}\n\n\
         Delete these lines from `DIVIDA_MAKEPAD`. The count in its doc is the \
         ADR 0036 ruler, and it is only valid while true.",
        pagos
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join("\n  ")
    );

    assert_eq!(
        medida.len(),
        10,
        "Makepad debt changed size and both checks above \
         passed — this only happens if the sweep stopped measuring"
    );
}
