//! **An agent builds a form from scratch, with no shell at all.**
//!
//! This file is not another catalogue unit test: it is the proof that the
//! surface of [ADR 0005] closes. An empty form enters, and leaves with two
//! components created, a property changed, a component renamed and an
//! event wired — **all through the catalogue**, via `by_id` and `Args`, with the
//! document being touched only through `Doc::apply`.
//!
//! What it measures that tests inside the crate do not measure:
//!
//! 1. **The action is sufficient.** Here there is no `Ide`, no `egui`, no
//!    `crawrad-ide` in the graph — and even so the form stands up. If this
//!    test passes, the Phase 4 MCP server already has what to expose: it is this
//!    applier of [`Effect`] with a transport on top.
//! 2. **`Author` arrives.** [`Author::Agent`] existed since the first day with
//!    zero producers in production, and the doc comment of `Context::author`
//!    recorded that as an unpaid promise. These four are the first
//!    an agent will call, so here the audit must state
//!    `agent:claude` on **every** entry, not `user`.
//!
//! The approach is that of the windowless harness in `crawrad-runtime-mock/tests/fatia_vertical.rs`:
//! a miniature IDE, with the same parts and no interface. The difference is
//! that there the test builds the transaction, and here the catalogue builds it —
//! which is precisely what was missing.
//!

use crawrad_actions::{Args, Author, Context, Effect, Project, by_id};
use crawrad_designer::clipboard::Clipboard;
use crawrad_doc::{Doc, DocError};
use crawrad_ir::{Document, Rect, Value};
use crawrad_registry::Registry;

/// **The other side of the protocol, in 40 lines.**
///
/// It is the [`Effect`] applier that the MCP server will be: it does not know what
/// each action does, it only knows how to translate what it DECLARES. No `match` by id
/// inside here — if there were, the test would be measuring the test.
struct Agent {
    doc: Doc,
    registry: Registry,
    clipboard: Clipboard,
    who: Author,
    /// What the shell status bar would say. A refused request must
    /// PRODUCE a sentence — a test that does not read it does not measure what the caller
    /// receives, and on the other side of the protocol that sentence is the error response.
    status: Option<String>,
    /// What the selection would be. The agent has no screen, but [`Effect::selection`] is
    /// how it discovers the id of what it just created: `component.add` chooses
    /// the free name (`btn1`, `btn2`), and without it the caller would have to guess.
    selection: Vec<String>,
}

impl Agent {
    /// An empty form, sized as `New Project…` creates it.
    fn new(who: &str) -> Self {
        let registry = Registry::builtin();
        let mut d = Document::new("form_main", "Form");
        let root = registry
            .instantiate("Form", d.root.clone(), Rect::new(0.0, 0.0, 640.0, 420.0))
            .expect("Form is in the catalogue");
        d.nodes.insert(d.root.clone(), root);
        Self {
            doc: Doc::new(d),
            registry,
            clipboard: Clipboard::default(),
            who: Author::Agent(who.to_string()),
            status: None,
            selection: Vec::new(),
        }
    }

    /// Calls an action **by id**, with named arguments, and applies the effect.
    ///
    /// `enabled` is consulted beforehand, as the shell does and as MCP will have to
    /// do to avoid offering a tool that is not valid right now.
    fn call(&mut self, id: &str, args: Args) -> Result<(), DocError> {
        let action = by_id(id).unwrap_or_else(|| panic!("`{id}` is not in the catalogue"));
        let effect = {
            let c = Context {
                doc: Some(&self.doc),
                registry: &self.registry,
                clipboard: &self.clipboard,
                args,
                project: Project::default(),
                author: self.who.clone(),
                editor_dirty: false,
                text: Default::default(),
            };
            assert!(
                action.enabled.allows(&c),
                "`{id}` should be enabled with the form open"
            );
            (action.execute)(&c)
        };
        self.apply(effect)
    }

    fn apply(&mut self, e: Effect) -> Result<(), DocError> {
        self.status = e.status;
        if let Some(tx) = e.transaction {
            self.doc.apply(tx)?;
        }
        if let Some(sel) = e.selection {
            self.selection = sel;
        }
        Ok(())
    }

    /// The id of what the last action created.
    fn created(&self) -> String {
        self.selection
            .first()
            .cloned()
            .expect("create action returns id in `Effect::selection`")
    }

    fn node(&self, id: &str) -> &crawrad_ir::Node {
        self.doc
            .ir()
            .node(id)
            .unwrap_or_else(|| panic!("`{id}` should be in the document"))
    }
}

#[test]
fn agent_builds_a_form_from_scratch_through_the_catalogue() {
    let mut a = Agent::new("claude");

    // ---- 1. create two components -------------------------------------
    //
    // `component.add(class, x, y)` is the LITERAL example of ADR 0005, and it did
    // not fit while the argument was `Vec<String>` — `x` and `y` are numbers.
    a.call(
        "component.add",
        Args::text("class", "Button")
            .and("x", Value::Int(24))
            .and("y", Value::Int(24)),
    )
    .expect("create button");
    let button = a.created();

    a.call(
        "component.add",
        Args::text("class", "Edit")
            .and("x", Value::Float(24.0))
            .and("y", Value::Float(72.0))
            .and("w", Value::Int(240)),
    )
    .expect("create edit");
    let edit = a.created();

    assert_ne!(button, edit, "two free names, not the same twice");
    assert_eq!(
        a.node(&a.doc.ir().root.clone()).children,
        vec![button.clone(), edit.clone()],
        "both enter the form, in the order they were requested"
    );

    // Missing size comes from registry; present size commands. This is what separates
    // "the argument is optional" from "the argument is ignored".
    assert_eq!(
        a.node(&button).rect(),
        Some(Rect::new(24.0, 24.0, 96.0, 32.0)),
        "without `w`/`h`, size is the class default"
    );
    assert_eq!(
        a.node(&edit).rect(),
        Some(Rect::new(24.0, 72.0, 240.0, 24.0)),
        "with `w`, request overrides default"
    );

    // Tab order lived in shell, and so a component created
    // outside window was born tied at zero.
    assert_eq!(a.node(&button).props.get("tab_order"), Some(&Value::Int(0)));
    assert_eq!(a.node(&edit).props.get("tab_order"), Some(&Value::Int(1)));

    // ---- 2. change a property -----------------------------------------
    a.call(
        "component.set_property",
        Args::texts("nodes", [button.clone()])
            .and("key", Value::Str("text".into()))
            .and("value", Value::Str("Sign in".into())),
    )
    .expect("change text");
    assert_eq!(
        a.node(&button).props.get("text"),
        Some(&Value::Str("Sign in".into()))
    );

    // ---- 3. rename ----------------------------------------------------
    //
    // `Name` is a unique and validated property (ADR 0008), which is why it has its
    // own action instead of being another key of `set_property`.
    let before = a.node(&button).name().to_string();
    a.call(
        "component.rename",
        Args::text("node", &button).and("name", Value::Str("btn_sign_in".into())),
    )
    .expect("rename button");
    assert_eq!(a.node(&button).name(), "btn_sign_in");
    assert_ne!(before, "btn_sign_in", "the test must be measuring a change");
    // IDENTITY does not change with name — difference bought by ADR 0008,
    // and what lets agent continue using id received.
    assert!(a.doc.ir().node(&button).is_some(), "id survived name");

    // ---- 4. wire an event ---------------------------------------------
    a.call(
        "event.bind",
        Args::text("node", &button)
            .and("event", Value::Str("on_click".into()))
            .and("handler", Value::Str("btn_sign_in_click".into())),
    )
    .expect("wire click");
    assert_eq!(
        a.node(&button).events.get("on_click").map(String::as_str),
        Some("btn_sign_in_click")
    );

    // ---- document closes ----------------------------------------------
    //
    // Five catalogue calls later, what comes out is a VALID form —
    // not a command stack that happened to apply.
    a.doc
        .ir()
        .validate()
        .expect("form assembled by agent must be valid");

    // ---- audit says WHO -----------------------------------------------
    //
    // This is half of delivery invisible on screen. Without it,
    // history would say `user` for everything agent did, and promise
    // of `Author` would be broken precisely where it matters.
    let expected = Author::Agent("claude".into());
    assert_eq!(
        a.doc.audit().len(),
        5,
        "five requests, five entries — each action is an undo step"
    );
    for e in a.doc.audit() {
        assert_eq!(
            e.author, expected,
            "revision {} ({}) does not say who requested",
            e.revision, e.label
        );
    }

    // And label is what status bar shows: derived from document, not
    // raw id. Reading Ctrl+Z must recognize what was done.
    let labels: Vec<&str> = a.doc.audit().iter().map(|e| e.label.as_str()).collect();
    assert_eq!(
        labels,
        vec![
            "Add Button",
            "Add Edit",
            &format!("{before}.text")[..],
            &format!("Rename {before} to btn_sign_in")[..],
            "btn_sign_in.on_click",
        ]
    );

    // ---- and undo -----------------------------------------------------
    //
    // Five Ctrl+Z return empty form. Leverage study §12.5 gives
    // agent: mistakes are cheap because change is transaction,
    // not rewritten file.
    for _ in 0..5 {
        a.doc.undo().expect("undo what agent did");
    }
    let root = a.doc.ir().root.clone();
    assert!(
        a.node(&root).children.is_empty(),
        "five undos must return empty form"
    );
}

#[test]
fn agent_modifies_the_same_property_on_both_in_a_single_step() {
    // Plural `nodes` is not decorative: "2 items selected" of Inspector
    // becoming primitive. One batch = one transaction = **one** Ctrl+Z, what
    // `crawrad-doc` sells and loop outside would discard.
    let mut a = Agent::new("claude");
    for _ in 0..2 {
        a.call("component.add", Args::text("class", "Button"))
            .expect("create");
    }
    let root = a.doc.ir().root.clone();
    let pair = a.node(&root).children.clone();
    let revision = a.doc.revision();

    a.call(
        "component.set_property",
        Args::texts("nodes", pair.clone())
            .and("key", Value::Str("enabled".into()))
            .and("value", Value::Bool(false)),
    )
    .expect("disable both");

    for id in &pair {
        assert_eq!(a.node(id).props.get("enabled"), Some(&Value::Bool(false)));
    }
    assert_eq!(
        a.doc.revision(),
        revision + 1,
        "both must fit into a single revision"
    );
    assert_eq!(
        a.doc.undo_label(),
        Some("Button.enabled (2 items)"),
        "batch label states class and count, as Inspector wrote"
    );

    a.doc.undo().expect("one Ctrl+Z");
    for id in &pair {
        assert_ne!(
            a.node(id).props.get("enabled"),
            Some(&Value::Bool(false)),
            "one Ctrl+Z must return BOTH"
        );
    }
}

#[test]
fn crooked_request_returns_with_message_and_without_transaction() {
    // Other side of protocol has no status bar to read nor unlit
    // button to view: has call response. Request that cannot
    // be served must return saying WHY — silence here becomes, on
    // agent side, form thought changed.
    let mut a = Agent::new("claude");
    a.call("component.add", Args::text("class", "Button"))
        .expect("create button");
    let button = a.created();
    let revision = a.doc.revision();

    let refusals: &[(&str, Args, &str)] = &[
        (
            "component.add",
            Args::text("class", "Bottom"),
            "unknown class: Bottom",
        ),
        (
            "component.add",
            Args::text("class", "Button").and("parent", Value::Str("does_not_exist".into())),
            "no such component: `does_not_exist`",
        ),
        (
            "component.add",
            Args::none(),
            "add component: `class` is required",
        ),
        (
            "component.rename",
            Args::text("node", "does_not_exist").and("name", Value::Str("btnX".into())),
            "no such component: `does_not_exist`",
        ),
        (
            "component.set_property",
            Args::texts("nodes", [button.clone()]).and("key", Value::Str("text".into())),
            "set property: `value` is required",
        ),
        // Event class lacks. **Not a guard** —
        // `Document::validate` still ignores `events`, invented event
        // entering via another path still passes. Gain is caller
        // knowing mistyped name, rather than non-firing handler and useless stub.
        (
            "event.bind",
            Args::text("node", &button)
                .and("event", Value::Str("on_clik".into()))
                .and("handler", Value::Str("x".into())),
            "`on_clik` is not an event of Button",
        ),
    ];

    for (id, args, sentence) in refusals {
        a.status = None;
        a.call(id, args.clone()).expect("refusal is not doc error");
        assert_eq!(
            a.status.as_deref(),
            Some(*sentence),
            "`{id}` refused without stating what happened"
        );
    }
    assert_eq!(
        a.doc.revision(),
        revision,
        "no refused request may touch document"
    );
}

#[test]
fn binding_and_unbinding_event_is_the_same_action() {
    // Two actions for back and forth would be two places to fix, same
    // reason as `ViewEffect::ToggleDesignCode`. Here return is empty
    // `handler`, literally what Inspector field does when user
    // clears text.
    let mut a = Agent::new("claude");
    a.call("component.add", Args::text("class", "Button"))
        .expect("create");
    let button = a.created();

    let bind = |h: &str| {
        Args::text("node", &button)
            .and("event", Value::Str("on_click".into()))
            .and("handler", Value::Str(h.into()))
    };

    a.call("event.bind", bind("btn1_click")).expect("bind");
    assert!(a.node(&button).events.contains_key("on_click"));

    // Requesting again what is already there is not a change: transaction here
    // would push Ctrl+Z undoing nothing visible.
    let revision = a.doc.revision();
    a.call("event.bind", bind("btn1_click")).expect("repeat");
    assert_eq!(a.doc.revision(), revision, "rebinding same is not change");

    a.call("event.bind", bind("")).expect("unbind");
    assert!(
        !a.node(&button).events.contains_key("on_click"),
        "empty handler UNBINDS event, does not store empty string"
    );

    // Same applies with no argument: absent and empty state same
    // thing, unbind gesture having no argument to give.
    a.call("event.bind", bind("btn1_click")).expect("rebind");
    a.call(
        "event.bind",
        Args::text("node", &button).and("event", Value::Str("on_click".into())),
    )
    .expect("unbind without argument");
    assert!(!a.node(&button).events.contains_key("on_click"));
}

#[test]
fn binding_and_unbinding_data_binding_is_the_same_action() {
    let mut a = Agent::new("claude");
    a.call("component.add", Args::text("class", "Edit"))
        .expect("create");
    let edit = a.created();

    let bind = |src: &str| {
        Args::text("node", &edit)
            .and("prop", Value::Str("text".into()))
            .and("source", Value::Str(src.into()))
    };

    a.call("binding.set", bind("ds_clients.NAME"))
        .expect("wire binding");
    assert!(a.node(&edit).binding("text").is_some());
    let b = a.node(&edit).binding("text").unwrap();
    assert_eq!(b.source, "ds_clients");
    assert_eq!(b.field.as_deref(), Some("NAME"));

    let revision = a.doc.revision();
    a.call("binding.set", bind("ds_clients.NAME"))
        .expect("repeat");
    assert_eq!(
        a.doc.revision(),
        revision,
        "rebinding same binding is not change"
    );

    a.call("binding.set", bind("")).expect("unbind binding");
    assert!(
        a.node(&edit).binding("text").is_none(),
        "empty source UNBINDS binding"
    );

    a.call("binding.set", bind("ds_clients.NAME"))
        .expect("rebind");
    a.call(
        "binding.set",
        Args::text("node", &edit).and("prop", Value::Str("text".into())),
    )
    .expect("unbind without source argument");
    assert!(a.node(&edit).binding("text").is_none());
}

#[test]
fn the_five_are_published_with_signatures() {
    // Signature IS MCP tool: `Param::name` that catalogue
    // announces must be key `execute` reads, otherwise agent sends
    // right argument with wrong name and gets no error.
    //
    // All five REQUIRE argument, which removes them from menus: menu
    // has nowhere to type node id (see `Action::requires_argument`).
    let expected: &[(&str, &[&str])] = &[
        ("component.add", &["class", "parent", "x", "y", "w", "h"]),
        (
            "component.set_property",
            &["nodes", "key", "value", "label"],
        ),
        ("component.rename", &["node", "name", "everywhere"]),
        ("event.bind", &["node", "event", "handler"]),
        ("event.rename", &["node", "event", "name"]),
        (
            "binding.set",
            &["node", "prop", "source", "field", "mode", "converter"],
        ),
    ];
    for (id, names) in expected {
        let a = by_id(id).unwrap_or_else(|| panic!("`{id}` vanished from catalogue"));
        let published: Vec<&str> = a.params.iter().map(|p| p.name).collect();
        assert_eq!(&published[..], *names, "signature of `{id}` changed");
        assert!(
            a.requires_argument(),
            "`{id}` without required argument would appear in menus, active and inert"
        );
        assert!(
            !a.hint.is_empty(),
            "`{id}` without hint does not become tool"
        );
        for p in a.params {
            assert!(
                !p.description.is_empty(),
                "`{id}.{}` without description does not become tool parameter",
                p.name
            );
        }
    }
}

#[test]
fn renaming_handler_moves_every_binding() {
    let mut a = Agent::new("claude");
    a.call("component.add", Args::text("class", "Button"))
        .expect("create");
    let btn1 = a.created();
    a.call("component.add", Args::text("class", "Button"))
        .expect("create");
    let btn2 = a.created();

    a.call(
        "event.bind",
        Args::text("node", &btn1)
            .and("event", Value::Str("on_click".into()))
            .and("handler", Value::Str("btn1_click".into())),
    )
    .expect("bind btn1");
    a.call(
        "event.bind",
        Args::text("node", &btn2)
            .and("event", Value::Str("on_click".into()))
            .and("handler", Value::Str("btn1_click".into())),
    )
    .expect("bind btn2");

    assert_eq!(
        a.node(&btn1).events.get("on_click").map(String::as_str),
        Some("btn1_click")
    );
    assert_eq!(
        a.node(&btn2).events.get("on_click").map(String::as_str),
        Some("btn1_click")
    );

    let revision = a.doc.revision();
    a.call(
        "event.rename",
        Args::text("node", &btn1)
            .and("event", Value::Str("on_click".into()))
            .and("name", Value::Str("save_click".into())),
    )
    .expect("rename handler");

    assert_eq!(a.doc.revision(), revision + 1);
    assert_eq!(
        a.node(&btn1).events.get("on_click").map(String::as_str),
        Some("save_click")
    );
    assert_eq!(
        a.node(&btn2).events.get("on_click").map(String::as_str),
        Some("save_click")
    );

    a.doc.undo().expect("one Ctrl+Z");
    assert_eq!(
        a.node(&btn1).events.get("on_click").map(String::as_str),
        Some("btn1_click")
    );
    assert_eq!(
        a.node(&btn2).events.get("on_click").map(String::as_str),
        Some("btn1_click")
    );
}

#[test]
fn renaming_handler_refuses_taken_empty_and_invalid() {
    let mut a = Agent::new("claude");
    a.call("component.add", Args::text("class", "Button"))
        .expect("create button a");
    let btn_a = a.created();
    a.call("component.add", Args::text("class", "Button"))
        .expect("create button b");
    let btn_b = a.created();

    a.call(
        "event.bind",
        Args::text("node", &btn_a)
            .and("event", Value::Str("on_click".into()))
            .and("handler", Value::Str("a_click".into())),
    )
    .expect("bind btn_a");
    a.call(
        "event.bind",
        Args::text("node", &btn_b)
            .and("event", Value::Str("on_click".into()))
            .and("handler", Value::Str("b_click".into())),
    )
    .expect("bind btn_b");

    let rename = |name: &str| {
        Args::text("node", &btn_a)
            .and("event", Value::Str("on_click".into()))
            .and("name", Value::Str(name.into()))
    };

    let revision = a.doc.revision();

    a.call("event.rename", rename("b_click"))
        .expect("call rename with taken name");
    assert_eq!(
        a.doc.revision(),
        revision,
        "taken name must not change document"
    );

    a.call("event.rename", rename(""))
        .expect("call rename with empty name");
    assert_eq!(
        a.doc.revision(),
        revision,
        "empty name must not change document"
    );

    a.call("event.rename", rename("1x"))
        .expect("call rename with invalid identifier");
    assert_eq!(
        a.doc.revision(),
        revision,
        "invalid name must not change document"
    );

    a.call("event.rename", rename("a_click"))
        .expect("call rename with same name");
    assert_eq!(
        a.doc.revision(),
        revision,
        "same name must not change document"
    );
}

/// **The Inspector's Events field renames only to a new name**: a name in use
/// shares that handler, empty unbinds, an unbound event binds.
#[test]
fn the_events_field_renames_only_to_a_new_name() {
    let mut a = Agent::new("claude");
    a.call("component.add", Args::text("class", "Button"))
        .expect("create");
    let btn = a.created();
    assert_eq!(
        crawrad_actions::handler_edit(None, &btn, "on_click", "x").0,
        "event.bind"
    );
    a.call(
        "event.bind",
        Args::text("node", &btn)
            .and("event", Value::Str("on_click".into()))
            .and("handler", Value::Str("a_click".into())),
    )
    .expect("bind");
    let edit = |text: &str| crawrad_actions::handler_edit(Some(a.doc.ir()), &btn, "on_click", text);
    assert_eq!(edit(" save_click ").0, "event.rename");
    assert_eq!(edit(" save_click ").1.as_text("name"), Some("save_click"));
    assert_eq!(edit("a_click").0, "event.bind");
    assert_eq!(edit("").0, "event.bind");
    assert_eq!(
        crawrad_actions::handler_edit(Some(a.doc.ir()), &btn, "on_enter", "z").0,
        "event.bind"
    );
}
