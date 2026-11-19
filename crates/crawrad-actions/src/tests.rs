use super::*;
use crawrad_ir::{Document, Node, Rect};

fn scenario() -> (Doc, Registry, Clipboard) {
    let mut d = Document::new("form_main", "Form");
    d.node_mut("form_main")
        .unwrap()
        .set_rect(Rect::new(0.0, 0.0, 400.0, 300.0));
    for (id, r) in [
        ("Button1", Rect::new(10.0, 10.0, 96.0, 32.0)),
        ("Button2", Rect::new(200.0, 100.0, 96.0, 32.0)),
    ] {
        let mut n = Node::new(id, "Button");
        n.set_rect(r);
        n.props
            .insert("tab_order".into(), crawrad_ir::Value::Int(0));
        d.add_child("form_main", n).unwrap();
    }
    (Doc::new(d), Registry::builtin(), Clipboard::default())
}

fn ctx<'a>(doc: &'a Doc, reg: &'a Registry, clipboard: &'a Clipboard) -> Context<'a> {
    ctx_with(doc, reg, clipboard, Args::none())
}

fn ctx_with<'a>(
    doc: &'a Doc,
    reg: &'a Registry,
    clipboard: &'a Clipboard,
    args: Args,
) -> Context<'a> {
    ctx_from(doc, reg, clipboard, args, Author::User)
}

/// The same, stating WHO requests. This is what audit tests go through:
/// the shell sends `User` because that is true, and the other side of the protocol sends
/// `Agent`.
fn ctx_from<'a>(
    doc: &'a Doc,
    reg: &'a Registry,
    clipboard: &'a Clipboard,
    args: Args,
    author: Author,
) -> Context<'a> {
    Context {
        doc: Some(doc),
        registry: reg,
        clipboard,
        args,
        // The project that RUNS — the default. The opposite case has its own scenario,
        // in `run_is_not_offered_when_there_is_nothing_to_run`.
        project: Project::default(),
        author,
        editor_dirty: false,
        text: Default::default(),
    }
}

/// The context **without document** — newly installed IDE, folder without
/// form, newly created console project (ADR 0017).
fn ctx_without_document<'a>(reg: &'a Registry, clipboard: &'a Clipboard) -> Context<'a> {
    Context {
        doc: None,
        registry: reg,
        clipboard,
        args: Args::none(),
        project: Project::default(),
        author: Author::User,
        editor_dirty: false,
        text: Default::default(),
    }
}

#[test]
fn every_catalogue_id_is_unique() {
    // Ids are the public surface of the RTA: a repeated id would cause the agent's
    // tool to point to the wrong action.
    let cat = catalogue();
    let mut ids: Vec<&str> = cat.iter().map(|a| a.id).collect();
    ids.sort_unstable();
    let before = ids.len();
    ids.dedup();
    assert_eq!(
        before,
        ids.len(),
        "there is a duplicate id in the catalogue"
    );
}

#[test]
fn every_action_has_label_and_group() {
    for a in catalogue() {
        assert!(!a.label.is_empty(), "{} without label", a.id);
        assert!(!a.group.is_empty(), "{} without group", a.id);
        assert!(
            a.id.contains('.'),
            "{} should be 'group.action', for MCP to group",
            a.id
        );
    }
}

#[test]
fn disabled_action_is_not_executed_in_vain() {
    // Without selection, actions depending on it remain disabled — and that is
    // what the IDE uses to grey out the button and the agent not to attempt.
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);
    for id in [
        "edit.copy",
        "edit.delete",
        "align.left",
        "order.bring_to_front",
    ] {
        let action = by_id(id).unwrap();
        assert!(!action.enabled.allows(&c), "{id} should not be enabled");
    }
}

#[test]
fn copy_returns_text_to_system_clipboard() {
    let (mut d, r, a) = scenario();
    d.select_one("Button1");
    let c = ctx(&d, &r, &a);
    let action = by_id("edit.copy").unwrap();
    assert!(action.enabled.allows(&c));

    let e = (action.execute)(&c);
    let txt = e.copy_text.expect("should return text");
    assert!(Clipboard::from_text(&txt).is_some());
    assert!(e.transaction.is_none(), "copy does not alter document");
}

#[test]
fn paste_returns_transaction_and_selection_of_created() {
    let (mut d, r, mut area) = scenario();
    d.select_one("Button1");
    area.copy(d.ir(), &["Button1".to_string()]);

    let c = ctx(&d, &r, &area);
    let e = ((by_id("edit.paste").unwrap()).execute)(&c);
    let tx = e.transaction.expect("should paste");
    let sel = e.selection.expect("should select what was pasted");
    assert_eq!(sel.len(), 1);
    assert_ne!(sel[0], "Button1");
    d.apply(tx).unwrap();
    assert_eq!(d.ir().nodes.len(), 4);
}

#[test]
fn align_left_produces_correct_transaction() {
    let (mut d, r, a) = scenario();
    d.select(vec!["Button1".into(), "Button2".into()]);
    let c = ctx(&d, &r, &a);
    let e = ((by_id("align.left").unwrap()).execute)(&c);
    let tx = e.transaction.expect("should align");
    d.apply(tx).unwrap();
    assert_eq!(d.ir().node("Button2").unwrap().rect().unwrap().x, 10.0);
}

#[test]
fn select_all_gets_form_children() {
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);
    let e = ((by_id("edit.select_all").unwrap()).execute)(&c);
    assert_eq!(e.selection.unwrap().len(), 2);
}

#[test]
fn undo_requests_history_instead_of_transaction() {
    // Undo is not a new transaction: it is an order to the document. Separation
    // makes this explicit for whoever executes, including the agent.
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);
    let e = ((by_id("edit.undo").unwrap()).execute)(&c);
    assert_eq!(e.history, Some(History::Undo));
    assert!(e.transaction.is_none());
}

#[test]
fn action_that_changes_nothing_returns_empty_effect() {
    let (mut d, r, a) = scenario();
    d.select(vec!["Button1".into(), "Button2".into()]);
    // aligns once
    let c = ctx(&d, &r, &a);
    let tx = ((by_id("align.top").unwrap()).execute)(&c)
        .transaction
        .unwrap();
    d.apply(tx).unwrap();

    // the second has nothing to do
    let c = ctx(&d, &r, &a);
    let e = ((by_id("align.top").unwrap()).execute)(&c);
    assert!(e.is_empty(), "cannot push empty transaction into history");
}

#[test]
fn search_by_id_finds_what_catalogue_publishes() {
    for a in catalogue() {
        assert!(by_id(a.id).is_some(), "{} vanished from search", a.id);
    }
    assert!(by_id("does.not.exist").is_none());
}

#[test]
fn groups_match_what_interface_expects() {
    for g in [
        "File",
        "Edit",
        "View",
        "Go",
        "Refactor",
        "Align",
        "Size",
        "Order",
        "In form",
        "Help",
        // Gained content on 15/09/2026, when the three project actions
        // left group `File`. They are here so the two menus never
        // become empty in silence again.
        "Run",
        "Project",
        // The `Component` of the classic RAD IDE, born on 18/09/2026 with package
        // installation. Enters here for the same reason as the two above: menu that becomes
        // empty in silence and menu that vanishes.
        "Component",
        // The `Tools` of the classic RAD IDE (Tools ▸ Options...).
        "Tools",
    ] {
        assert!(!by_group(g).is_empty(), "group {g} empty");
    }
}

/// A form with an action list, an action and a button wired to it.
fn with_actions() -> (Doc, Registry, Clipboard) {
    let (mut d, r, a) = scenario();
    let mut acl = r
        .instantiate(
            "ActionList",
            "ActionList1",
            Rect::new(300.0, 8.0, 28.0, 28.0),
        )
        .unwrap();
    acl.children.push("Action1".into());
    let mut act = r
        .instantiate("Action", "Action1", Rect::new(0.0, 0.0, 28.0, 28.0))
        .unwrap();
    act.props
        .insert("text".into(), crawrad_ir::Value::Str("&Save".into()));

    let mut doc = d.ir().clone();
    doc.nodes.insert("Action1".into(), act);
    doc.add_child("form_main", acl).unwrap();
    doc.node_mut("Button1")
        .unwrap()
        .props
        .insert("action".into(), crawrad_ir::Value::Ref("Action1".into()));
    doc.validate().expect("the scenario must be valid");
    d = Doc::new(doc);
    (d, r, a)
}

#[test]
fn create_action_requests_a_selected_list() {
    let (mut d, r, a) = with_actions();
    d.select_one("Button1");
    assert!(
        !by_id("action.new")
            .unwrap()
            .enabled
            .allows(&ctx(&d, &r, &a)),
        "a button is no place for a new action"
    );

    d.select_one("ActionList1");
    let e = ((by_id("action.new").unwrap()).execute)(&ctx(&d, &r, &a));
    let tx = e.transaction.expect("should create");
    let new_nodes = e.selection.expect("and select what was created");
    d.apply(tx).unwrap();
    assert_eq!(
        d.ir().node("ActionList1").unwrap().children.len(),
        2,
        "the action is born INSIDE the list"
    );
    assert_eq!(
        d.ir().node(&new_nodes[0]).unwrap().props.get("text"),
        Some(&crawrad_ir::Value::Str(new_nodes[0].clone())),
        "initial label equals name, like Action1 in the classic RAD IDE"
    );
    assert!(d.ir().validate().is_ok());
}

#[test]
fn create_action_with_an_action_selected_uses_its_list() {
    let (mut d, r, a) = with_actions();
    d.select_one("Action1");
    let e = ((by_id("action.new").unwrap()).execute)(&ctx(&d, &r, &a));
    d.apply(e.transaction.unwrap()).unwrap();
    assert_eq!(d.ir().node("ActionList1").unwrap().children.len(), 2);
}

#[test]
fn delete_list_takes_actions_along() {
    // The removal command only accepts leaves: whoever builds the transaction
    // must descend the tree, and Ctrl+Z must bring everything back.
    let (mut d, r, a) = with_actions();
    d.select_one("ActionList1");
    let e = ((by_id("edit.delete").unwrap()).execute)(&ctx(&d, &r, &a));
    d.apply(e.transaction.unwrap()).unwrap();

    assert!(d.ir().node("ActionList1").is_none());
    assert!(d.ir().node("Action1").is_none(), "action went along");
    assert!(d.ir().validate().is_ok());

    d.undo().unwrap();
    assert!(
        d.ir().node("Action1").is_some(),
        "and undo brings both back"
    );
    assert!(d.ir().validate().is_ok());
}

#[test]
fn delete_action_disconnects_whoever_pointed_to_it() {
    // Dangling reference is silent trash: button would keep pointing
    // to an action that does not exist, and would only be discovered during code generation.
    let (mut d, r, a) = with_actions();
    d.select_one("Action1");
    let e = ((by_id("edit.delete").unwrap()).execute)(&ctx(&d, &r, &a));
    d.apply(e.transaction.unwrap()).unwrap();

    assert_eq!(
        d.ir().node("Button1").unwrap().props.get("action"),
        Some(&crawrad_ir::Value::Ref(String::new())),
        "the button remains there, without action"
    );
    assert!(d.ir().validate().is_ok());

    d.undo().unwrap();
    assert_eq!(
        d.ir().node("Button1").unwrap().props.get("action"),
        Some(&crawrad_ir::Value::Ref("Action1".into())),
        "undo reconnects"
    );
}

#[test]
fn create_multiple_standard_actions_in_one_transaction() {
    let (mut d, r, a) = with_actions();
    d.select_one("ActionList1");
    let c = ctx(&d, &r, &a);
    let e = create_standard_actions(&c, &["EditCut", "EditCopy", "EditPaste"]);
    let tx = e
        .transaction
        .expect("must generate transaction with 3 actions");
    let created = e.selection.expect("must select 3 created nodes");
    assert_eq!(created.len(), 3);

    d.apply(tx).unwrap();
    // 1 original + 3 new = 4 actions
    assert_eq!(d.ir().node("ActionList1").unwrap().children.len(), 4);

    let cut_id = &created[0];
    let cut_node = d.ir().node(cut_id).unwrap();
    assert_eq!(
        cut_node.props.get("text"),
        Some(&crawrad_ir::Value::Str("Cu&t".into()))
    );
    assert_eq!(
        cut_node.props.get("shortcut"),
        Some(&crawrad_ir::Value::Str("Ctrl+X".into()))
    );

    // A single undo undoes all 3 created
    d.undo().unwrap();
    assert_eq!(d.ir().node("ActionList1").unwrap().children.len(), 1);
    assert!(d.ir().node(cut_id).is_none());
}

#[test]
fn standard_action_is_born_with_template_name() {
    // In the classic RAD IDE the default action is called EditCopy1, not Action5. With thirty
    // actions in the list, the name is the only thing that distinguishes one from the other —
    // and it is what the generated code will use.
    //
    // **Here it comes out in snake_case**, and the difference is the rule of the house: WHAT the
    // IDE creates is from the classic RAD IDE, HOW it is written is from Rust. Until 17/09/2026 this
    // path passed the RAW template base to `suggest_id_base`, and the list
    // came out with `EditCut1` in the middle of `edit_copy1` — the owner saw it on screen.
    let (mut d, r, a) = with_actions();
    d.select_one("ActionList1");
    let c = ctx(&d, &r, &a);
    let e = create_standard_actions(&c, &["EditCopy", "EditCut", "EditCopy", "Action"]);
    let created = e.selection.clone().expect("must select created nodes");
    d.apply(e.transaction.expect("must generate transaction"))
        .unwrap();

    assert_eq!(
        created,
        vec!["edit_copy1", "edit_cut1", "edit_copy2", "action1"],
        "name comes from template IN SNAKE_CASE; number increments when repeating"
    );
    for id in &created {
        assert!(d.ir().node(id).is_some(), "{id} must exist in document");
    }
}

#[test]
fn dataset_standard_action_is_born_with_its_operation() {
    // A dataset preset is the operation it runs: the node keeps it, and the
    // dataset stays empty for the user to pick in the Inspector.
    let (mut d, r, a) = with_actions();
    d.select_one("ActionList1");
    let c = ctx(&d, &r, &a);
    let e = create_standard_actions(&c, &["DatasetPost", "EditCopy"]);
    let created = e.selection.clone().expect("must select created nodes");
    d.apply(e.transaction.expect("must generate transaction"))
        .unwrap();

    let post = d.ir().node(&created[0]).unwrap();
    assert_eq!(
        post.props.get("dataset_operation"),
        Some(&crawrad_ir::Value::Enum("Post".into()))
    );
    assert_eq!(
        post.props.get("shortcut"),
        Some(&crawrad_ir::Value::Str("Ctrl+Enter".into()))
    );
    assert!(
        !matches!(post.props.get("dataset"), Some(crawrad_ir::Value::Ref(id)) if !id.is_empty()),
        "the preset never picks a dataset"
    );
    assert_eq!(
        d.ir()
            .node(&created[1])
            .unwrap()
            .props
            .get("dataset_operation"),
        Some(&crawrad_ir::Value::Enum("None".into())),
        "a non-dataset preset runs no dataset operation"
    );
}

#[test]
fn standard_action_is_born_with_template_category() {
    // The ActionList editor groups by `category`. If the template brought the
    // category and the node did not store it, the categories panel would be stuck in
    // "(No Category)" forever — which is what happened.
    let (mut d, r, a) = with_actions();
    d.select_one("ActionList1");
    let c = ctx(&d, &r, &a);
    let e = create_standard_actions(&c, &["EditCopy", "Action"]);
    let created = e.selection.clone().expect("must select created nodes");
    d.apply(e.transaction.expect("must generate transaction"))
        .unwrap();

    assert_eq!(
        d.ir().node(&created[0]).unwrap().props.get("category"),
        Some(&crawrad_ir::Value::Str("Edit".into())),
        "EditCopy belongs to Edit group"
    );
    // "(No Category)" is a screen label, not a stored value.
    assert_eq!(
        d.ir().node(&created[1]).unwrap().props.get("category"),
        Some(&crawrad_ir::Value::Str(String::new())),
        "generic action is born without category"
    );
}

#[test]
fn action_list_survives_the_file() {
    // Report from project owner on 31/08/2026: "saved earlier, and the actions
    // tree disappeared — second time today". Actions are CHILDREN of ActionList, and
    // a lost child in round-trip would wipe the entire list in silence.
    // This test walks the whole path: create, apply, write, reopen.
    let (mut d, r, a) = with_actions();
    d.select_one("ActionList1");
    let c = ctx(&d, &r, &a);
    let e = create_standard_actions(&c, &["EditCut", "EditCopy", "EditPaste"]);
    d.apply(e.transaction.expect("must generate transaction"))
        .unwrap();

    let before = d.ir().node("ActionList1").unwrap().children.clone();
    assert_eq!(before.len(), 4, "1 from scenario + 3 created");

    let path = std::env::temp_dir().join("crawrad_actions_round_trip.crawform");
    d.ir().save(&path).expect("write");
    let read = crawrad_ir::Document::load(&path).expect("reopen");
    let _ = std::fs::remove_file(&path);

    read.validate().expect("what was written must be valid");
    assert_eq!(
        read.node("ActionList1").unwrap().children,
        before,
        "ActionList children must return in same order"
    );
    for id in &before {
        let node = read
            .node(id)
            .unwrap_or_else(|| panic!("{id} vanished from file"));
        assert_eq!(node.class, "Action");
    }
    // And reopened document must be identical to what left memory.
    assert_eq!(&read, d.ir(), "round-trip must not lose anything");
}

// ------------------------------------------------------------ the pages

/// A form with a single-page `PageControl` — the state where the "+" of the
/// tab strip is the only path to the second.
///
/// Ids are what `suggest_id` would actually produce (`page_control1`,
/// `tab_sheet1`): that is what makes what is born here measurable instead
/// of guessed.
fn with_pages() -> (Doc, Registry, Clipboard) {
    let (d, r, a) = scenario();
    let mut doc = d.ir().clone();
    let pgc = r
        .instantiate(
            "PageControl",
            "page_control1",
            Rect::new(10.0, 60.0, 320.0, 200.0),
        )
        .unwrap();
    doc.add_child("form_main", pgc).unwrap();
    let mut tab = r
        .instantiate("TabSheet", "tab_sheet1", Rect::new(0.0, 24.0, 320.0, 176.0))
        .unwrap();
    tab.set_text("Page1");
    doc.add_child("page_control1", tab).unwrap();
    doc.validate().expect("scenario must be valid");
    (Doc::new(doc), r, a)
}

#[test]
fn new_page_is_born_and_opens_in_the_same_transaction() {
    // The reason this action exists, and why it is not two calls:
    // creating tab and opening it are ONE undo step. With two actions, the
    // first Ctrl+Z would close the newly created page and second would delete it —
    // and in between the form would show a tab nobody asked for.
    let (mut d, r, a) = with_pages();
    d.select_one("page_control1");
    let agent = Author::Agent("claude".into());
    let c = ctx_from(&d, &r, &a, Args::none(), agent.clone());
    let e = (by_id("page.new").unwrap().execute)(&c);

    let tx = e.transaction.expect("creating page produces transaction");
    assert_eq!(tx.author, agent, "who requested signs (ADR 0005)");
    assert_eq!(tx.commands.len(), 2, "one batch = one Ctrl+Z");
    assert_eq!(
        e.selection.as_deref(),
        Some(&["tab_sheet2".to_string()][..]),
        "what is born remains selected, like when dropping from palette"
    );

    d.apply(tx).expect("transaction must be applicable");
    let ir = d.ir();
    let pgc = ir.node("page_control1").unwrap();
    assert_eq!(pgc.children, vec!["tab_sheet1", "tab_sheet2"]);
    let new_tab = ir.node("tab_sheet2").expect("new page is in doc");
    assert_eq!(new_tab.class, "TabSheet");
    assert_eq!(
        new_tab.text(),
        Some("Page2"),
        "label follows classic RAD IDE"
    );
    assert_eq!(
        pgc.props.get("active_page"),
        Some(&Value::Int(1)),
        "the new page OPENS — creating and remaining looking at previous is invisible work"
    );
}

#[test]
fn new_page_only_exists_with_a_page_control_in_sight() {
    // Honest `enabled`: the menu cannot offer `New page` with a
    // button selected, and across the protocol refusal must say
    // WHY — an agent has no tab strip to click.
    let (mut d, r, a) = with_pages();
    let action = by_id("page.new").unwrap();

    d.select_one("Button1");
    let c = ctx(&d, &r, &a);
    assert!(!action.enabled.allows(&c), "Button does not receive page");
    let e = (action.execute)(&c);
    assert!(
        e.transaction.is_none(),
        "and executing anyway does not mutate"
    );
    assert_eq!(
        e.status.as_deref(),
        Some("`Button1` is a Button, not a PageControl")
    );

    // Selected PAGE counts for its owner: clicking a tab and asking for another
    // is natural gesture, same rule as `action.new` with `ActionList`.
    d.select_one("tab_sheet1");
    let c = ctx(&d, &r, &a);
    assert!(action.enabled.allows(&c));
    let tx = (action.execute)(&c).transaction.expect("owner was found");
    assert!(
        matches!(&tx.commands[0], Command::AddNode { parent, .. } if parent == "page_control1"),
        "page enters PageControl, not TabSheet"
    );

    // And with no selection there is no target — neither for menu, nor for agent.
    d.select(Vec::new());
    let c = ctx(&d, &r, &a);
    assert!(!action.enabled.allows(&c));
    assert_eq!(
        (action.execute)(&c).status.as_deref(),
        Some("new page: select a PageControl, or name one in `control`")
    );
}

#[test]
fn specified_control_overrides_selection() {
    // This is how the "+" on the tab strip calls the action: clicked control may
    // not be selected, and whoever specifies the node must override response. It is
    // also the only way the other side of protocol has to choose —
    // there is no window there.
    let (mut d, r, a) = with_pages();
    d.select_one("Button1");
    let args = Args::text("control", "page_control1").and("text", Value::Str("  Settings ".into()));
    let c = ctx_with(&d, &r, &a, args);
    let action = by_id("page.new").unwrap();
    assert!(
        action.enabled.allows(&c),
        "with specified control, selection does not matter"
    );
    let tx = (action.execute)(&c)
        .transaction
        .expect("produces transaction");
    match &tx.commands[0] {
        Command::AddNode { parent, node, .. } => {
            assert_eq!(parent, "page_control1");
            // Argument label rules, trimmed: agent creating "Settings" tab
            // should not have to call `component.set_property` immediately after just to name it.
            assert_eq!(node.text(), Some("Settings"));
        }
        other => panic!("first command had to create page: {other:?}"),
    }

    // And an id that does not exist refuses with its name in the sentence.
    let c = ctx_with(&d, &r, &a, Args::text("control", "does_not_exist"));
    assert_eq!(
        (action.execute)(&c).status.as_deref(),
        Some("no such component: `does_not_exist`")
    );
}

#[test]
fn page_counts_pages_and_not_children() {
    // `active_page` is an index in the `TabSheet` list, and `PageControl` can
    // hold children that are not pages. Counting `children.len()` would open wrong
    // tab — and defect would only appear in form having both.
    let (mut d, r, a) = with_pages();
    let mut doc = d.ir().clone();
    let extra = r
        .instantiate("Timer", "timer1", Rect::new(0.0, 0.0, 28.0, 28.0))
        .expect("Timer is in catalogue");
    doc.add_child("page_control1", extra).unwrap();
    doc.validate().unwrap();
    d = Doc::new(doc);
    d.select_one("page_control1");

    let c = ctx(&d, &r, &a);
    let tx = (by_id("page.new").unwrap().execute)(&c)
        .transaction
        .expect("produces transaction");
    assert!(
        matches!(&tx.commands[0], Command::AddNode { index, .. } if *index == 2),
        "page enters end of CHILDREN"
    );
    assert!(
        matches!(&tx.commands[1], Command::SetProp { value, .. } if *value == Some(Value::Int(1))),
        "but its index is among the PAGES"
    );
    d.apply(tx).expect("applicable");
    assert_eq!(d.ir().node("tab_sheet2").unwrap().text(), Some("Page2"));
}

#[test]
fn page_publishes_signature_that_mcp_reads() {
    // The signature IS the MCP tool: `Param::name` that catalogue
    // announces must be the key that `execute` reads, otherwise agent sends
    // right argument with wrong name and gets no error.
    //
    // And both are OPTIONAL on purpose: that is what keeps action in `Edit`
    // menu, alongside `New action`, instead of outside it by demanding an id
    // that no menu knows how to type (see `Action::requires_argument`).
    let a = by_id("page.new").expect("`page.new` is in catalogue");
    let published: Vec<&str> = a.params.iter().map(|p| p.name).collect();
    assert_eq!(published, ["control", "text"]);
    assert!(
        !a.requires_argument(),
        "requiring argument would take `New page` out of menu"
    );
    assert!(!a.hint.is_empty(), "without hint it does not become a tool");
    for p in a.params {
        assert!(
            !p.description.is_empty(),
            "`page.new.{}` without description does not become tool parameter",
            p.name
        );
    }
    assert_eq!(a.group, "Edit", "is the group drawn by menu");
}

#[test]
fn action_with_argument_executes_through_catalogue() {
    // Declared hole of ADR 0005: `action.standard` existed in catalogue
    // returning `Effect::none()` while work lived in loose function.
    // Catalogue is MCP source, so it stated IDE knew how to do this and
    // calling agent got nothing. Now it executes via same path as all others.
    let (mut d, r, a) = with_actions();
    d.select_one("ActionList1");

    let action = by_id("action.standard").expect("is in catalogue");
    assert!(!action.params.is_empty(), "and declares what it receives");
    assert!(action.params[0].list, "receives multiple at once");

    let c = ctx_with(
        &d,
        &r,
        &a,
        Args::texts("classes", ["EditCopy", "EditPaste"]),
    );
    assert!(action.enabled.allows(&c));
    let effect = (action.execute)(&c);
    let tx = effect
        .transaction
        .expect("catalogue action must produce effect");
    let created = effect.selection.expect("and select what was created");
    // In snake_case: see `standard_action_is_born_with_template_name`.
    assert_eq!(created, vec!["edit_copy1", "edit_paste1"]);

    d.apply(tx).unwrap();
    assert_eq!(d.ir().node("ActionList1").unwrap().children.len(), 3);
}

#[test]
fn action_with_argument_without_argument_does_nothing() {
    // Calling without parameter cannot invent: better to do nothing than
    // create an action nobody asked for.
    let (mut d, r, a) = with_actions();
    d.select_one("ActionList1");
    let action = by_id("action.standard").unwrap();
    let c = ctx(&d, &r, &a);
    let e = (action.execute)(&c);
    assert!(e.transaction.is_none() && e.selection.is_none());
}

#[test]
fn catalogue_does_not_know_the_interface() {
    // Rule sustaining ADR 0005: if an action could mention egui, it
    // would cease being executable by an agent — and catalogue would have become just
    // button rearrangement. Checked by dependency graph.
    let deps = include_str!("../Cargo.toml");
    for forbidden in ["egui", "eframe", "winit", "windows", "runtime-win32"] {
        assert!(
            !deps.contains(forbidden),
            "crawrad-actions cannot depend on {forbidden}"
        );
    }
}

#[test]
fn run_generates_beforehand_and_is_thus_a_single_action() {
    // The `Run` of the classic RAD IDE compiles what is on screen, not what remains on
    // disk. Here it is a single action, and shell knows the order — action
    // does not touch files (ADR 0005).
    let a = by_id("project.run").unwrap();
    assert_eq!(
        a.shortcut,
        Some("F5"),
        "default scheme is Modern (ADR 0011)"
    );
    // `Run`, not `File`: decision of owner on 15/09/2026 — menu follows
    // classic RAD IDE, where Run lives in Run menu.
    assert_eq!(a.group, "Run");
    let (d, r, actions) = scenario();
    let e = (a.execute)(&ctx(&d, &r, &actions));
    assert_eq!(e.file, Some(FileEffect::Run));
    assert!(e.transaction.is_none(), "running does not alter document");
}

#[test]
fn run_is_not_offered_when_there_is_nothing_to_run() {
    // Measured defect, and Amendment 3 of ADR 0018: in a `--lib` project build
    // passed and `cargo run -p <lib>` replied `error: a bin target must be
    // available for cargo run`. IDE promised its central gesture and
    // delivered tool error.
    //
    // Predicate belongs to CATALOGUE and not shell (ADR 0005): thus bar,
    // menu, `F5` and MCP all reach same answer, instead of three `if`
    // that might disagree.
    let a = by_id("project.run").unwrap();
    let (d, r, actions) = scenario();
    let mut c = ctx(&d, &r, &actions);

    assert!(
        a.enabled.allows(&c),
        "with executable target `Run` remains lit — this is normal case"
    );

    c.project = Project {
        has_binary: false,
        ..Project::default()
    };
    assert!(
        !a.enabled.allows(&c),
        "`Run` was offered in project without executable target: it is `cargo run` of \
         a library, which answers `a bin target must be available`"
    );

    // And it is ONLY `Run`. `Generate` writes `src/`, and library has `src/`;
    // `Open code` opens folder, and folder exists. Disabling all three together because
    // one does not apply would take from library what it always had.
    for id in ["project.generate", "project.open_editor"] {
        let other = by_id(id).unwrap();
        assert!(
            other.enabled.allows(&c),
            "`{id}` went out along with `Run` — it does not depend on executable target"
        );
    }
}

#[test]
fn not_knowing_is_not_the_same_as_knowing_it_is_not() {
    // Default of `Project` is OPEN, and that is a decision not carelessness: when
    // `cargo` could not be asked — outside PATH, broken manifest,
    // no folder open yet — IDE does not invent verdict. Leaves
    // button as it always was, and whoever explains what happened is `cargo` itself
    // in status bar.
    //
    // Closing due to lack of answer would trade readable error for dead
    // button that cannot be read, in a frequent case: first run, before
    // any folder.
    assert!(
        Project::default().has_binary,
        "default closed: project about which nothing is known is born without `Run`, and \
         that is how first execution of IDE would end up with dead button"
    );
    let (d, r, actions) = scenario();
    assert!(
        by_id("project.run")
            .unwrap()
            .enabled
            .allows(&ctx(&d, &r, &actions))
    );
}

// ---------------------------------------------------------------- paths

/// The hole table, and it is the entire test.
///
/// Each line is a way of writing a path; the second column states whether it
/// can become an open file. First four `false` are what previous
/// validation — `is_absolute()` plus `..` — allowed through: on Windows
/// `is_absolute()` requires prefix **and** root, and `PathBuf::join` DISCARDS root
/// passed by shell when path has only one of the two.
///
/// Table is not the rule: rule is refusing component that re-anchors. Table
/// is proof that rule covers forms someone types.
const PATHS: &[(&str, bool)] = &[
    // --- those that enter
    ("src/main.rs", true),
    (r"src\main.rs", true),
    ("Cargo.toml", true),
    // `.` and a `..` that does not ascend too far are legitimate paths, and
    // normalization erases them.
    ("./src/main.rs", true),
    ("src/../src/main.rs", true),
    // --- those that old validation already refused
    ("../../../Windows/System32/drivers/etc/hosts", false),
    (r"..\..\..\Windows\System32\drivers\etc\hosts", false),
    (r"C:\Windows\System32\drivers\etc\hosts", false),
    // --- those it let through, and why REJECT arrived
    (r"\Windows\System32\drivers\etc\hosts", false),
    ("/Windows/System32/drivers/etc/hosts", false),
    ("C:something.txt", false),
    (r"\\server\share\x.rs", false),
    (r"\\?\C:\Windows\x", false),
    // --- and degenerates
    ("", false),
    (".", false),
    ("..", false),
    ("src/../..", false),
];

#[test]
fn only_paths_staying_inside_project_are_accepted() {
    for (raw, accepts) in PATHS {
        let out = PathInProject::new(raw);
        assert_eq!(
            out.is_some(),
            *accepts,
            "`{raw}` should {}: got {out:?}",
            if *accepts { "enter" } else { "be refused" }
        );
    }
}

#[test]
fn validated_path_does_not_reanchor_when_joined() {
    // The property that truly matters, stated without depending on table: what
    // leaves `new` ALWAYS stays under root after `join`. That is what
    // shell does with it on next line, and was what was broken.
    //
    // `starts_with` alone would not suffice as VALIDATION — it is lexical, and
    // `<root>/../..` starts at `<root>` with `..` still inside. Since here
    // `..` was already resolved by `new`, it serves as VERIFICATION.
    let root = std::path::Path::new(if cfg!(windows) {
        r"F:\project\generated"
    } else {
        "/project/generated"
    });
    for (raw, _) in PATHS {
        let Some(c) = PathInProject::new(raw) else {
            continue;
        };
        let full = root.join(c.as_str());
        assert!(
            full.starts_with(root),
            "`{raw}` passed validation and escaped: {}",
            full.display()
        );
        assert!(
            !full
                .components()
                .any(|k| matches!(k, std::path::Component::ParentDir)),
            "`{raw}` passed with unresolved `..`: {}",
            full.display()
        );
    }
}

#[test]
fn normalization_returns_clean_path() {
    // Not cosmetic: shell compares and displays what comes out here, and stray `.` and `..`
    // would make two forms of same file look like two files.
    let clean = |s: &str| PathInProject::new(s).map(|c| c.as_str().replace('\\', "/"));
    assert_eq!(clean("./src/main.rs").as_deref(), Some("src/main.rs"));
    assert_eq!(clean("src/../src/main.rs").as_deref(), Some("src/main.rs"));
    assert_eq!(clean("src/main.rs").as_deref(), Some("src/main.rs"));
}

#[test]
fn open_in_editor_without_argument_opens_folder() {
    // Menu `Run` calls SAME action without any argument, and that was why
    // optional parameter existed to avoid breaking. Empty `filter` is
    // inside here: empty string is absence of argument, not path.
    let a = by_id("project.open_editor").unwrap();
    let (d, r, actions) = scenario();
    for args in [Args::none(), Args::text("path", "")] {
        let e = (a.execute)(&ctx_with(&d, &r, &actions, args));
        assert_eq!(e.file, Some(FileEffect::OpenInEditor(None)));
        assert!(e.status.is_none(), "opening folder is no case for warning");
    }
}

#[test]
fn open_in_editor_with_internal_path_carries_the_path() {
    let a = by_id("project.open_editor").unwrap();
    let (d, r, actions) = scenario();
    let e = (a.execute)(&ctx_with(
        &d,
        &r,
        &actions,
        Args::text("path", "src/form_main.rs"),
    ));
    let Some(FileEffect::OpenInEditor(Some(c))) = e.file else {
        panic!("expected path in effect, got {:?}", e.file);
    };
    assert_eq!(c.as_str().replace('\\', "/"), "src/form_main.rs");
}

#[test]
fn open_in_editor_refuses_escaping_project_in_the_action_itself() {
    // HERE, not in shell: `Effect` is `pub` and catalogue is surface of
    // MCP (ADR 0005). A second applier — MCP server, test,
    // next shell — would receive raw text if refusal lived only in
    // `crawrad-ide`. Effect of refused path has NO `file`: nothing
    // to apply, and `status` is what appears in bar.
    let a = by_id("project.open_editor").unwrap();
    let (d, r, actions) = scenario();
    for (raw, accepts) in PATHS {
        if *accepts || raw.is_empty() {
            continue;
        }
        let e = (a.execute)(&ctx_with(&d, &r, &actions, Args::text("path", *raw)));
        assert!(
            e.file.is_none(),
            "`{raw}` became open file effect: {:?}",
            e.file
        );
        assert!(
            e.status.is_some_and(|s| s.starts_with("refused:")),
            "`{raw}` was refused in silence"
        );
    }
}

#[test]
fn open_editor_parameter_is_declared_for_mcp() {
    // Catalogue is signature agent reads (ADR 0005). Parameter that
    // action uses and does not declare is MCP tool nobody knows how to call.
    let a = by_id("project.open_editor").unwrap();
    assert_eq!(a.params.len(), 1);
    assert_eq!(a.params[0].name, "path");
    assert!(!a.params[0].list, "is a single path, not a list");
    assert!(
        !a.params[0].description.is_empty(),
        "description is what agent reads to know what to send"
    );
}

/// The actions that **do not speak of the form**, and therefore remain valid without
/// an open document. That is why `Context` carries the document in an
/// `Option`.
///
/// Three are about the folder. The fourth is about **compilation**: a console
/// project created by `New Project…` has no form and fails in the
/// same way, and tying the build panel to the document would wipe the compilation
/// error precisely for whoever only has code.
const WITHOUT_DOCUMENT: &[&str] = &[
    // CREATING A PROJECT does not depend on having an open one — it is precisely what is done
    // when there is none.
    "project.new",
    // Creating a component is about the project (or the user folder), not the form
    "file.new_component",
    // Adding a project or choosing the active one is about the workspace
    "project.add",
    "project.add_existing",
    "project.set_active",
    "project.open_editor",
    // The Welcome page's commands are what is done with no form open
    "project.open_folder",
    "file.open_form",
    "project.forget_recent",
    "file.recover_unsaved",
    "file.discard_recovery",
    "help.welcome",
    // Checking the machine for Rust and its build tools speaks of the machine, not the form.
    "help.environment",
    // OPENING A FILE IN A TAB also does not speak of the form: the Explorer
    // lists the MEMBER files, and a console project created by
    // `New Project…` has no form and has `src/main.rs` to open.
    // Tying it to the document would leave the panel with lines that do not respond
    // precisely for whoever only has code.
    "editor.open",
    // UI ZOOM is valid in a newly opened IDE: enlarging menu and panels
    // is useful before having a form, and is when someone arranges the screen to
    // begin. It is opposite of `view.zoom_in`, which enlarges the DESIGN — without
    // a design it is an active command that does nothing.
    "view.ui_zoom_in",
    "view.ui_zoom_out",
    "view.ui_zoom_reset",
    "project.generate",
    "project.run",
    "project.run_language",
    "view.toggle_problems",
    // THE INLAY HINTS are code, not form: a console project has them too.
    "editor.toggle_inlay_hints",
    // GOING TO A LINE is code: a console project has lines too.
    // The code editor's commands left (capability D7): they need the code
    // editor in front — see `capability_matrix/matrix.golden`.
    // GOING TO A DEFINITION is code too.
    "editor.apply_code_action",
    // the teacher acts on the code file's diagnostic, never on a form
    "chat.fix_explain",
    // THE LANGUAGE SERVER is the workspace's, not the form's.
    "lsp.reload_workspace",
    "lsp.restart",
    "lsp.stop",
    // THE COMPONENT SUITE does not speak of the form: installing a package is
    // precisely what is done BEFORE having one, and seeing what is installed
    // (with refusals) is the first thing to look at when a component does not
    // appear in the palette of a newly opened IDE.
    "component.install",
    // BREAKPOINT IS A LINE OF CODE, not of form: a console project
    // without any form is debugged the same way.
    // DEBUGGER GESTURES likewise do not speak of the form: a console
    // program is debugged the same way.
    // Step, Pause and Stop need a debug SESSION, not a form: see
    // `the_debugger_commands_follow_the_session`.
    "project.run_without_debugging",
    "debug.add_watch",
    "debug.remove_watch",
    "component.installed",
    // THE EXPLORER is about the project's folder, not the form: a console
    // project has a tree and no form (`the_explorer_needs_a_project_not_a_form`).
    "explorer.toggle_folder",
    "explorer.toggle_legend",
    "explorer.toggle_open_editors",
    "explorer.toggle_outline",
    "explorer.toggle_timeline",
    "explorer.open_timeline_entry",
    "explorer.toggle_dependencies",
    "explorer.new_file",
    "explorer.new_form",
    "explorer.new_folder",
    "explorer.open",
    "explorer.reveal",
    "explorer.copy_path",
    "explorer.copy_rel_path",
    "explorer.rename",
    "explorer.delete",
    "explorer.refresh",
    "explorer.collapse_all",
    // QUICK OPEN (File ▸ Quick Open...): searching files in project is valid without form.
    "file.quick_open",
    // COMMAND PALETTE (View ▸ Command Palette...): searching capabilities and commands is valid without form.
    "view.command_palette",
    // CREATE NEW FORM OR FILE (File ▸ New Form... / New File...):
    // Does not depend on having open form — can be created in empty or console project.
    "file.new_form",
    // It needs a form of the PROJECT to inherit from, not an open one
    "file.new_inherited_form",
    "file.new_file",
    // IDE OPTIONS (Tools ▸ Options...): configuring IDE, themes, grid,
    // targets and ACP agents is valid before opening any form.
    "tools.options",
    // BUILD MANAGER (Project ▸ Build...): compiling targets and configuring profiles is valid without form.
    "project.build_manager",
    // PROJECT OPTIONS (Project > Options...): the title and icon are the project's, not a form's.
    "project.options",
    // MCP MANAGER (Tools ▸ MCP Servers...): configuring MCP servers is valid without form.
    "tools.mcp",
    // AI ASSISTANT INSTRUCTIONS (Tools ▸ AI Custom Instructions...): valid without form.
    "tools.ai_memory",
    // DECOUPLED CODE NOTES (Tools ▸ Sidecar Code Note...): opening or creating
    // decoupled annotations is valid before opening form (in code editor or project).
    "tools.sidecar_note",
    // PANEL ARRANGEMENT (ADR 0014, Amendment 9). The whole family is about the
    // WINDOW, and none of them look at the document: moving Inspector to the
    // right is valid in newly opened IDE, without folder and without form — which is
    // precisely when someone arranges the screen before starting.
    //
    // Written by prefix below, not one by one: new arrangement action
    // is born covered instead of failing a test that does not speak of it.
];

/// An action **about the window**, and not about the form.
///
/// `WITHOUT_DOCUMENT` above is a closed list on purpose — each entry is
/// an exception that someone had to write. This prefix is the opposite: the
/// arrangement family is closed by rule (`group: "View"`, predicate
/// `|_| true`, `Effect::layout`), so checking it item-by-item would only produce a
/// second list to diverge from the first.
fn is_about_the_window(id: &str) -> bool {
    id.starts_with("layout.")
}

#[test]
fn without_document_only_folder_actions_remain_enabled() {
    // Since ADR 0017 the IDE exists without a document — first run, folder
    // without form, pending choice. The shell did not construct `Context`
    // in that state and the ENTIRE catalogue died in silence along with it, including the
    // three actions that need no document at all: in a console project,
    // created by `New Project…` itself, `Open code` did nothing and gave
    // no warning.
    //
    // This test guards both sides of the boundary simultaneously.
    let reg = Registry::builtin();
    let clipboard = Clipboard::default();
    let c = ctx_without_document(&reg, &clipboard);

    for a in catalogue() {
        let enabled = a.enabled.allows(&c);
        if WITHOUT_DOCUMENT.contains(&a.id) || is_about_the_window(a.id) {
            assert!(
                enabled,
                "`{}` does not speak of the form and must be valid without an open one",
                a.id
            );
        } else {
            assert!(
                !enabled,
                "`{}` is about the FORM and cannot act without one",
                a.id
            );
        }
    }
}

#[test]
fn the_debugger_commands_follow_the_session() {
    let reg = Registry::builtin();
    let clipboard = Clipboard::default();
    let mut c = ctx_without_document(&reg, &clipboard);
    let on = |c: &Context, id: &str| {
        catalogue()
            .iter()
            .any(|a| a.id == id && a.enabled.allows(c))
    };
    let step = ["debug.step_over", "debug.step_into", "debug.step_out"];
    for (session, stepping, pause, stop) in [
        (DebugSession::None, false, false, false),
        (DebugSession::Running, false, true, true),
        (DebugSession::Stopped, true, false, true),
    ] {
        c.project.debug = session;
        for id in step {
            assert_eq!(on(&c, id), stepping, "{id} with {session:?}");
        }
        assert_eq!(on(&c, "debug.pause"), pause, "pause with {session:?}");
        assert_eq!(on(&c, "debug.stop"), stop, "stop with {session:?}");
    }
}

#[test]
fn generate_needs_a_form_in_the_project() {
    let reg = Registry::builtin();
    let clipboard = Clipboard::default();
    let mut c = ctx_without_document(&reg, &clipboard);
    let generate = catalogue().into_iter().find(|a| a.id == "project.generate");
    let generate = generate.expect("project.generate is in the catalogue");
    assert!(generate.enabled.allows(&c));
    c.project.has_forms = false;
    assert!(
        !generate.enabled.allows(&c),
        "a console project has nothing to generate"
    );
}

#[test]
fn without_document_no_form_action_produces_effect() {
    // `enabled` above is the INTERFACE gate. This is the MCP gate: the
    // catalogue is also the agent surface (ADR 0005), and an applier on the
    // other side might call `execute` without checking the predicate. No
    // form action may return a transaction with the document absent —
    // it would have no node to land on.
    let reg = Registry::builtin();
    let clipboard = Clipboard::default();
    let c = ctx_without_document(&reg, &clipboard);

    for a in catalogue()
        .iter()
        .filter(|a| !WITHOUT_DOCUMENT.contains(&a.id) && !is_about_the_window(a.id))
    {
        let e = (a.execute)(&c);
        assert!(
            e.transaction.is_none(),
            "`{}` produced transaction without open document",
            a.id
        );
        assert!(
            e.selection.as_ref().map(|s| s.is_empty()).unwrap_or(true),
            "`{}` selected nodes of a nonexistent document",
            a.id
        );
    }
}

#[test]
fn without_document_open_editor_still_opens_group_folder() {
    // User case: open console project, no form, and they
    // ask for code. Without argument it is the FOLDER — and folder exists even without
    // document, because what defines it is the open root (ADR 0017).
    let reg = Registry::builtin();
    let clipboard = Clipboard::default();
    let c = ctx_without_document(&reg, &clipboard);

    let e = (by_id("project.open_editor").unwrap().execute)(&c);
    assert_eq!(e.file, Some(FileEffect::OpenInEditor(None)));

    // And refusal of external path remains valid without document: it is
    // about the SHAPE of the path, not about form content — if it
    // depended on document, the state without document would be a hole in
    // validation reached by MCP.
    let c = Context {
        doc: None,
        registry: &reg,
        clipboard: &clipboard,
        args: Args::text("path", "../outside.rs"),
        project: Project::default(),
        author: Author::User,
        editor_dirty: false,
        text: Default::default(),
    };
    let e = (by_id("project.open_editor").unwrap().execute)(&c);
    assert!(
        e.status.is_some_and(|s| s.starts_with("refused:")),
        "path outside project must be refused even without document"
    );
}

#[test]
fn f12_switches_view_and_touches_no_file() {
    // The `F12` of the classic RAD IDE enters through catalogue, not through an `if` in keyboard
    // handling (ADR 0005 and ADR 0011). Two things must be true at the same
    // time, and both are guarded by this test:
    //
    // 1. the effect is of VIEW. If it came as `File`, an agent requesting
    //    the effect of a tab switch would receive the response of what writes to
    //    disk — and shell would apply both families through the same arm;
    // 2. `is_empty()` sees it. It is the sieve deciding what is not worth
    //    pushing forward; a new field it did not look at would cause the whole
    //    action to pass as "did nothing".
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);
    let action = by_id("view.toggle_code").expect("F12 must be in catalogue");
    assert_eq!(action.shortcut, Some("F12"));

    let e = (action.execute)(&c);
    assert_eq!(e.view, Some(ViewEffect::ToggleDesignCode));
    assert_eq!(
        e.file, None,
        "switching tab neither reads nor writes anything"
    );
    assert!(
        e.transaction.is_none(),
        "switching tab does not alter document"
    );
    assert!(!e.is_empty(), "`is_empty()` is not inspecting the view");
}

#[test]
fn welcome_is_in_help_switches_view_and_does_not_touch_document() {
    // The `Help ▸ Welcome` of modern editors, and it is a catalogue action for a reason of
    // reach: the welcome screen had NO way back at all — the
    // center only draws it without a document, and IDE reopens the last folder by itself.
    // Entering through catalogue (ADR 0005), it is born with menu, label, hint and shortcut
    // slot already prepared, and MCP sees it along with it.
    //
    // Three things at once, each guarding against a different failure
    // mode:
    //
    // 1. effect is of VIEW. Coming as `File`, an agent reading `Effect`
    //    to know what will happen would read "will write to disk";
    // 2. **no** transaction: switching what is on screen does not alter the
    //    form, and a step in Ctrl+Z here would undo nothing;
    // 3. `is_empty()` sees it — sieve deciding what is not worth pushing
    //    forward already inspects view, which makes shell apply it.
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);
    let action = by_id("help.welcome").expect("`Welcome` must be in catalogue");
    assert_eq!(action.group, "Help", "modern editors put it in Help menu");

    let e = (action.execute)(&c);
    assert_eq!(e.view, Some(ViewEffect::ToggleWelcome));
    assert_eq!(
        e.file, None,
        "showing screen neither reads nor writes anything"
    );
    assert!(e.transaction.is_none(), "and does not alter document");
    assert!(!e.is_empty(), "`is_empty()` is not inspecting the view");

    // And it is the same key both ways, like `F12` and the problems
    // panel: a `Show` would require a second command to return, and the
    // second is what nobody finds.
    let again = (action.execute)(&c);
    assert_eq!(
        again.view, e.view,
        "action toggles: returns same request, and shell owns the state"
    );
}

#[test]
fn without_open_form_the_welcome_page_still_opens() {
    // With no form the Welcome page is the centre, and asking for it
    // again shows it again (fresh rows) — never a dead command.
    let reg = Registry::builtin();
    let clipboard = Clipboard::default();
    let c = ctx_without_document(&reg, &clipboard);
    assert!(by_id("help.welcome").unwrap().enabled.allows(&c));
}

/// `project.forget_recent` needs the folder: without one it refuses aloud
/// instead of guessing which row was meant.
#[test]
fn forget_recent_refuses_without_a_path() {
    let reg = Registry::builtin();
    let clipboard = Clipboard::default();
    let c = ctx_without_document(&reg, &clipboard);
    let e = (by_id("project.forget_recent").unwrap().execute)(&c);
    assert!(e.file.is_none());
    assert!(
        e.status
            .as_deref()
            .is_some_and(|s| s.starts_with("refused")),
        "{:?}",
        e.status
    );
}

#[test]
fn lock_controls_is_designer_mode_and_does_not_touch_document() {
    // The `Edit ▸ Lock Controls` of the classic RAD IDE. Three things at once, each
    // guarding against a different failure mode:
    //
    // 1. effect is of DESIGNER. Coming as `View`, shell would apply it via
    //    the arm that changes what is in the center of the window, and an agent reading
    //    `Effect` to know what will happen would read "will change screen";
    // 2. **no** transaction. It is state of editor, not of form:
    //    `.crawform` and `.designer.rs` come out identical locked or unlocked,
    //    and a transaction here would appear in Ctrl+Z as a step that changed
    //    nothing — and would travel to disk on next `Save`;
    // 3. `is_empty()` sees it. It is the sieve deciding what is not worth
    //    pushing forward; a field it did not look at would cause the whole
    //    command to pass as "did nothing".
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);
    let action = by_id("edit.lock_controls").expect("lock must be in catalogue");
    assert_eq!(
        action.group, "Edit",
        "in the classic RAD IDE it lives in Edit menu"
    );

    let e = (action.execute)(&c);
    assert_eq!(e.designer, Some(DesignerMode::ToggleLock));
    assert!(e.transaction.is_none(), "lock does not enter document");
    assert_eq!(e.view, None, "center of window does not change");
    assert_eq!(e.file, None, "neither reads nor writes anything");
    assert!(
        !e.is_empty(),
        "`is_empty()` is not inspecting designer mode"
    );
}

#[test]
fn without_form_there_is_no_control_to_lock() {
    // An active switch in an IDE without a form is a switch turned on
    // without effect — and which, in first session WITH form, makes person
    // conclude dragging broke by itself.
    let (r, a) = (Registry::builtin(), Clipboard::default());
    let c = ctx_without_document(&r, &a);
    let action = by_id("edit.lock_controls").unwrap();
    assert!(!action.enabled.allows(&c));
}

#[test]
fn canvas_zoom_does_not_touch_document() {
    // ADR 0019: zoom is state of editor. If it produced transaction,
    // `.crawform` would change revision just because someone zoomed in view — and
    // `Ctrl+S` would have work to save after a gesture that touched
    // no component.
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);
    for (id, expected) in [
        ("view.zoom_in", Zoom::In),
        ("view.zoom_out", Zoom::Out),
        ("view.zoom_reset", Zoom::Reset),
    ] {
        let action = by_id(id).unwrap_or_else(|| panic!("{id} must be in catalogue"));
        assert_eq!(action.group, "View", "{id} switches neither file nor tab");
        let e = (action.execute)(&c);
        assert_eq!(e.zoom, Some(expected));
        assert!(e.transaction.is_none(), "{id} does not enter document");
        assert!(e.selection.is_none(), "{id} does not modify selection");
        assert_eq!(e.file, None, "{id} neither reads nor writes anything");
        assert_eq!(e.view, None, "{id} does not change center");
        assert!(!e.is_empty(), "`is_empty()` is not inspecting zoom");
    }
}

#[test]
fn without_form_there_is_no_canvas_to_zoom() {
    // Same rule as `lock_controls`: active inert command is invitation to
    // conclude IDE is broken.
    let (r, a) = (Registry::builtin(), Clipboard::default());
    let c = ctx_without_document(&r, &a);
    for id in ["view.zoom_in", "view.zoom_out", "view.zoom_reset"] {
        assert!(!by_id(id).unwrap().enabled.allows(&c), "{id}");
    }
}

// ------------------------------------------------- argument has name and type

#[test]
fn argument_is_named_and_name_is_what_mcp_publishes() {
    // The key that `execute` reads is `Param::name` that catalogue announces.
    // With positional list that was impossible to guarantee: position only
    // existed inside function body, and callers guessed from outside.
    let (mut d, r, a) = with_actions();
    d.select_one("ActionList1");
    let action = by_id("action.standard").unwrap();
    let name = action.params[0].name;

    // with right name, creates
    let c = ctx_with(&d, &r, &a, Args::texts(name, ["EditCopy"]));
    assert!((action.execute)(&c).transaction.is_some());

    // with other name, invents nothing — argument did not arrive
    let c = ctx_with(&d, &r, &a, Args::texts("classe", ["EditCopy"]));
    assert!(
        (action.execute)(&c).transaction.is_none(),
        "argument with wrong name cannot be read by mistake"
    );
}

#[test]
fn args_carries_what_vec_string_could_not_carry() {
    // Measured block: `component.add(class, x, y)` — the literal example of
    // ADR 0005 — did not fit in `Vec<String>`, because `x` and `y` are `f32`.
    // This test is proof it fits now, which is why it exists
    // before the action.
    let args = Args::text("class", "Button")
        .and("x", crawrad_ir::Value::Float(10.5))
        .and("y", crawrad_ir::Value::Int(20))
        .and("flag", crawrad_ir::Value::Bool(true));

    assert_eq!(args.as_text("class"), Some("Button"));
    assert_eq!(args.as_number("x"), Some(10.5));
    // Across protocol, JSON sends `20` for a field that is float;
    // accepting `Int` where float is expected avoids error that is not error.
    assert_eq!(args.as_number("y"), Some(20.0));
    assert_eq!(args.as_bool("flag"), Some(true));
    assert_eq!(args.as_bool("other"), None);
    assert_eq!(args.as_bool("class"), None);

    // And what is NOT there does not become value: `None` is absence, not zero.
    assert_eq!(args.as_number("w"), None);
    assert_eq!(args.as_text("x"), None, "number is not read as text");
    assert!(Args::none().is_empty());
    assert!(!args.is_empty());
}

#[test]
fn text_list_only_reads_list() {
    // Parameter `list: true` receives `Value::List`. Loose text is NOT
    // a list of one: accepting both would make published signature worth
    // less than what action actually accepts.
    let args = Args::texts("classes", ["EditCopy", "EditPaste"]);
    assert_eq!(args.as_texts("classes"), vec!["EditCopy", "EditPaste"]);
    assert!(
        Args::text("classes", "EditCopy")
            .as_texts("classes")
            .is_empty()
    );
    assert!(args.as_texts("other").is_empty());
}

// --------------------------------------------------------------- audit

#[test]
fn context_author_signs_transaction() {
    // `Author` was in `crawrad-doc` from first day without a single producer
    // in production: 32 `Transaction::user` and zero `Plugin | Agent`. With MCP
    // connected in that state, history would say `user` for everything agent
    // did — and question "who changed this?" would remain unanswered in the only
    // case where it matters.
    let (mut d, r, a) = with_actions();
    d.select_one("ActionList1");
    let agent = Author::Agent("claude".into());

    for id in ["action.new", "action.standard"] {
        let args = Args::texts("classes", ["EditCopy"]);
        let c = ctx_from(&d, &r, &a, args, agent.clone());
        let tx = (by_id(id).unwrap().execute)(&c)
            .transaction
            .unwrap_or_else(|| panic!("{id} should produce transaction"));
        assert_eq!(tx.author, agent, "{id} did not sign with requester");
    }

    // And removal, which builds transaction by hand in `removal`.
    d.select_one("Action1");
    let c = ctx_from(&d, &r, &a, Args::none(), Author::Plugin("x".into()));
    let tx = (by_id("edit.delete").unwrap().execute)(&c)
        .transaction
        .expect("delete produces transaction");
    assert_eq!(tx.author, Author::Plugin("x".into()));

    // Shell continues saying `User`, because it is true.
    d.select_one("Action1");
    let c = ctx(&d, &r, &a);
    let tx = (by_id("edit.delete").unwrap().execute)(&c)
        .transaction
        .unwrap();
    assert_eq!(tx.author, Author::User);
}

/// Action families whose transaction is created inside `crawrad-designer`, which
/// is why they **still return signed `User`** even when requester was an
/// agent. One per family, because fix is per layout function:
/// `align`, `z_order`, `paste` and `duplicate`.
///
/// **This list only shrinks.**
const DO_NOT_CROSS_DESIGNER: &[&str] = &[
    "align.left",
    "order.bring_to_front",
    "edit.paste",
    "edit.duplicate",
];

#[test]
fn author_still_stops_at_crawrad_designer_boundary() {
    // THIS MEASURES A HOLE, not desired behaviour. Transactions of
    // align, order, paste and duplicate are constructed inside
    // `crawrad-designer`, which does not yet receive author.
    //
    // Designer mouse gesture IS user's by definition, so change
    // there is not signature change — it is a new parameter in
    // arrangement functions, which did not fit this iteration.
    //
    // All FOUR together, not single sample: with only `align.left` bound,
    // fixing alignment would leave test red and cheap path
    // would be inverting assertion — and `paste`, `duplicate` and `order` would leave
    // radar without anyone deciding that. Here, fixing one makes message say
    // WHICH crossed and tells to remove from list, keeping others bound.
    let agent = Author::Agent("claude".into());
    let mut still_signs_user = Vec::new();

    for id in DO_NOT_CROSS_DESIGNER {
        let (mut d, r, mut area) = scenario();
        d.select(vec!["Button1".into(), "Button2".into()]);
        if *id == "edit.paste" {
            // Paste needs non-empty clipboard; rest operates
            // on selection.
            let sel = d.selection().to_vec();
            assert!(area.copy(d.ir(), &sel) > 0, "clipboard must have content");
        }
        let c = ctx_from(&d, &r, &area, Args::none(), agent.clone());
        let tx = (by_id(id).unwrap().execute)(&c)
            .transaction
            .unwrap_or_else(|| {
                panic!("{id} should produce transaction — without it nothing is measured")
            });
        if tx.author == Author::User {
            still_signs_user.push(*id);
        }
    }

    assert_eq!(
        still_signs_user,
        DO_NOT_CROSS_DESIGNER,
        "author crossed to: {:?}. Remove THAT/THOSE from `DO_NOT_CROSS_DESIGNER` \
         and keep others bound — inverting assertion would silence missing ones.",
        DO_NOT_CROSS_DESIGNER
            .iter()
            .filter(|id| !still_signs_user.contains(id))
            .collect::<Vec<_>>()
    );
}

// ------------------------------------- what the shell still mutates externally

/// How many document mutations live in shell, outside catalogue.
///
/// Applies to **all `.rs` of `crates/crawrad-ide/src`**, subfolders included —
/// not to cherry-picked list of files, which was how this guard
/// began and where `Transaction::user` in `tree.rs` passed green.
///
/// **This number decreases by refactoring and only increases by table row.**
///
/// Written rule previously was *"can only decrease"*, and it was contradicted nine
/// lines below in same block, first time legitimate exception
/// appeared. Ratchet whose rule is contradicted within itself loses teeth
/// on third time: next arrival reads `4 → 5` and concludes statement is
/// decorative.
///
/// So it states what test always actually required, and what failure
/// text down there offered — *"put it in catalogue,
/// **or justify here why it is an exception**"*. Increasing requires both
/// halves, and they form the cost:
///
/// 1. **a new row in table below**, stating where mutation lives, what it
///    mutates and name of action that would remove it;
/// 2. **why it is not an interface gesture** — question §12.3 asks is
///    whether an agent can reach same place as user. Mutation already
///    running INSIDE a catalogue action is reachable from outside and is not a
///    hole; mutation only mouse reaches is.
///
/// Without both, number only decreases.
///
/// It is measure of hole that §12.3 calls the classic sin
/// of OTA: 41 of 52 menu items already come from catalogue, but mutations below
/// do not — so agent aligned, copied and deleted, and **could not
/// create a component or modify a property**.
///
/// **13 → 4**, with four missing mutations (`component.add`,
/// `component.set_property`, `component.rename`, `event.bind`). Nine of thirteen
/// were those, and nine became catalogue call with `Args`.
///
/// **4 → 5 on 08/09/2026, and increase is justified below** — failure
/// text of this test opens that door (*"put it in catalogue, or
/// justify here why it is an exception"*). Fifth is the only one **already
/// reachable from outside**: runs inside `project.generate`, which is catalogue
/// action. See row 5 of table.
///
/// **5 → 4 on 15/09/2026**, with `page.new`: new `TabSheet` left shell.
/// Was row waiting for "a way to compose effect", and what
/// removed it was cheaper — **an action already knowing both halves**, which is
/// what `action.standard` does with N `AddNode` since forever. Batch remains
/// single one, and Ctrl+Z as well.
///
/// **4 → 2 on 17/09/2026**, with `component.reorder`. Were THREE mutations of
/// same nature — `Move Up`/`Move Down` of ActionList Editor, two in egui
/// shell and one Makepad shell just wrote — and none had
/// id: agent could create and delete action, and could not change
/// their ORDER, which decides menu order in generated app.
///
/// Action became boundary checker: first in list with `Move Up`
/// returns silence, and shells only decide if BUTTON lights up.
/// **2 → 1 on 17/09/2026, and IT DID NOT DECREASE BY MERIT.**
///
/// Failure text of this test instructs checking exactly this, because
/// count already plummeted once due to lost file slice. Here
/// cause is different and honest, but still not merit:
/// ADR 0032
/// removed egui shell per owner request — *"the IDE WILL ONLY BE IN MAKEPAD"* — and
/// **one of two mutations went away inside `main.rs`**, without becoming any
/// action.
///
/// The one that remains is from `generator.rs`, and it remains justified
/// exception as always: runs inside `project.generate`, which is catalogue
/// action, and thus is already reachable from outside.
///
/// **What this means to reader:** debt was not paid, it changed
/// location. If egui shell returned (it is in git history), count would return to 2 — and
/// number here would have to increase along with it, with this same note reversed.
const MUTATIONS_OUTSIDE_CATALOGUE: usize = 1;

/// The original 13, measured at commit `3994caf`, and what happened to each.
/// Row changes; reason does not:
///
/// | # | where | what | destination |
/// |---|---|---|---|
/// | ~~1~~ | `main.rs` `confirmar_evento_pendente` | `SetEvent` of event field confirmed by Enter | **`event.bind`** |
/// | ~~2~~ | `main.rs` `confirmar_rename_pendente` | pending `Rename` on focus change | **`component.rename`** |
/// | ~~3~~ | `main.rs` double click on component | wires default link handler | **`event.bind`** |
/// | ~~4~~ | `main.rs` `acao_de_aba` Open | `active_page` of PageControl | **`component.set_property`** |
/// | ~~5~~ | `main.rs` `acao_de_aba` Create | new `TabSheet`, and it opens | **`page.new`** |
/// | ~~6~~ | `main.rs` drop from palette | `AddNode` with parent and rect | **`component.add`** — literal example of ADR 0005 |
/// | 1 | `main.rs` ActionList editor | `Reorder` up | `component.reorder` |
/// | 2 | `main.rs` ActionList editor | `Reorder` down | `component.reorder` |
/// | ~~9~~ | `inspector.rs` | `SetEvent` on focus loss | **`event.bind`** |
/// | ~~10~~ | `inspector.rs` | `Rename` via Inspector | **`component.rename`** |
/// | ~~11~~ | `inspector.rs` | `SetProp` on N selected, one batch | **`component.set_property`** |
/// | ~~12~~ | `items_editor.rs` | `items` of collection editor | **`component.set_property`** |
/// | 3 | `image_list_editor.rs` | three `SetProp` of ImageList | `component.set_properties` |
/// | 4 | `crawrad-project/src/generator.rs` pruning pre-pass | N `SetEvent { handler: None }` of pruning pre-pass | remains outside — see below |
///
/// **The four remaining did not remain out of forgetfulness**, and none is
/// one of four mutations that entered with `component.`:
///
/// - **1 and 2 (`Reorder`)** request `component.reorder`, which is none of the
///   four;
/// - **3 (ImageList)** are THREE properties in single transaction
///   (`images_data`, `image_width`, `image_height`).
///   `component.set_property` modifies **one** key, and splitting into three calls
///   would trade one Ctrl+Z for three — opposite of what `crawrad-doc` sells.
///   Name of what is missing is `component.set_properties`, in plural of KEYS,
///   and is design decision not refactoring;
/// - **4 (pruning pre-pass)** is the only one **not an interface gesture**,
///   so it increases ratchet without violating what ratchet measures. §12.3 requires
///   mutation that *"does not exist for RTA nor for MCP"*; this exists:
///   runs inside `generate_project`, which is what shell does when receiving
///   `Effect { file: Some(FileEffect::Generate) }` of action `project.generate`.
///   Agent calling `project.generate` receives disconnection along with it, with
///   id, in catalogue, and with same `Ctrl+Z` (ADR 0025).
///
///   It does **not** become its own action for two reasons, first being same that
///   held new `TabSheet` (former row 5): N `SetEvent` in single
///   transaction — one `event.bind` per binding
///   would trade one undo for N, and shared handler would have living
///   binding between steps, calling method in `.designer.rs` that
///   already left. Second is that it is not a **choice**: decider of which
///   bindings expired is `crawrad_codegen::vinculos_vencidos`, reading user's
///   file on disk. Catalogue action only knowing how to repeat response
///   of another function adds no surface — adds second
///   place where ruler can diverge.
///
///   What would drop this row is **not** what was written here. It was said
///   that it and new `TabSheet` waited for "a way to compose catalogue
///   effect in single transaction"; `page.new` dropped the other without parts,
///   writing both commands inside action itself. What is missing
///   for this one is second reason above, which composition would not solve:
///   it is not choice of caller, but answer of
///   `crawrad_codegen::vinculos_vencidos` — and row 3 waits for project
///   decision (`component.set_properties`), not infrastructure.
#[test]
fn shell_still_mutates_document_outside_catalogue() {
    // Scan is of ENTIRE DIRECTORY, descending into subfolders.
    //
    // First version enumerated four `include_str!` — precisely four
    // files where 13 live —, and simply placing `Transaction::user` in
    // `tree.rs` left guard GREEN. Ratchet guard enumerating
    // targets measures what is known and silences new findings, opposite
    // of what it exists to do. Descent is recursive because shell has
    // subfolders (`explorer/`, `diagnostics/`, `welcome/`), and new file
    // must enter count automatically.
    //
    // NUMBER of files is not written here, for measured reason:
    // first writing stated "18 files in three folders", and became 20 in
    // four **within same wave** — `welcome/` folder was born later.
    // Number aging in commit that wrote it is not documentation, is
    // trap for next reader.
    //
    // Reading at TEST time, and not with `include_str!`, has second effect:
    // `crawrad-actions` test stops recompiling on every letter edited in
    // shell — ~300 KB of `main.rs` entered test binary.
    //
    // Path comes from `CARGO_MANIFEST_DIR` and not current working directory,
    // which `cargo test` does not guarantee. This remains NOT a crate
    // dependency: `crawrad-actions` cannot know shell (see
    // `catalogue_does_not_know_the_interface`), and reading file text creates
    // no link in graph.
    let manifest = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let roots = [
        manifest.join("../crawrad-ide/src"),
        manifest.join("../crawrad-project/src"),
    ];

    let mut files = Vec::new();
    for root in &roots {
        let mut f = Vec::new();
        rs_files(root, &mut f);
        assert!(
            f.len() > 4,
            "scan found {} file(s) in {} — path moved location",
            f.len(),
            root.display()
        );
        files.extend(f);
    }

    // File that IS an entire test module leaves count BEFORE everything,
    // because inside it there is no slice to make: `explorer/tests.rs` has no
    // `#[cfg(test)] mod` to find — `#[cfg(test)]` is on DECLARATION, there
    // in `explorer.rs`. Measured: legitimate `#[test]` with `Transaction::user`
    // in `explorer/tests.rs` increased ratchet from 13 to 14 and told author
    // to put their test in actions catalogue.
    let test_only = test_only_files(&files);

    let mut details = Vec::new();
    for path in &files {
        if test_only.contains(path) {
            continue;
        }
        let source = std::fs::read_to_string(path).expect("read source");
        let production = production_of(&source);

        // Invariant closing circle, valid for all machinery above:
        // if `#[test]` remains in what was called production,
        // some test region was NOT recognized — and whatever is inside
        // will be counted as IDE mutation. Failing here, stating
        // file, costs less than ratchet accusing test author.
        assert!(
            !production.contains("#[test]"),
            "{}: stray `#[test]` in what this guard calls production. \
             Some test region of this file was not recognized by \
             `production_of`/`test_only_files`, and what is inside \
             becomes mutation in count below. Fix recognizer — not \
             the number.",
            path.display()
        );

        let n: usize = DOORS
            .iter()
            .map(|door| production.matches(door).count())
            .sum();
        if n > 0 {
            let name = roots
                .iter()
                .find_map(|r| {
                    path.strip_prefix(r).ok().map(|rel| {
                        let crate_name = r.parent().and_then(|p| p.file_name()).unwrap_or_default();
                        format!(
                            "{}/{rel}",
                            crate_name.to_string_lossy(),
                            rel = rel.display()
                        )
                    })
                })
                .unwrap_or_else(|| path.to_string_lossy().into_owned());
            details.push((name, n));
        }
    }

    let total: usize = details.iter().map(|(_, n)| n).sum();
    let list: Vec<String> = details.iter().map(|(f, n)| format!("{f}: {n}")).collect();

    assert_eq!(
        total,
        MUTATIONS_OUTSIDE_CATALOGUE,
        "mutations outside catalogue: {total}, expected {MUTATIONS_OUTSIDE_CATALOGUE} \
         (in {} files — {}).\n\
         INCREASED? A new mutation was born in the shell. It does not exist for RTA nor \
         for MCP, has no id, does not appear in catalogue and nobody from outside \
         can call it (ADR 0005). Put it in catalogue, or justify here \
         why it is an exception.\n\
         DECREASED? One became real action: lower the constant, cross out table row \
         above, and this number is demo of progress. And verify if \
         decreased by MERIT: count already plummeted once because file slice \
         was lost, and lowering constant there would have blinded guard \
         permanently.",
        files.len(),
        list.join(", ")
    );
}

/// The three doors for a transaction — all three count.
///
/// - `Transaction::user(` — shortcut, form the 13 use today;
/// - `Transaction::new(` — general form, with explicit author;
/// - `Transaction {` — **struct literal**. Fields are `pub`
///   (`crawrad-doc/src/lib.rs:54-57`), so it builds transaction without passing
///   through any function and escapes name counting. Counting text
///   catches `fn … -> Transaction {`, which is INTENTIONAL: a shell
///   function returning transaction is precisely a mutation outside catalogue.
///
/// **What this guard does not reach**, stated here rather than silenced: an
/// alias (`use crawrad_doc::Transaction as T`), a macro building literal, or
/// transaction built in another crate and only applied here. Closing that would require
/// reading type graph instead of text — ghost chase for unmeasured
/// gain. What guards those three is review, and now it knows what to
/// look for.
///
/// And other side, also measured: count is on TEXT of production, with
/// comments and strings inside. Comment citing `Transaction::user(` increases
/// ratchet — verified putting `// probe: Transaction::user(` in `tree.rs`, and
/// count went to 14. Remains so on purpose: error falls to side failing
/// loudly, and rewriting comment phrase costs nothing. Code scanner of
/// `code_of` would give code-only count, but errs towards side SILENCING —
/// a `Transaction /* … */ {` would fail to match —, and this test does not trade noise
/// for silence.
const DOORS: [&str; 3] = ["Transaction::user(", "Transaction::new(", "Transaction {"];

/// Collects `.rs` files in folder, descending into subfolders, in stable order.
fn rs_files(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
    let mut paths: Vec<std::path::PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("shell must be in {}: {e}", dir.display()))
        .filter_map(Result::ok)
        .map(|e| e.path())
        .collect();
    paths.sort();

    for path in paths {
        if path.is_dir() {
            rs_files(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

/// Files that are an ENTIRE test module, and therefore do not enter
/// count on any line.
///
/// Criterion is not file NAME — that would be same trap of `include_str!`
/// list, and an `explorer/cases.rs` would slip past. Criterion is
/// **declaration**: writing `#[cfg(test)] mod tests;` in `explorer.rs`
/// states that entire `explorer/tests.rs` only exists under `cfg(test)`.
///
/// Scan walks to fixed point because marked file carries its
/// children with it: `mod cases;` WITHOUT attribute inside
/// `explorer/tests.rs` is also test, since entire parent is.
fn test_only_files(files: &[std::path::PathBuf]) -> std::collections::BTreeSet<std::path::PathBuf> {
    let mut marked = std::collections::BTreeSet::new();
    let sources: Vec<String> = files
        .iter()
        .map(|c| std::fs::read_to_string(c).expect("read shell source"))
        .collect();

    loop {
        let before = marked.len();
        for (path, source) in files.iter().zip(&sources) {
            // In already marked file, EVERY declaration counts; in production, only
            // those inside a `#[cfg(test)]` region.
            let text: String = if marked.contains(path) {
                source.clone()
            } else {
                test_text(source)
            };
            for candidate in declared_module_paths(path, &text) {
                if files.contains(&candidate) {
                    marked.insert(candidate);
                }
            }
        }
        if marked.len() == before {
            return marked;
        }
    }
}

/// Only what the file contains of tests, regions joined by LINE BREAK.
///
/// The break is not decorative. Joining raw, two neighbouring declarations become
/// `mod tests;#[cfg(test)]` on a single line, and `declared_modules` — reading
/// line-by-line — loses the first. Found by test
/// `test_module_in_own_file_is_removed_entirely`: a test file
/// left out of list returns to counting as production, the false
/// positive this fix arrived to remove.
fn test_text(source: &str) -> String {
    test_regions(source)
        .iter()
        .map(|(i, f)| &source[*i..*f])
        .collect::<Vec<_>>()
        .join("\n")
}

fn declared_module_paths(parent: &std::path::Path, text: &str) -> Vec<std::path::PathBuf> {
    let mut paths = Vec::new();
    let mut custom_path: Option<String> = None;
    for line in text.lines() {
        let l = line.trim();
        if let Some(rest) = l.strip_prefix("#[path = \"")
            && let Some(rel_path) = rest.strip_suffix("\"]")
        {
            custom_path = Some(rel_path.to_string());
            continue;
        }
        let l = if let Some(rest) = l.strip_prefix("pub") {
            let r = rest.trim_start();
            if let Some(paren_rest) = r.strip_prefix('(') {
                if let Some((_, after)) = paren_rest.split_once(')') {
                    after.trim_start()
                } else {
                    l
                }
            } else {
                r
            }
        } else {
            l
        };
        let Some(rest) = l.strip_prefix("mod ") else {
            continue;
        };
        let Some(name) = rest.strip_suffix(';') else {
            continue;
        };
        let name = name.trim();
        if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            if let Some(custom) = custom_path.take() {
                if let Some(parent_dir) = parent.parent() {
                    let mut p = parent_dir.to_path_buf();
                    for seg in custom.split(['/', '\\']) {
                        match seg {
                            // `..` IS RESOLVED HERE, and absence of this
                            // cost a red: a submodule of `ide_app/`
                            // declaring its own test writes
                            // `#[path = "../test/ide_app/x.rs"]`, and pushing
                            // `..` as folder produced
                            // `src/ide_app/../test/ide_app/x.rs` — which is not
                            // equal to any path in scanned list, so
                            // entire test file counted as PRODUCTION.
                            //
                            // Rust `Path` does not normalize by itself: `join`
                            // and `push` are lexical, and `canonicalize` touches
                            // disk and returns `\\?\` on Windows, which fails
                            // to match list as well.
                            ".." => {
                                p.pop();
                            }
                            "." | "" => {}
                            _ => p.push(seg),
                        }
                    }
                    paths.push(p);
                }
            } else {
                paths.extend(module_paths(parent, name));
            }
        }
    }
    paths
}

/// Names of `mod name;` — declaration without body — found in text.
fn declared_modules(text: &str) -> Vec<String> {
    let mut names = Vec::new();
    for line in text.lines() {
        let l = line.trim();
        let l = l.strip_prefix("pub ").unwrap_or(l);
        let Some(rest) = l.strip_prefix("mod ") else {
            continue;
        };
        let Some(name) = rest.strip_suffix(';') else {
            continue;
        };
        let name = name.trim();
        if !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            names.push(name.to_owned());
        }
    }
    names
}

/// Where file of `mod name;` declared in `parent` may live.
fn module_paths(parent: &std::path::Path, name: &str) -> Vec<std::path::PathBuf> {
    let is_module_root = matches!(
        parent.file_stem().and_then(|s| s.to_str()),
        Some("main" | "lib" | "mod")
    );
    let dir = if is_module_root {
        parent.parent().map(std::path::Path::to_path_buf)
    } else {
        parent
            .parent()
            .zip(parent.file_stem())
            .map(|(d, s)| d.join(s))
    };
    let Some(dir) = dir else {
        return Vec::new();
    };
    vec![
        dir.join(format!("{name}.rs")),
        dir.join(name).join("mod.rs"),
    ]
}

/// What remains of file after REMOVING all that exists only under test.
///
/// **Removing, not slicing — here was the hole.** Previous version truncated
/// file at first `#[cfg(test)] mod` and gave rest to a "tail"
/// watched by list of item beginnings (`fn `, `impl `, `struct `,
/// `enum `, `const `). List had hole in everything not in it, and
/// measurement was taken one by one:
/// `pub(crate) fn`, `pub(super) fn`, `mod`, `macro_rules!`, `type`, `trait`,
/// `impl<'a>`, `async fn`, `unsafe impl` and `static` — TEN forms, each with
/// a `Transaction::user` inside, and all ten passed GREEN. Only plain `fn `
/// was caught.
///
/// Slicing had an even worse failure mode, needing no item
/// in tail: recognizer simply failing to match `#[cfg(test)]` of a
/// module — `#[allow(dead_code)]` in between, or `mod` on SAME line as
/// attribute — made slice SLIP to next suite (where `main.rs` has
/// eight) and skipped module became production without a sound. Measured both ways:
/// green in both; and with test `Transaction` inside skipped module
/// ratchet went to 15, accusing test.
///
/// Excision has no tail, no slipping and no item list: what
/// is under `#[cfg(test)]` leaves, wherever it is and whatever type,
/// and all rest is production — including what comes AFTER suites,
/// case that slicing missed, and including 800 lines following
/// `#[cfg(test)] pub const` of `welcome.rs:229` that attribute slice
/// lost.
///
/// What remains outside excision, and why: `#[cfg(all(test, …))]` is not
/// recognized and continues counting as production. Safe side of error —
/// overcounting fails loudly, undercounting silences.
fn production_of(source: &str) -> String {
    let mut out = String::with_capacity(source.len());
    let mut cursor = 0usize;
    for (i, f) in test_regions(source) {
        out.push_str(&source[cursor..i]);
        cursor = f;
    }
    out.push_str(&source[cursor..]);
    out
}

/// Byte ranges of each `#[cfg(test)]` item, from `#` to `}` or `;` closing item.
///
/// Attribute is searched in CODE (see `code_of`), never inside
/// comment or string: `#[cfg(test)]` cited in doc comment cannot
/// move any boundary. From attribute end walk to first `;`
/// or `{` outside parenthesis and brackets — `;` closes declaration
/// (`mod tests;`, `use super::*;`) and `{` opens body, matched counting
/// braces. `[f32; 2]` of `const` is reason to count bracket: without this
/// semicolon of TYPE closes item too early.
///
/// Attribute between `#[cfg(test)]` and item passes free, because its brackets
/// match along the way — and that was exactly the slipping.
fn test_regions(source: &str) -> Vec<(usize, usize)> {
    const ATTR: &[u8] = b"#[cfg(test)]";
    let code = code_of(source);
    let bytes: Vec<u8> = code.iter().map(|(_, b)| *b).collect();

    let mut regions = Vec::new();
    let mut k = 0usize;
    while k + ATTR.len() <= bytes.len() {
        if &bytes[k..k + ATTR.len()] != ATTR {
            k += 1;
            continue;
        }
        let end = end_of_item(&bytes, k + ATTR.len()).unwrap_or_else(|| {
            panic!(
                "the `#[cfg(test)]` item starting at «{}» does not close — scanner \
                 lost step (raw string? nested comment?). This guard \
                 prefers failing loudly to counting incorrectly.",
                &source[code[k].0..(code[k].0 + 60).min(source.len())]
            )
        });
        regions.push((code[k].0, code[end].0 + 1));
        // A `#[cfg(test)]` INSIDE an already excised region opens no new
        // region: it is already outside production.
        k = end + 1;
    }
    regions
}

/// From attribute end to code byte index closing item.
fn end_of_item(bytes: &[u8], start: usize) -> Option<usize> {
    let mut nested = 0i32;
    let mut j = start;
    while j < bytes.len() {
        match bytes[j] {
            b'(' | b'[' => nested += 1,
            b')' | b']' => nested -= 1,
            b';' if nested == 0 => return Some(j),
            b'{' if nested == 0 => {
                let mut level = 0i32;
                while j < bytes.len() {
                    match bytes[j] {
                        b'{' => level += 1,
                        b'}' => {
                            level -= 1;
                            if level == 0 {
                                return Some(j);
                            }
                        }
                        _ => {}
                    }
                    j += 1;
                }
                return None;
            }
            _ => {}
        }
        j += 1;
    }
    None
}

/// Bytes of file that are CODE, each with source offset.
///
/// Excluded: line comments, block comments (which nest in Rust),
/// strings, raw strings with any number of `#`, character literals.
/// Single quote of lifetime (`'a`, `'static`) opens no literal,
/// distinction that prevents brace counter from getting lost inside
/// `impl<'a>`.
fn code_of(source: &str) -> Vec<(usize, u8)> {
    let b = source.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0usize;
    while i < b.len() {
        if b[i] == b'/' && b.get(i + 1) == Some(&b'/') {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
            let mut level = 1u32;
            i += 2;
            while i < b.len() && level > 0 {
                if b[i] == b'/' && b.get(i + 1) == Some(&b'*') {
                    level += 1;
                    i += 2;
                } else if b[i] == b'*' && b.get(i + 1) == Some(&b'/') {
                    level -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        if let Some(end) = end_of_raw_string(b, i) {
            i = end;
            continue;
        }
        if b[i] == b'"' {
            i += 1;
            while i < b.len() {
                if b[i] == b'\\' {
                    i += 2;
                    continue;
                }
                let closed = b[i] == b'"';
                i += 1;
                if closed {
                    break;
                }
            }
            continue;
        }
        if b[i] == b'\''
            && let Some(end) = end_of_char(b, i)
        {
            i = end;
            continue;
        }
        out.push((i, b[i]));
        i += 1;
    }
    out
}

/// End of raw string starting at `i`, if it is one.
fn end_of_raw_string(b: &[u8], i: usize) -> Option<usize> {
    // `r` must OPEN word (or come right after `b` in `br"`), otherwise
    // `r` of `for` becomes start of string.
    if b[i] != b'r' {
        return None;
    }
    let adjacent = |p: Option<usize>| {
        p.map(|p| b[p])
            .is_some_and(|c| c.is_ascii_alphanumeric() || c == b'_')
    };
    let opens_word = !adjacent(i.checked_sub(1));
    let after_b = i.checked_sub(1).map(|p| b[p]) == Some(b'b') && !adjacent(i.checked_sub(2));
    if !opens_word && !after_b {
        return None;
    }
    let mut j = i + 1;
    while b.get(j) == Some(&b'#') {
        j += 1;
    }
    let fences = j - (i + 1);
    if b.get(j) != Some(&b'"') {
        return None;
    }
    j += 1;
    while j < b.len() {
        if b[j] == b'"' && b[j + 1..].iter().take(fences).all(|c| *c == b'#') {
            return Some(j + 1 + fences);
        }
        j += 1;
    }
    Some(b.len())
}

/// End of character literal starting at `i` — `None` when that
/// quote belongs to a lifetime.
fn end_of_char(b: &[u8], i: usize) -> Option<usize> {
    if b.get(i + 1) == Some(&b'\\') {
        // Escape: byte right after backslash never closes literal, which is why
        // search starts after it — `'\''` is the case caught.
        let mut j = i + 3;
        while j < b.len() && b[j] != b'\'' {
            j += 1;
        }
        return Some(j + 1);
    }
    let mut j = i + 1;
    while j < b.len() && (b[j].is_ascii_alphanumeric() || b[j] == b'_') {
        j += 1;
    }
    if j > i + 1 && b.get(j) != Some(&b'\'') {
        return None; // lifetime
    }
    let mut j = i + 1;
    while j < b.len() && b[j] != b'\'' {
        j += 1;
    }
    Some(j + 1)
}

// ------------------------------------- proof of slicing itself

/// Slicing recognizes suite written in any compilable form.
///
/// Each form here was measured in real shell before fix, with
/// minimal mutation, and passed GREEN through old guard — which matched
/// `#[cfg(test)]` only when FOLLOWING line began exactly with `mod `.
/// First two are "slipping": slice did not vanish, moved to next suite,
/// and skipped suite became silent production.
#[test]
fn slicing_recognizes_suite_in_any_shape() {
    let shapes = [
        ("own line", "#[cfg(test)]\nmod t {\n    fn x() {}\n}\n"),
        ("same line", "#[cfg(test)] mod t {\n    fn x() {}\n}\n"),
        (
            "attribute in middle",
            "#[cfg(test)]\n#[allow(dead_code)]\nmod t {\n    fn x() {}\n}\n",
        ),
        ("pub mod", "#[cfg(test)]\npub mod t {\n    fn x() {}\n}\n"),
        (
            "pub(crate) mod",
            "#[cfg(test)]\npub(crate) mod t {\n    fn x() {}\n}\n",
        ),
        (
            "nested",
            "#[cfg(test)]\nmod t {\n    mod inside {\n        fn x() {}\n    }\n}\n",
        ),
    ];
    for (name, source) in shapes {
        let source = format!("fn production() {{}}\n{source}");
        let remainder = production_of(&source);
        assert!(
            remainder.contains("fn production()"),
            "{name}: slicing ate production — {remainder:?}"
        );
        assert!(
            !remainder.contains("mod t"),
            "{name}: suite remained in production — {remainder:?}"
        );
    }
}

/// Production written AFTER suites remains in count.
///
/// Old guard cut file at first `#[cfg(test)] mod` and watched
/// tail with list of item beginnings — `fn `, `impl `, `struct `, `enum `,
/// `const `. Each form passed green by not being on list, with
/// `Transaction::user` inside, measured one-by-one in `palette_alignment.rs`.
#[test]
fn production_after_suite_remains_in_count() {
    let tails = [
        "pub(crate) fn f() -> Transaction { Transaction::user(vec![]) }",
        "pub(super) fn f() -> Transaction { Transaction::user(vec![]) }",
        "mod extra {\n    fn f() -> Transaction { Transaction::user(vec![]) }\n}",
        "macro_rules! m {\n    () => { Transaction::user(vec![]) };\n}",
        "trait T {\n    fn f(&self) -> Transaction { Transaction::user(vec![]) }\n}",
        "impl<'a> T for U<'a> {\n    fn f(&self) -> Transaction { Transaction::user(vec![]) }\n}",
        "async fn f() -> Transaction { Transaction::user(vec![]) }",
        "unsafe fn f() -> Transaction { Transaction::user(vec![]) }",
        "static F: fn() -> Transaction = || Transaction::user(vec![]);",
    ];
    for tail in tails {
        let source = format!("#[cfg(test)]\nmod t {{\n    fn x() {{}}\n}}\n\n{tail}\n");
        let remainder = production_of(&source);
        let n: usize = DOORS.iter().map(|p| remainder.matches(p).count()).sum();
        assert!(n > 0, "escaped count: {tail}");
    }
}

/// `#[cfg(test)]` on single item removes only that item.
///
/// `welcome.rs:229` has `#[cfg(test)] pub const` on line 229 of
/// 1059-line file: cutting there left 800 lines of production out of count, and
/// number remained correct by luck — that file has no transaction.
/// `[f32; 2]` is here because semicolon of TYPE would close item
/// too early if slicing did not count brackets.
#[test]
fn cfg_test_on_single_item_removes_only_the_item() {
    let source = "#[cfg(test)]\npub const A: [f32; 2] = [1.0, 2.0];\n\nfn afterwards() -> Transaction { Transaction::user(vec![]) }\n";
    let remainder = production_of(source);
    assert!(!remainder.contains("pub const A"), "test const remained");
    assert!(
        remainder.contains("fn afterwards()"),
        "production after it vanished"
    );
}

/// Scanner does not confuse text with code.
///
/// Brace inside raw string, comment and character literal cannot
/// close any suite; and quote of lifetime (`impl<'a>`) does not
/// open literal. Without this brace matching gets lost and slice swallows
/// rest of file — failure causing ratchet to PLUMMET and look like
/// progress.
#[test]
fn scanner_does_not_confuse_text_with_code() {
    let source = "#[cfg(test)]\nmod t {\n    const S: &str = r#\"}\n}\n\"#;\n    // }\n    /* } /* } */ */\n    const C: char = '}';\n    fn f<'a>(x: &'a str) -> &'a str { x }\n}\n\nfn afterwards() -> Transaction { Transaction::user(vec![]) }\n";
    let remainder = production_of(source);
    assert!(
        !remainder.contains("const S"),
        "slicing stopped early: {remainder:?}"
    );
    assert!(
        remainder.contains("fn afterwards()"),
        "slicing ate too much: {remainder:?}"
    );
}

/// A `#[cfg(test)]` cited in comment moves no boundary.
#[test]
fn attribute_cited_in_comment_does_not_slice() {
    let source = "/// See `#[cfg(test)]`\n/// mod tests;\nfn production() -> Transaction { Transaction::user(vec![]) }\n";
    let remainder = production_of(source);
    assert!(
        remainder.contains("fn production()"),
        "comment triggered slicing: {remainder:?}"
    );
}

/// The file of a `mod` declared under `#[cfg(test)]` is entire test.
///
/// By DECLARATION and not by name: `explorer/cases.rs` counts identically, as
/// does `explorer/mod.rs`. Measured: legitimate `#[test]` with
/// `Transaction::user` in `explorer/tests.rs` increased ratchet from 13 to 14.
#[test]
fn test_module_in_own_file_is_removed_entirely() {
    let source = "#[cfg(test)]\nmod tests;\n#[cfg(test)]\nmod cases;\nmod production;\n";
    assert_eq!(
        declared_modules(&test_text(source)),
        vec!["tests", "cases"],
        "two neighbouring declarations must not merge on single line"
    );

    let parent = std::path::Path::new("src/explorer.rs");
    assert_eq!(
        module_paths(parent, "tests"),
        vec![
            std::path::PathBuf::from("src/explorer/tests.rs"),
            std::path::PathBuf::from("src/explorer/tests/mod.rs"),
        ]
    );
    // Module root (`main`, `lib`, `mod`) opens no folder with own
    // name: `mod x;` in `main.rs` is `src/x.rs`, not `src/main/x.rs`.
    assert_eq!(
        module_paths(std::path::Path::new("src/main.rs"), "x")[0],
        std::path::PathBuf::from("src/x.rs")
    );
}

/// `File ▸ Close` closes the **form** and leaves folder open.
///
/// Descends to catalogue, while menu neighbours do not — difference is
/// that of ADR 0005 written in shell `MENU_FILE`: `Open Folder…` and
/// `New Project…` open system dialog and launch subprocess, and an action
/// cannot do that. Closing touches no disk: is `Effect`
/// stating "release what is open", and shell knows how to release. Thus it
/// has `enabled`, has hint, and MCP sees it immediately.
///
/// **Without shortcut, like `Reload`.** Closing has no undo, and two keys
/// rest of world uses for this — `Ctrl+W` and `Ctrl+F4` — are not for
/// text field here (shell's `is_text_field`), so they would
/// close document under someone typing in code editor. The
/// `File ▸ Close` of classic RAD IDE also has no shortcut.
#[test]
fn close_is_catalogue_action_and_only_exists_with_open_form() {
    let a = by_id("file.close").expect("`File ▸ Close` must be in catalogue");
    assert_eq!(a.group, "File");
    assert_eq!(a.shortcut, None, "see shortcut paragraph above");
    let (d, r, actions) = scenario();
    let e = (a.execute)(&ctx(&d, &r, &actions));
    assert_eq!(e.file, Some(FileEffect::Close));
    assert!(
        e.transaction.is_none(),
        "close does not alter document — it releases it"
    );
    assert!(a.enabled.allows(&ctx(&d, &r, &actions)));
    // Converse is what matters: without open form, active `Close` is
    // menu item doing nothing — same defect `has_document`
    // fixed in `Save` and `Reload` when ADR 0017 ceased guaranteeing
    // document.
    assert!(
        !a.enabled.allows(&ctx_without_document(&r, &actions)),
        "without open form there is nothing to close"
    );
}

/// **`Save` is also valid on an edited FILE, with no form at all.**
///
/// # The defect this gate locks
///
/// Predicate of `file.save` was [`has_document`], and was correct while
/// only savable thing was form. On 17/09/2026 code editor of
/// Makepad shell began editing for real, and combination existed:
/// Explorer opens any `.rs` in project, and [ADR 0017] permits project
/// **without form**. User would type, press `Ctrl+S`, and IDE would
/// answer nothing — action refused before reaching disk.
///
/// Converse remains valid: without document and nothing typed,
/// `Save` stays inactive rather than active doing nothing.
///
/// [`has_document`]: super::has_document
#[test]
fn save_works_on_edited_file_without_form() {
    let a = by_id("file.save").expect("`Save` must be in catalogue");
    let (r, actions) = (Registry::builtin(), Clipboard::default());

    let clean = ctx_without_document(&r, &actions);
    assert!(
        !a.enabled.allows(&clean),
        "without form and nothing typed, active `Save` does nothing"
    );

    let dirty = Context {
        editor_dirty: true,
        text: Default::default(),
        ..ctx_without_document(&r, &actions)
    };
    assert!(
        a.enabled.allows(&dirty),
        "text is typed and unwritten, and `Ctrl+S` is refusing"
    );

    // And with an open form it applies either way, with or without editor.
    let (d, r2, actions2) = scenario();
    assert!(a.enabled.allows(&ctx(&d, &r2, &actions2)));
}

// ---------------------------------------------------------------------------
// THE DOMAIN OF A PARAMETER — what MCP publishes, and what menu draws.
// ---------------------------------------------------------------------------

/// The domain of `panel` IS [`Panel::ALL`] — derived, never re-written.
///
/// Reason field exists with function inside rather than slice:
/// seventh panel must appear in tool signature and lines of
/// `View` menu without an edited line in catalogue.
#[test]
fn panel_domain_is_panel_list() {
    for id in [
        "layout.show_panel",
        "layout.toggle_panel",
        "layout.hide_panel",
    ] {
        let a = by_id(id).expect("action is in catalogue");
        let p = a
            .params
            .iter()
            .find(|p| p.name == "panel")
            .expect("it requests `panel`");
        let ids: Vec<&str> = p.choices().iter().map(|c| c.id).collect();
        let expected: Vec<&str> = Panel::ALL.iter().map(|x| x.id()).collect();
        assert_eq!(
            ids, expected,
            "`{id}` publishes different set of panels than `Panel::ALL`"
        );
        for c in p.choices() {
            assert_eq!(
                Panel::from_id(c.id).map(|x| x.label()),
                Some(c.label),
                "label of `{}` is not that of panel",
                c.id
            );
        }
    }
}

/// Every value published by a domain is ACCEPTED by action.
///
/// Converse of `no such panel:` that agent hears today when mistyping
/// name: domain offering value action refuses would be worse than
/// no domain, because signature would be lying.
#[test]
fn every_domain_value_is_accepted_by_action() {
    let (d, r, actions) = scenario();
    for a in catalogue() {
        for p in a.params {
            for c in p.choices() {
                let mut args = Args::text(p.name, c.id);
                // Other required params get first value of their domain,
                // otherwise refusal would come from missing argument and test
                // would pass measuring nothing.
                for other in a.params.iter().filter(|o| o.required && o.name != p.name) {
                    if let Some(v) = other.choices().first() {
                        args = args.and(other.name, crawrad_ir::Value::Str(v.id.into()));
                    }
                }
                let mut c2 = ctx(&d, &r, &actions);
                c2.args = args;
                let effect = (a.execute)(&c2);
                let refusal = effect.status.unwrap_or_default();
                assert!(
                    !refusal.contains("no such"),
                    "`{}` publishes `{}` as value of `{}` and refuses: {refusal}",
                    a.id,
                    c.id,
                    p.name
                );
            }
        }
    }
}

/// Only ONE action requests row per value, which reads as state.
///
/// Six panels times four single-argument actions equals twenty-four
/// rows in `View` menu, and twenty-two nobody asked for. Full
/// rationale in [`Domain::row_per_value`]; here it becomes measurement.
#[test]
fn only_toggle_becomes_row_per_value() {
    let expanded: Vec<&str> = catalogue()
        .iter()
        .filter(|a| a.row_per_value().is_some())
        .map(|a| a.id)
        .collect();
    assert_eq!(expanded, vec!["layout.toggle_panel"]);
}

/// The `panel` of toggle is SAME parameter as others, minus mark.
///
/// Both `const` written in full because functional record updates
/// not allowed in `const`. What cannot diverge is name and
/// domain: different name is argument agent sends wrong, and
/// different domain is menu listing different panels than tool.
#[test]
fn toggle_parameter_is_the_same_as_the_others() {
    let get_param = |id: &str| {
        *by_id(id)
            .expect("action is in catalogue")
            .params
            .iter()
            .find(|p| p.name == "panel")
            .expect("it requests `panel`")
    };
    let toggle = get_param("layout.toggle_panel");
    let show = get_param("layout.show_panel");
    assert_eq!(toggle.name, show.name);
    assert_eq!(toggle.description, show.description);
    assert_eq!(toggle.required, show.required);
    assert_eq!(
        toggle.choices(),
        show.choices(),
        "toggle began listing different panels than rest of catalogue"
    );
    assert!(toggle.domain.is_some_and(|d| d.row_per_value));
    assert!(show.domain.is_some_and(|d| !d.row_per_value));
}

/// Description of parameter with domain DOES NOT repeat values.
///
/// Two places stating which panels exist is list aging silently —
/// and phrase is `&'static str`, unable to track
/// `Panel::ALL`. What answers this is [`Domain`].
#[test]
fn description_with_domain_does_not_repeat_values() {
    for a in catalogue() {
        for p in a.params {
            for c in p.choices() {
                assert!(
                    !p.description.contains(c.id),
                    "`{}`: description of `{}` writes `{}`, which domain already publishes",
                    a.id,
                    p.name,
                    c.id
                );
            }
        }
    }
}

// ---------------------------------------------------------------------------
// THE SECOND CHORD — pair that keyless host delivers
// ---------------------------------------------------------------------------

/// **Every preferred chord points to an action that exists.**
///
/// Table written by id, and id is text: a `view.zoom_in` becoming
/// `view.canvas_zoom_in` would leave row pointing to void, and chord
/// simply stops working without complaint.
#[test]
fn every_preferred_shortcut_has_action() {
    for (id, chords) in PREFERRED_SHORTCUTS {
        assert!(
            by_id(id).is_some(),
            "{chords:?} declared for `{id}`, which does not exist in catalogue"
        );
        assert!(
            !chords.is_empty(),
            "`{id}` in table with EMPTY list — `preferred_shortcut` would return `None` and row does nothing"
        );
    }
}

/// **No preferred chord collides with a catalogue `shortcut`.**
///
/// Both sets trigger on same bridge, so key on both sides
/// would fire two actions simultaneously — same rule
/// `no_shortcut_is_repeated` already enforces within single set.
#[test]
fn preferred_chord_does_not_collide_with_any_shortcut() {
    for (id, chords) in PREFERRED_SHORTCUTS {
        for preferred in *chords {
            for a in catalogue() {
                let Some(shortcut) = a.shortcut else { continue };
                assert_ne!(
                    shortcut, *preferred,
                    "`{preferred}` is preferred of `{id}` and shortcut of `{}` — key fires both",
                    a.id
                );
            }
        }
    }
}

/// **And none repeats across actions.**
///
/// Within action repetition is just noise; BETWEEN actions it makes
/// single key zoom in and zoom out at same time.
#[test]
fn no_preferred_chord_is_repeated() {
    let mut seen: Vec<(&str, &str)> = Vec::new();
    for (id, chords) in PREFERRED_SHORTCUTS {
        for chord in *chords {
            if let Some((other, _)) = seen.iter().find(|(_, c)| c == chord) {
                panic!("`{chord}` is in `{other}` and in `{id}` — both would fire");
            }
            seen.push((id, chord));
        }
    }
}

/// **An action WITHOUT `shortcut` and only preferred is one whose host already does it.**
///
/// Preferred is chord only keyless shell wires. If action
/// lacks `shortcut`, it has **no key at all** in other shell —
/// acceptable only when host there already does exactly that
/// alone.
///
/// Case of UI zoom, and only one: `egui` consumes `Ctrl +` and
/// shifts its `zoom_factor`, same thing action requests. Owner
/// described as: *"look at egui, it was always there"*.
///
/// List is FULL NAMES, not prefix: new `view.ui_zoom_anything`
/// does not enter free, adder must specify which
/// host already does it.
#[test]
fn action_without_shortcut_and_only_preferred_is_done_by_host() {
    const HOST_ALREADY_DOES: &[&str] =
        &["view.ui_zoom_in", "view.ui_zoom_out", "view.ui_zoom_reset"];
    for (id, chords) in PREFERRED_SHORTCUTS {
        let a = by_id(id).expect("checked above");
        if a.shortcut.is_some() {
            continue;
        }
        assert!(
            HOST_ALREADY_DOES.contains(id),
            "`{id}` left only with {chords:?} and not on list of actions host already does. In egui shell it would have no key at all, with nobody serving in its place"
        );
    }
}

/// **Every preferred chord is exactly one of keys egui owns.**
///
/// Test giving PURPOSE to table: if preferred chord did not fall
/// in egui zoom, it would have no reason to be second chord — would
/// simply be `shortcut`, and table convenience drawer.
///
/// Key list is same as shell's
/// `no_catalogue_shortcut_falls_in_egui_ide_zoom`, written
/// here by NAME because crate does not know `egui` — and will not.
///
/// Numpad keys enter by CHARACTER they produce, which
/// is how `egui` sees them: reads logical key, and `NumpadAdd` is
/// `+`.
#[test]
fn preferred_is_always_a_key_egui_owns() {
    const FROM_EGUI: &[&str] = &["+", "=", "-", "0"];
    for (id, chords) in PREFERRED_SHORTCUTS {
        for chord in *chords {
            let key = chord.rsplit('+').next().unwrap_or(chord);
            // `Ctrl+-` ends in `-`, `rsplit` returns empty: separator
            // of chord is character itself. Sole case handled here
            // instead of altering chord format.
            let key = if key.is_empty() { "-" } else { key };
            let key = match key {
                "NumpadAdd" => "+",
                "NumpadSubtract" => "-",
                "Numpad0" => "0",
                other => other,
            };
            assert!(
                FROM_EGUI.contains(&key),
                "`{chord}`, preferred of `{id}`, not key egui consumes. If no host owns it, should be `shortcut` of action, not second chord"
            );
        }
    }
}

/// **`Ctrl +` is THREE keys, all three wired.**
///
/// Defect owner saw on screen — *"makepad doesn't budge"* — with
/// `Ctrl+=` wired alone: `+` of main keyboard is `Shift` + `=`, and
/// numpad is entirely different key. Wiring one is chord
/// working for author and nobody else.
#[test]
fn plus_and_minus_cover_keyboard_forms() {
    let zoom_in = preferred_shortcuts("view.ui_zoom_in");
    for form in ["Ctrl+=", "Ctrl+Shift+=", "Ctrl+NumpadAdd"] {
        assert!(
            zoom_in.contains(&form),
            "`{form}` left zoom-in. Pressing this form of `Ctrl +` sees IDE not budge again"
        );
    }
    let zoom_out = preferred_shortcuts("view.ui_zoom_out");
    for form in ["Ctrl+-", "Ctrl+NumpadSubtract"] {
        assert!(zoom_out.contains(&form), "`{form}` left zoom-out");
    }
    // `Ctrl+Shift+-` DOES NOT enter: `-` does not shift, chord nobody presses.
    assert!(!zoom_out.contains(&"Ctrl+Shift+-"));
}

/// **`component.set_property` refuses key not declared by target's class.**
///
/// # The hole this gate closes, published to AGENTS
///
/// Until 17/09/2026, what refused property keys was a global list of
/// forty names in `crawrad-ir` — not knowing WHO each key belonged to.
/// Measured: a `Button` with `interval` (property of `Timer`) **validated**.
///
/// Inspector never offered this, drawing from descriptor.
/// But this action is what [ADR 0029] publishes outwards, and **an agent asks
/// by name**: `set_property(nodes: [Button1], key: "interval", value: 500)`
/// entered document becoming dead data nobody reads.
///
/// [Amendment 1 of ADR 0031] removed gatekeeping role from list and question
/// became CLASS's. This is point where checked before document
/// changes: single mutation surface (ADR 0005), only place in
/// system where catalogue and transaction meet.
///
#[test]
fn set_property_refuses_key_not_declared_by_class() {
    let (d, r, a) = scenario();
    let action = by_id("component.set_property").expect("action is in catalogue");

    // `interval` belongs to `Timer`, and target is `Button`.
    let args = Args::texts("nodes", ["Button1"])
        .and("key", Value::Str("interval".into()))
        .and("value", Value::Int(500));
    let e = (action.execute)(&ctx_with(&d, &r, &a, args));
    assert!(
        e.transaction.is_none(),
        "transaction was born, and `interval` entered document"
    );
    // PHRASE STATES CLASS AND KEY — requester needs to know which
    // is wrong.
    let status = e.status.unwrap_or_default();
    assert!(
        status.contains("Button") && status.contains("interval"),
        "refusal does not name culprit: {status}"
    );

    // And what class DECLARES continues passing, otherwise gate would have killed
    // action instead of guarding it.
    let args = Args::texts("nodes", ["Button1"])
        .and("key", Value::Str("text".into()))
        .and("value", Value::Str("OK".into()));
    let e = (action.execute)(&ctx_with(&d, &r, &a, args));
    assert!(
        e.transaction.is_some(),
        "`Button.text` exists and must pass"
    );
}

/// `Checked` and `State` say the same thing twice; setting one sets the
/// other in the same transaction (one undo), and only on a CheckBox.
#[test]
fn set_property_keeps_checked_and_state_in_agreement() {
    let (d, r, a) = scenario();
    let mut doc = d.ir().clone();
    let chk = r
        .instantiate("CheckBox", "check_box1", Rect::new(10.0, 20.0, 120.0, 20.0))
        .unwrap();
    doc.add_child("form_main", chk).unwrap();
    let d = Doc::new(doc);
    let action = by_id("component.set_property").expect("action is in catalogue");
    let set = |key: &str, value: Value| {
        let args = Args::texts("nodes", ["check_box1"])
            .and("key", Value::Str(key.into()))
            .and("value", value);
        (action.execute)(&ctx_with(&d, &r, &a, args))
            .transaction
            .expect("produces transaction")
    };
    let other = |tx: &Transaction, key: &str| {
        tx.commands.iter().find_map(|c| match c {
            Command::SetProp {
                key: k,
                value: Some(v),
                ..
            } if k == key => Some(v.clone()),
            _ => None,
        })
    };

    let tx = set("checked", Value::Bool(true));
    assert_eq!(other(&tx, "state"), Some(Value::Enum("cbChecked".into())));
    let tx = set("state", Value::Enum("cbGrayed".into()));
    assert_eq!(
        other(&tx, "checked"),
        Some(Value::Bool(false)),
        "grayed is not checked"
    );
    let tx = set("state", Value::Enum("cbChecked".into()));
    assert_eq!(other(&tx, "checked"), Some(Value::Bool(true)));
}

/// A ScrollBar's `Kind` is its orientation, and the canvas and both
/// targets read it from the box's shape. Turning it swaps the width and the
/// height, so the two never disagree; the same `Kind` again changes nothing.
#[test]
fn set_property_kind_swaps_the_scroll_bar_box() {
    let (d, r, a) = scenario();
    let mut doc = d.ir().clone();
    let bar = r
        .instantiate(
            "ScrollBar",
            "scroll_bar1",
            Rect::new(10.0, 20.0, 120.0, 16.0),
        )
        .unwrap();
    doc.add_child("form_main", bar).unwrap();
    let d = Doc::new(doc);
    let action = by_id("component.set_property").expect("action is in catalogue");
    let kind = |k: &str| {
        Args::texts("nodes", ["scroll_bar1"])
            .and("key", Value::Str("kind".into()))
            .and("value", Value::Enum(k.into()))
    };

    let tx = (action.execute)(&ctx_with(&d, &r, &a, kind("sbVertical")))
        .transaction
        .expect("produces transaction");
    let rect = tx.commands.iter().find_map(|c| match c {
        Command::SetProp {
            key,
            value: Some(Value::Rect(r)),
            ..
        } if key == "rect" => Some(*r),
        _ => None,
    });
    assert_eq!(
        rect,
        Some(Rect::new(10.0, 20.0, 16.0, 120.0)),
        "the box turns upright"
    );

    let tx = (action.execute)(&ctx_with(&d, &r, &a, kind("sbHorizontal")))
        .transaction
        .expect("produces transaction");
    assert!(
        !tx.commands
            .iter()
            .any(|c| matches!(c, Command::SetProp { key, .. } if key == "rect")),
        "already horizontal: the box stays as it is"
    );
}

/// **Key with target prefix passes without class declaring it.**
///
/// Written promise of contract: *"runtimes that do not know it must
/// preserve it in file and ignore it at runtime"*. Refusing here
/// would close escape hatch of ADR 0002 — existing for what
/// is specific to a target.
#[test]
fn set_property_allows_target_prefixed_key_through() {
    let (d, r, a) = scenario();
    let action = by_id("component.set_property").expect("action is in catalogue");
    let args = Args::texts("nodes", ["Button1"])
        .and("key", Value::Str("makepad.invented".into()))
        .and("value", Value::Bool(true));
    let e = (action.execute)(&ctx_with(&d, &r, &a, args));
    assert!(
        e.transaction.is_some(),
        "target key is preserved, not judged by class"
    );
}

#[test]
fn classic_and_modern_keymap_schemes_maintain_parity_without_conflict() {
    for &scheme in KeymapScheme::ALL {
        let cat = catalogue_with_scheme(scheme);
        let mut seen = Vec::new();
        for a in &cat {
            if let Some(sc) = a.shortcut {
                // One key, two owners, is allowed only for twins of disjoint
                // views (the components' Copy in the designer, the text's in
                // the code): the view in front decides which runs.
                let twin_of_other_view = cat.iter().any(|b| {
                    b.id != a.id
                        && b.shortcut == Some(sc)
                        && matches!(
                            (a.view, b.view),
                            (ViewAffinity::Design, ViewAffinity::Code)
                                | (ViewAffinity::Code, ViewAffinity::Design)
                        )
                });
                if twin_of_other_view {
                    continue;
                }
                assert!(
                    !seen.contains(&sc),
                    "duplicate shortcut in scheme {:?}: `{}` in `{}`",
                    scheme,
                    sc,
                    a.id
                );
                seen.push(sc);
            }
        }
        assert!(
            seen.len() >= 18,
            "scheme {:?} has fewer than 18 shortcuts: {}",
            scheme,
            seen.len()
        );
    }

    // Specific differences between schemes
    let run_modern =
        by_id_with_scheme("project.run", KeymapScheme::Modern).and_then(|a| a.shortcut);
    let run_classic =
        by_id_with_scheme("project.run", KeymapScheme::Classic).and_then(|a| a.shortcut);
    assert_eq!(run_modern, Some("F5"));
    assert_eq!(run_classic, Some("F9"));

    let bld_modern =
        by_id_with_scheme("project.build_manager", KeymapScheme::Modern).and_then(|a| a.shortcut);
    let bld_classic =
        by_id_with_scheme("project.build_manager", KeymapScheme::Classic).and_then(|a| a.shortcut);
    assert_eq!(bld_modern, Some("Ctrl+Shift+B"));
    assert_eq!(bld_classic, Some("Shift+F9"));

    let redo_modern = by_id_with_scheme("edit.redo", KeymapScheme::Modern).and_then(|a| a.shortcut);
    let redo_classic =
        by_id_with_scheme("edit.redo", KeymapScheme::Classic).and_then(|a| a.shortcut);
    assert_eq!(redo_modern, Some("Ctrl+Y"));
    assert_eq!(redo_classic, Some("Ctrl+Shift+Z"));
}

#[test]
fn set_property_with_align_recalculates_layout_in_same_transaction() {
    let (d, r, a) = scenario();
    let action = by_id("component.set_property").expect("action is in catalogue");
    let args = Args::texts("nodes", ["Button1"])
        .and("key", Value::Str("align".into()))
        .and("value", Value::Enum("Bottom".into()));
    let e = (action.execute)(&ctx_with(&d, &r, &a, args));
    let tx = e.transaction.expect("produces transaction");

    // Transaction must contain SetProp AND SetRect of recalculated layout!
    let has_set_prop = tx
        .commands
        .iter()
        .any(|cmd| matches!(cmd, crawrad_doc::Command::SetProp { key, .. } if key == "align"));
    let has_set_rect = tx
        .commands
        .iter()
        .any(|cmd| matches!(cmd, crawrad_doc::Command::SetRect { node, .. } if node == "Button1"));
    assert!(has_set_prop, "transaction must contain align SetProp");
    assert!(
        has_set_rect,
        "transaction must contain SetRect computed by layout"
    );
}

#[test]
fn every_action_with_parameter_generates_valid_json_schema_across_entire_catalogue() {
    let actions = catalogue();
    let mut total_actions_with_param = 0;
    let mut total_params = 0;

    for action in actions {
        if action.params.is_empty() {
            continue;
        }
        total_actions_with_param += 1;
        let schema = action.input_schema();

        assert_eq!(
            schema["type"], "object",
            "schema of action `{}` must have type object",
            action.id
        );

        let props = schema["properties"]
            .as_object()
            .unwrap_or_else(|| panic!("properties missing or invalid in action `{}`", action.id));

        let required: Vec<&str> = schema["required"]
            .as_array()
            .unwrap_or_else(|| panic!("required missing or not array in action `{}`", action.id))
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();

        for p in action.params {
            total_params += 1;
            let param_schema = props.get(p.name).unwrap_or_else(|| {
                panic!(
                    "parameter `{}` not found in schema of `{}`",
                    p.name, action.id
                )
            });

            // Check requiredness -> required
            if p.required {
                assert!(
                    required.contains(&p.name),
                    "required parameter `{}` must be in required in action `{}`",
                    p.name,
                    action.id
                );
            } else {
                assert!(
                    !required.contains(&p.name),
                    "optional parameter `{}` must NOT be in required in action `{}`",
                    p.name,
                    action.id
                );
            }

            // Check list -> array
            if p.list {
                assert_eq!(
                    param_schema["type"], "array",
                    "list parameter `{}` must have type: array in action `{}`",
                    p.name, action.id
                );
                assert!(
                    param_schema["items"].is_object(),
                    "list parameter `{}` must have defined items in action `{}`",
                    p.name,
                    action.id
                );
            } else {
                match p.kind {
                    ParamKind::String | ParamKind::ComponentId => {
                        assert_eq!(param_schema["type"], "string");
                    }
                    ParamKind::Integer => {
                        assert_eq!(param_schema["type"], "integer");
                    }
                    ParamKind::Number => {
                        assert_eq!(param_schema["type"], "number");
                    }
                    ParamKind::Boolean => {
                        assert_eq!(param_schema["type"], "boolean");
                    }
                    ParamKind::Enum(variants) => {
                        assert_eq!(param_schema["type"], "string");
                        let enum_vals = param_schema["enum"]
                            .as_array()
                            .expect("enum variants present");
                        assert!(
                            !enum_vals.is_empty(),
                            "enum must have variants for parameter `{}` in action `{}`",
                            p.name,
                            action.id
                        );
                        if !variants.is_empty() {
                            assert_eq!(enum_vals.len(), variants.len());
                        }
                    }
                    ParamKind::Any => {
                        // Any does not restrict type
                    }
                }
            }
        }
    }

    assert!(
        total_actions_with_param >= 12,
        "expected at least 12 actions with parameters in catalogue, found {total_actions_with_param}"
    );
    assert!(
        total_params >= 25,
        "expected at least 25 parameters in catalogue, found {total_params}"
    );
}

#[test]
fn requiredness_maps_strictly_to_required_failing_if_inverted() {
    // Mutation test: if p.required mapping is inverted or tampered with, fails immediately
    let actions = catalogue();
    for action in actions {
        let schema = action.input_schema();
        let required: Vec<&str> = schema["required"]
            .as_array()
            .expect("required must be array")
            .iter()
            .map(|v| v.as_str().expect("required item must be string"))
            .collect();

        let expected_required: Vec<&str> = action
            .params
            .iter()
            .filter(|p| p.required)
            .map(|p| p.name)
            .collect();

        assert_eq!(
            required, expected_required,
            "required array of `{}` must strictly mirror required: true parameters",
            action.id
        );
    }
}

#[test]
fn list_parameter_generates_type_array_and_items() {
    let p_list = Param {
        name: "targets",
        kind: ParamKind::ComponentId,
        description: "target components",
        list: true,
        required: true,
        domain: None,
    };

    let schema = p_list.json_schema();
    assert_eq!(schema["type"], "array");
    assert_eq!(schema["description"], "target components");
    assert_eq!(schema["items"]["type"], "string");

    let p_simple = Param {
        name: "target",
        kind: ParamKind::ComponentId,
        description: "target component",
        list: false,
        required: true,
        domain: None,
    };
    let schema_s = p_simple.json_schema();
    assert_eq!(schema_s["type"], "string");
    assert_eq!(schema_s["description"], "target component");
    assert!(schema_s.get("items").is_none());
}

#[test]
fn every_parameter_with_domain_has_schema_enum_identical_to_domain() {
    let actions = catalogue();
    let mut checked = 0;
    for action in actions {
        for p in action.params {
            if let Some(domain) = p.domain {
                let schema = p.json_schema();
                let schema_enum: Vec<String> = schema["enum"]
                    .as_array()
                    .unwrap_or_else(|| {
                        panic!(
                            "param `{}` of action `{}` has domain but generated no enum in schema",
                            p.name, action.id
                        )
                    })
                    .iter()
                    .map(|v| v.as_str().unwrap().to_string())
                    .collect();

                let domain_ids: Vec<String> = (domain.values)()
                    .into_iter()
                    .map(|c| c.id.to_string())
                    .collect();

                assert_eq!(
                    schema_enum, domain_ids,
                    "schema enum must match domain.values() in action `{}` param `{}`",
                    action.id, p.name
                );
                checked += 1;
            }
        }
    }
    assert!(
        checked > 0,
        "there must be parameters with domain in catalogue"
    );
}

/// **The inlay hints switch is a view effect, with no document needed.**
///
/// the catalog had no way to turn the grey types off; only the
/// Options page will have it otherwise, and an agent cannot click there.
#[test]
fn the_inlay_hints_switch_is_a_view_effect() {
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);
    let action = by_id("editor.toggle_inlay_hints").expect("in the catalog");
    let e = (action.execute)(&c);
    assert_eq!(e.editor, Some(EditorCommand::ToggleInlayHints));
    assert!(
        e.transaction.is_none(),
        "showing hints does not change the form"
    );
    assert!(!e.is_empty());
}

/// **The shortcut table is English, like every screen.** It said `ou`.
#[test]
fn the_shortcut_table_is_english() {
    for &scheme in KeymapScheme::ALL {
        for (name, keys) in scheme.shortcut_table() {
            assert!(!keys.contains(" ou "), "{name}: {keys}");
        }
    }
}

/// **`editor.go_to` puts the caret where it is told; column 1 when left out;
/// no line, nothing.** It is how an agent reaches a name in the code.
#[test]
fn go_to_asks_the_view_for_the_line_and_column() {
    let (d, r, a) = scenario();
    let mut c = ctx(&d, &r, &a);
    let action = by_id("editor.go_to").expect("in the catalog");
    c.args = Args::one("line", Value::Int(12));
    assert_eq!(
        (action.execute)(&c).editor,
        Some(EditorCommand::GoTo {
            line: 12,
            column: 1
        })
    );
    c.args = Args::one("line", Value::Int(12)).and("column", Value::Int(7));
    assert_eq!(
        (action.execute)(&c).editor,
        Some(EditorCommand::GoTo {
            line: 12,
            column: 7
        })
    );
    c.args = Args::none();
    assert!((action.execute)(&c).is_empty());
}

/// **A bookmark action carries its number; outside 1–9 it asks nothing.**
#[test]
fn bookmark_actions_carry_their_number() {
    let (d, r, a) = scenario();
    let mut c = ctx(&d, &r, &a);
    let set = by_id("editor.toggle_bookmark").expect("in the catalog");
    let go = by_id("editor.go_to_bookmark").expect("in the catalog");
    c.args = Args::one("number", Value::Int(3));
    assert_eq!(
        (set.execute)(&c).editor,
        Some(EditorCommand::ToggleBookmark(3))
    );
    assert_eq!(
        (go.execute)(&c).editor,
        Some(EditorCommand::GoToBookmark(3))
    );
    for wrong in [0, 10] {
        c.args = Args::one("number", Value::Int(wrong));
        assert!((set.execute)(&c).is_empty(), "{wrong} is not a bookmark");
    }
}

/// **MCP's JSON arguments become the catalog's `Args`**: integers stay
/// integers, other numbers floats, lists and maps nest, null is null.
#[test]
fn json_arguments_become_args() {
    use crawrad_ir::Value;
    let v = super::value_from_json(&serde_json::json!({
        "n": 3, "f": 1.5, "s": "x", "b": true, "l": [1, "y"], "z": null
    }));
    let Value::Map(m) = v else { panic!("{v:?}") };
    assert_eq!(m["n"], Value::Int(3));
    assert_eq!(m["f"], Value::Float(1.5));
    assert_eq!(m["s"], Value::Str("x".into()));
    assert_eq!(m["b"], Value::Bool(true));
    assert_eq!(
        m["l"],
        Value::List(vec![Value::Int(1), Value::Str("y".into())])
    );
    assert_eq!(m["z"], Value::Null);
    let args = super::args_from_json(&serde_json::json!({ "line": 12 }));
    assert_eq!(args.as_number("line"), Some(12.0));
}

#[test]
fn catalog_disk_conflict_actions() {
    for (id, label, hint, cmd) in [
        (
            "editor.keep_mine",
            "Keep My Version",
            "Keep your version of the file and overwrite disk on next save",
            EditorCommand::KeepMine,
        ),
        (
            "editor.reload_from_disk",
            "Reload from Disk",
            "Discard your edits and reload the file from disk",
            EditorCommand::ReloadFromDisk,
        ),
        (
            "editor.compare_with_disk",
            "Compare with Disk",
            "Open diff review comparing your edits with the disk copy",
            EditorCommand::CompareWithDisk,
        ),
    ] {
        let a = by_id(id).expect("must be in catalog");
        assert_eq!(a.label, label);
        assert_eq!(a.hint, hint);
        assert_eq!(a.group, "File");
        assert_eq!(a.place, Place::submenu(1, "Changed on Disk", 0));
        assert_eq!(a.shortcut, None);
        let (d, r, actions) = scenario();
        let e = (a.execute)(&ctx(&d, &r, &actions));
        assert_eq!(e.editor, Some(cmd));
        assert!(a.enabled.allows(&ctx(&d, &r, &actions)));
        assert!(!a.enabled.allows(&ctx_without_document(&r, &actions)));
    }
}

#[test]
fn every_action_has_non_empty_group() {
    for a in catalogue() {
        assert!(!a.group.is_empty(), "action {} has empty group", a.id);
    }
}

#[test]
fn every_submenu_has_at_least_two_rows() {
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);
    for &m in Menu::ALL {
        let top = menu::rows_with_scheme(m, &c, KeymapScheme::Modern);
        for row in top {
            if let Some(sub) = row.submenu {
                let sub_rows = menu::submenu_rows(m, &sub, &c, KeymapScheme::Modern);
                let command_count = sub_rows.iter().filter(|r| !r.separator).count();
                assert!(
                    command_count >= 2,
                    "submenu `{sub}` in menu {:?} has fewer than 2 rows (found {command_count})",
                    m
                );
            }
        }
    }
}

#[test]
fn view_menu_top_rows_section_order() {
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);
    let rows = menu::rows_with_scheme(Menu::View, &c, KeymapScheme::Modern);

    // exactly one Panels and one Zoom row and no zoom/panel item left at the top level
    let panels_count = rows
        .iter()
        .filter(|r| r.submenu.as_deref() == Some("Panels"))
        .count();
    let scale_count = rows
        .iter()
        .filter(|r| r.submenu.as_deref() == Some("Zoom"))
        .count();
    assert_eq!(
        panels_count, 1,
        "expected exactly 1 Panels submenu row in View"
    );
    assert_eq!(
        scale_count, 1,
        "expected exactly 1 Zoom submenu row in View"
    );

    for r in &rows {
        if let Some(act) = &r.action {
            assert!(
                !act.starts_with("view.zoom")
                    && !act.starts_with("view.ui_zoom")
                    && !act.starts_with("layout.toggle_panel"),
                "found `{act}` at top level of View menu"
            );
        }
    }

    // Split by rules and verify 4 sections
    let sections: Vec<&[Row]> = rows.split(|r| r.separator).collect();
    assert_eq!(
        sections.len(),
        4,
        "View menu should have 4 sections separated by rules"
    );

    // Section 0: View toggles
    assert!(sections[0].iter().all(|r| r.submenu.is_none()));
    // Section 1: Panels and Zoom
    assert_eq!(sections[1].len(), 2);
    assert_eq!(sections[1][0].label, "Panels");
    assert_eq!(sections[1][1].label, "Zoom");
    // Section 2: Inlay hints
    assert_eq!(sections[2].len(), 1);
    assert_eq!(
        sections[2][0].action.as_deref(),
        Some("editor.toggle_inlay_hints")
    );
    // Section 3: layout.reset and view.command_palette
    assert_eq!(sections[3].len(), 2);
    assert_eq!(sections[3][0].action.as_deref(), Some("layout.reset"));
    assert_eq!(
        sections[3][1].action.as_deref(),
        Some("view.command_palette")
    );
}

#[test]
fn submenu_rows_panels_and_scale() {
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);

    let panels = menu::submenu_rows(Menu::View, "Panels", &c, KeymapScheme::Modern);
    assert_eq!(
        panels.len(),
        Panel::ALL.len(),
        "submenu_rows(View, Panels) should have 1 row per panel"
    );
    for (i, p) in Panel::ALL.iter().enumerate() {
        assert_eq!(panels[i].panel_state, Some(*p));
        assert_eq!(panels[i].action.as_deref(), Some("layout.toggle_panel"));
    }

    let scale = menu::submenu_rows(Menu::View, "Zoom", &c, KeymapScheme::Modern);
    assert_eq!(scale.len(), 7, "Zoom should have 6 commands and 1 rule");
    let commands = scale.iter().filter(|r| !r.separator).count();
    let rules = scale.iter().filter(|r| r.separator).count();
    assert_eq!(commands, 6);
    assert_eq!(rules, 1);
    assert_eq!(scale[0].action.as_deref(), Some("view.zoom_in"));
    assert_eq!(scale[1].action.as_deref(), Some("view.zoom_out"));
    assert_eq!(scale[2].action.as_deref(), Some("view.zoom_reset"));
    assert!(scale[3].separator);
    assert_eq!(scale[4].action.as_deref(), Some("view.ui_zoom_in"));
    assert_eq!(scale[5].action.as_deref(), Some("view.ui_zoom_out"));
    assert_eq!(scale[6].action.as_deref(), Some("view.ui_zoom_reset"));
}

#[test]
fn go_refactor_and_file_changed_on_disk() {
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);

    // Go:
    // section 1: editor.goto_definition, editor.find_references
    // section 2: editor.navigate_back, editor.navigate_forward
    // section 3: editor.show_hover, editor.parameter_hints, editor.context_menu
    // section 4: form.open_base
    let go_rows = menu::rows_with_scheme(Menu::Go, &c, KeymapScheme::Modern);
    let go_sections: Vec<&[Row]> = go_rows.split(|r| r.separator).collect();
    assert_eq!(go_sections.len(), 4);
    let sec0: Vec<&str> = go_sections[0]
        .iter()
        .filter_map(|r| r.action.as_deref())
        .collect();
    assert_eq!(
        sec0,
        vec!["editor.goto_definition", "editor.find_references"]
    );
    let sec1: Vec<&str> = go_sections[1]
        .iter()
        .filter_map(|r| r.action.as_deref())
        .collect();
    assert_eq!(
        sec1,
        vec!["editor.navigate_back", "editor.navigate_forward"]
    );
    let sec2: Vec<&str> = go_sections[2]
        .iter()
        .filter_map(|r| r.action.as_deref())
        .collect();
    assert_eq!(
        sec2,
        vec![
            "editor.show_hover",
            "editor.parameter_hints",
            "editor.context_menu"
        ]
    );
    let sec3: Vec<&str> = go_sections[3]
        .iter()
        .filter_map(|r| r.action.as_deref())
        .collect();
    assert_eq!(sec3, vec!["form.open_base"]);

    // Refactor:
    // section 1: editor.rename_symbol, editor.code_actions, chat.fix_explain (apply_code_action skipped)
    // section 2: editor.format_document
    // section 3: ai.add_selection_to_chat
    let ref_rows = menu::rows_with_scheme(Menu::Refactor, &c, KeymapScheme::Modern);
    let ref_sections: Vec<&[Row]> = ref_rows.split(|r| r.separator).collect();
    assert_eq!(ref_sections.len(), 3);
    let rsec0: Vec<&str> = ref_sections[0]
        .iter()
        .filter_map(|r| r.action.as_deref())
        .collect();
    assert_eq!(
        rsec0,
        vec![
            "editor.rename_symbol",
            "editor.code_actions",
            "chat.fix_explain"
        ]
    );
    let rsec1: Vec<&str> = ref_sections[1]
        .iter()
        .filter_map(|r| r.action.as_deref())
        .collect();
    assert_eq!(rsec1, vec!["editor.format_document"]);
    let rsec2: Vec<&str> = ref_sections[2]
        .iter()
        .filter_map(|r| r.action.as_deref())
        .collect();
    assert_eq!(
        rsec2,
        vec![
            "ai.add_file_to_chat",
            "ai.add_selection_to_chat",
            "ai.explain_selection",
            "ai.review_selection"
        ]
    );

    // File ends with Changed on Disk row:
    let file_rows = menu::rows_with_scheme(Menu::File, &c, KeymapScheme::Modern);
    let last_row = file_rows.last().expect("File menu has rows");
    assert_eq!(last_row.submenu.as_deref(), Some("Changed on Disk"));
    assert_eq!(last_row.label, "Changed on Disk");

    let disk_rows = menu::submenu_rows(Menu::File, "Changed on Disk", &c, KeymapScheme::Modern);
    let disk_actions: Vec<&str> = disk_rows
        .iter()
        .filter_map(|r| r.action.as_deref())
        .collect();
    assert_eq!(
        disk_actions,
        vec![
            "editor.keep_mine",
            "editor.reload_from_disk",
            "editor.compare_with_disk"
        ]
    );
}

#[test]
fn no_menu_starts_or_ends_with_rule_and_no_adjacent_rules() {
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);

    for &m in Menu::ALL {
        let rows = menu::rows_with_scheme(m, &c, KeymapScheme::Modern);
        assert!(!rows.is_empty(), "menu {:?} is empty", m);
        assert!(
            !rows.first().unwrap().separator,
            "menu {:?} starts with rule",
            m
        );
        assert!(
            !rows.last().unwrap().separator,
            "menu {:?} ends with rule",
            m
        );
        for w in rows.windows(2) {
            assert!(
                !(w[0].separator && w[1].separator),
                "menu {:?} has adjacent rules",
                m
            );
        }

        for row in &rows {
            if let Some(sub) = &row.submenu {
                let sub_rows = menu::submenu_rows(m, sub, &c, KeymapScheme::Modern);
                assert!(!sub_rows.is_empty(), "submenu `{sub}` in {:?} is empty", m);
                assert!(
                    !sub_rows.first().unwrap().separator,
                    "submenu `{sub}` in {:?} starts with rule",
                    m
                );
                assert!(
                    !sub_rows.last().unwrap().separator,
                    "submenu `{sub}` in {:?} ends with rule",
                    m
                );
                for w in sub_rows.windows(2) {
                    assert!(
                        !(w[0].separator && w[1].separator),
                        "submenu `{sub}` in {:?} has adjacent rules",
                        m
                    );
                }
            }
        }
    }
}

/// The arrows walk only the rows that can run, wrapping at both ends, and a
/// panel is as tall as its rows (a rule thinner), never less than one.
#[test]
fn menu_arrows_and_heights() {
    let mut muted = menu::Row::custom("Muted", None);
    muted.enabled = false;
    let rows = vec![
        menu::Row::custom("A", None),
        menu::Row::rule(),
        muted,
        menu::Row::custom("B", None),
    ];
    assert_eq!(menu::next_live_row(&rows, None, true), Some(0));
    assert_eq!(menu::next_live_row(&rows, None, false), Some(3));
    assert_eq!(menu::next_live_row(&rows, Some(0), true), Some(3));
    assert_eq!(menu::next_live_row(&rows, Some(3), true), Some(0));
    assert_eq!(menu::next_live_row(&rows, Some(0), false), Some(3));
    assert_eq!(menu::next_live_row(&[], None, true), None);
    assert_eq!(menu::rows_height(&rows, 22.0, 7.0), 73.0);
    assert_eq!(menu::panel_height(&rows, 22.0, 7.0, 4.0), 81.0);
    assert_eq!(menu::panel_height(&[], 22.0, 7.0, 4.0), 30.0);
}

// ---------------------------------------------------------------------------
// DATA BINDING IN OBJECT INSPECTOR
// ---------------------------------------------------------------------------

#[test]
fn edit_shows_data_category_while_button_and_multi_selection_do_not() {
    let mut d = Document::new("form_main", "Form");
    let mut edit1 = Node::new("Edit1", "Edit");
    edit1.set_rect(Rect::new(10.0, 10.0, 100.0, 24.0));
    let mut edit2 = Node::new("Edit2", "Edit");
    edit2.set_rect(Rect::new(10.0, 40.0, 100.0, 24.0));
    let mut btn = Node::new("Button1", "Button");
    btn.set_rect(Rect::new(10.0, 70.0, 100.0, 24.0));
    d.add_child("form_main", edit1).unwrap();
    d.add_child("form_main", edit2).unwrap();
    d.add_child("form_main", btn).unwrap();

    let reg = Registry::builtin();
    let empty_set = std::collections::BTreeSet::new();

    // 1. Single Edit selection
    let rows_edit = inspector_model::rows(
        &reg,
        Some(&d),
        &["Edit1".to_string()],
        "",
        inspector_model::Tab::Properties,
        &empty_set,
        &empty_set,
    );
    let cat_labels: Vec<&str> = rows_edit
        .iter()
        .filter_map(|r| match r {
            inspector_model::Row::Category { label, .. } => Some(*label),
            _ => None,
        })
        .collect();
    assert!(cat_labels.contains(&DATA_CATEGORY));
    let app_idx = cat_labels.iter().position(|&l| l == "Appearance").unwrap();
    let data_idx = cat_labels.iter().position(|&l| l == DATA_CATEGORY).unwrap();
    assert_eq!(data_idx, app_idx + 1);

    let prop_keys: Vec<&str> = rows_edit
        .iter()
        .filter_map(|r| match r {
            inspector_model::Row::Prop(p) => Some(p.key),
            _ => None,
        })
        .collect();
    assert!(prop_keys.contains(&"binding.dataset"));
    assert!(prop_keys.contains(&"binding.field"));
    assert!(prop_keys.contains(&"binding.mode"));

    // 2. Single Button selection does not show Data category
    let rows_btn = inspector_model::rows(
        &reg,
        Some(&d),
        &["Button1".to_string()],
        "",
        inspector_model::Tab::Properties,
        &empty_set,
        &empty_set,
    );
    let btn_cats: Vec<&str> = rows_btn
        .iter()
        .filter_map(|r| match r {
            inspector_model::Row::Category { label, .. } => Some(*label),
            _ => None,
        })
        .collect();
    assert!(!btn_cats.contains(&DATA_CATEGORY));

    // 3. Two Edits selected does not show Data category
    let rows_multi = inspector_model::rows(
        &reg,
        Some(&d),
        &["Edit1".to_string(), "Edit2".to_string()],
        "",
        inspector_model::Tab::Properties,
        &empty_set,
        &empty_set,
    );
    let multi_cats: Vec<&str> = rows_multi
        .iter()
        .filter_map(|r| match r {
            inspector_model::Row::Category { label, .. } => Some(*label),
            _ => None,
        })
        .collect();
    assert!(!multi_cats.contains(&DATA_CATEGORY));
}

#[test]
fn inspector_binding_values_and_bound_property_mute_row() {
    let mut d = Document::new("form_main", "Form");
    let mut edit = Node::new("Edit1", "Edit");
    edit.set_rect(Rect::new(10.0, 10.0, 100.0, 24.0));
    edit.set_binding(
        "text",
        crawrad_ir::binding::BindingDef::new("dst1", crawrad_ir::binding::BindingMode::TwoWay)
            .with_field("name"),
    );
    d.add_child("form_main", edit).unwrap();

    let reg = Registry::builtin();
    let empty_set = std::collections::BTreeSet::new();

    let rows = inspector_model::rows(
        &reg,
        Some(&d),
        &["Edit1".to_string()],
        "",
        inspector_model::Tab::Properties,
        &empty_set,
        &empty_set,
    );

    let text_row = rows
        .iter()
        .find_map(|r| match r {
            inspector_model::Row::Prop(p) if p.key == "text" => Some(p),
            _ => None,
        })
        .expect("text property row must exist");
    assert_eq!(text_row.editor.template(), "RowMute");
    // a mute row is two captions: the value, then why it does not edit
    let captions: Vec<String> = text_row
        .editor
        .cells()
        .into_iter()
        .filter_map(|c| match c {
            inspector_model::Cell::Caption { text, .. } => Some(text),
            _ => None,
        })
        .collect();
    assert_eq!(
        captions,
        vec!["from dst1.name".to_string(), BOUND_TO_DATASET.to_string()]
    );

    let field_row = rows
        .iter()
        .find_map(|r| match r {
            inspector_model::Row::Prop(p) if p.key == "binding.field" => Some(p),
            _ => None,
        })
        .expect("binding.field row must exist");
    let field_val = field_row
        .editor
        .cells()
        .into_iter()
        .find_map(|c| match c {
            inspector_model::Cell::Field { text, .. } => Some(text),
            _ => None,
        })
        .expect("field text");
    assert_eq!(field_val, "name");

    let mut d2 = Document::new("form_main", "Form");
    let mut edit2 = Node::new("Edit1", "Edit");
    edit2.set_rect(Rect::new(10.0, 10.0, 100.0, 24.0));
    edit2.set_binding(
        "text",
        crawrad_ir::binding::BindingDef::new("dst1", crawrad_ir::binding::BindingMode::TwoWay),
    );
    d2.add_child("form_main", edit2).unwrap();

    let rows2 = inspector_model::rows(
        &reg,
        Some(&d2),
        &["Edit1".to_string()],
        "",
        inspector_model::Tab::Properties,
        &empty_set,
        &empty_set,
    );
    let text_row2 = rows2
        .iter()
        .find_map(|r| match r {
            inspector_model::Row::Prop(p) if p.key == "text" => Some(p),
            _ => None,
        })
        .expect("text row exists");
    let field_text2 = text_row2
        .editor
        .cells()
        .into_iter()
        .find_map(|c| match c {
            inspector_model::Cell::Caption { text, .. } => Some(text),
            _ => None,
        })
        .expect("value caption");
    assert_eq!(field_text2, "from dst1");
}

/// The built-in catalogue plus a non-visual `Dataset` that provides `DbDataset`,
/// shaped like the one `components/portadb-crawrad` installs.
fn registry_with_dataset() -> Registry {
    let mut r = Registry::builtin();
    let mut dataset = crawrad_registry::external::ExternalComponent::from(
        r.get("Timer").expect("Timer is built in"),
    );
    dataset.class = "Dataset".into();
    dataset.display = "Dataset".into();
    dataset.base_name = "dataset".into();
    dataset.prefix = "dst".into();
    dataset.provides = vec!["DbDataset".into()];
    r.register_external(dataset)
        .expect("the catalogue takes the dataset");
    r
}

#[test]
fn set_property_binding_gesture_lifecycle_and_undo() {
    let mut d = Document::new("form_main", "Form");
    let mut edit = Node::new("Edit1", "Edit");
    edit.set_rect(Rect::new(10.0, 10.0, 100.0, 24.0));
    d.add_child("form_main", edit).unwrap();
    d.add_child("form_main", Node::new("dst1", "Dataset"))
        .unwrap();

    let mut doc = Doc::new(d);
    let reg = registry_with_dataset();
    let clip = Clipboard::default();
    let action = by_id("component.set_property").expect("set_property in catalogue");

    // 0. a component that is not a dataset is refused
    let args_wrong = Args::texts("nodes", ["Edit1"])
        .and("key", Value::Str("binding.dataset".into()))
        .and("value", Value::Ref("Edit1".into()));
    let eff_wrong = (action.execute)(&ctx_with(&doc, &reg, &clip, args_wrong));
    assert!(eff_wrong.transaction.is_none());
    assert!(
        eff_wrong
            .status
            .as_deref()
            .unwrap_or("")
            .contains("is not a dataset")
    );

    // 1. binding.field without binding is refused with "pick a Dataset first"
    let args_field = Args::texts("nodes", ["Edit1"])
        .and("key", Value::Str("binding.field".into()))
        .and("value", Value::Str("name".into()));
    let eff_field = (action.execute)(&ctx_with(&doc, &reg, &clip, args_field));
    assert!(eff_field.transaction.is_none());
    assert!(
        eff_field
            .status
            .as_deref()
            .unwrap_or("")
            .contains("pick a Dataset first")
    );

    // 2. binding.dataset = dst1 creates binding with TwoWay mode
    let args_ds = Args::texts("nodes", ["Edit1"])
        .and("key", Value::Str("binding.dataset".into()))
        .and("value", Value::Ref("dst1".into()));
    let eff_ds = (action.execute)(&ctx_with(&doc, &reg, &clip, args_ds));
    let tx_ds = eff_ds.transaction.expect("transaction created");
    assert_eq!(tx_ds.label, "Edit1.text (binding)");
    doc.apply(tx_ds).unwrap();

    let b = doc.ir().node("Edit1").unwrap().binding("text").unwrap();
    assert_eq!(b.source, "dst1");
    assert_eq!(b.mode, crawrad_ir::binding::BindingMode::TwoWay);
    assert_eq!(b.field, None);

    // ONE undo removes the binding
    doc.undo().unwrap();
    assert!(doc.ir().node("Edit1").unwrap().binding("text").is_none());

    // Re-apply binding
    doc.redo().unwrap();
    assert!(doc.ir().node("Edit1").unwrap().binding("text").is_some());

    // 3. binding.mode changes only mode
    let args_mode = Args::texts("nodes", ["Edit1"])
        .and("key", Value::Str("binding.mode".into()))
        .and("value", Value::Enum("OneWay".into()));
    let eff_mode = (action.execute)(&ctx_with(&doc, &reg, &clip, args_mode));
    let tx_mode = eff_mode.transaction.expect("transaction created");
    doc.apply(tx_mode).unwrap();

    let b_mode = doc.ir().node("Edit1").unwrap().binding("text").unwrap();
    assert_eq!(b_mode.source, "dst1");
    assert_eq!(b_mode.mode, crawrad_ir::binding::BindingMode::OneWay);
    assert_eq!(b_mode.field, None);

    // 4. binding.field sets field
    let args_field2 = Args::texts("nodes", ["Edit1"])
        .and("key", Value::Str("binding.field".into()))
        .and("value", Value::Str("email".into()));
    let eff_field2 = (action.execute)(&ctx_with(&doc, &reg, &clip, args_field2));
    let tx_field2 = eff_field2.transaction.expect("transaction created");
    doc.apply(tx_field2).unwrap();

    let b_field = doc.ir().node("Edit1").unwrap().binding("text").unwrap();
    assert_eq!(b_field.source, "dst1");
    assert_eq!(b_field.mode, crawrad_ir::binding::BindingMode::OneWay);
    assert_eq!(b_field.field.as_deref(), Some("email"));

    // 5. binding.dataset empty removes binding
    let args_empty = Args::texts("nodes", ["Edit1"])
        .and("key", Value::Str("binding.dataset".into()))
        .and("value", Value::Ref("".into()));
    let eff_empty = (action.execute)(&ctx_with(&doc, &reg, &clip, args_empty));
    let tx_empty = eff_empty.transaction.expect("transaction created");
    doc.apply(tx_empty).unwrap();
    assert!(doc.ir().node("Edit1").unwrap().binding("text").is_none());
}

#[test]
fn binding_set_action_status_for_bindable_and_non_bindable_props() {
    let mut d = Document::new("form_main", "Form");
    let mut edit = Node::new("Edit1", "Edit");
    edit.set_rect(Rect::new(10.0, 10.0, 100.0, 24.0));
    let mut btn = Node::new("Button1", "Button");
    btn.set_rect(Rect::new(10.0, 40.0, 100.0, 24.0));
    d.add_child("form_main", edit).unwrap();
    d.add_child("form_main", btn).unwrap();

    let doc = Doc::new(d);
    let reg = Registry::builtin();
    let clip = Clipboard::default();
    let action = by_id("binding.set").expect("binding.set in catalogue");

    // 1. Edit.text is bindable: no "not supported" status
    let args_edit = Args::none()
        .and("node", Value::Str("Edit1".into()))
        .and("prop", Value::Str("text".into()))
        .and("source", Value::Str("dst1".into()));
    let eff_edit = (action.execute)(&ctx_with(&doc, &reg, &clip, args_edit));
    assert!(eff_edit.transaction.is_some());
    assert!(
        !eff_edit
            .status
            .as_deref()
            .unwrap_or("")
            .contains("not supported")
    );

    // 2. Button.text is not marked bindable: retains "not supported" status
    let args_btn = Args::none()
        .and("node", Value::Str("Button1".into()))
        .and("prop", Value::Str("text".into()))
        .and("source", Value::Str("dst1".into()));
    let eff_btn = (action.execute)(&ctx_with(&doc, &reg, &clip, args_btn));
    assert!(eff_btn.transaction.is_some());
    assert!(
        eff_btn
            .status
            .as_deref()
            .unwrap_or("")
            .contains("data binding is not supported by the generated app yet")
    );
}

#[test]
fn data_category_dataset_row_lists_the_datasets_of_the_form() {
    let mut d = Document::new("form_main", "Form");
    d.add_child("form_main", Node::new("Edit1", "Edit"))
        .unwrap();
    d.add_child("form_main", Node::new("dst1", "Dataset"))
        .unwrap();
    d.add_child("form_main", Node::new("Button1", "Button"))
        .unwrap();
    let reg = registry_with_dataset();
    let none = std::collections::BTreeSet::new();

    let rows = inspector_model::rows(
        &reg,
        Some(&d),
        &["Edit1".to_string()],
        "",
        inspector_model::Tab::Properties,
        &none,
        &none,
    );
    let dataset_row = rows
        .iter()
        .find_map(|r| match r {
            inspector_model::Row::Prop(p) if p.key == "binding.dataset" => Some(p),
            _ => None,
        })
        .expect("the Data category has a Dataset row");
    let options: Vec<String> = dataset_row
        .editor
        .cells()
        .into_iter()
        .find_map(|c| match c {
            inspector_model::Cell::Choice { options, .. } => Some(options),
            _ => None,
        })
        .expect("the Dataset row is a choice");
    assert!(options.iter().any(|o| o.contains("dst1")), "{options:?}");
    assert!(
        !options.iter().any(|o| o.contains("Button1")),
        "{options:?}"
    );
}

#[test]
fn one_bound_component_mutes_the_property_for_the_whole_selection() {
    let mut d = Document::new("form_main", "Form");
    let mut bound = Node::new("Edit1", "Edit");
    bound.set_binding(
        "text",
        crawrad_ir::binding::BindingDef::new("dst1", crawrad_ir::binding::BindingMode::TwoWay)
            .with_field("name"),
    );
    d.add_child("form_main", bound).unwrap();
    d.add_child("form_main", Node::new("Edit2", "Edit"))
        .unwrap();
    let reg = Registry::builtin();
    let none = std::collections::BTreeSet::new();

    for selection in [
        vec!["Edit1".to_string(), "Edit2".to_string()],
        vec!["Edit2".to_string(), "Edit1".to_string()],
    ] {
        let rows = inspector_model::rows(
            &reg,
            Some(&d),
            &selection,
            "",
            inspector_model::Tab::Properties,
            &none,
            &none,
        );
        let text_row = rows
            .iter()
            .find_map(|r| match r {
                inspector_model::Row::Prop(p) if p.key == "text" => Some(p),
                _ => None,
            })
            .expect("text row");
        assert_eq!(text_row.editor.template(), "RowMute", "{selection:?}");
    }
}

#[test]
fn every_binding_mode_survives_its_name() {
    for mode in [
        crawrad_ir::BindingMode::OneWay,
        crawrad_ir::BindingMode::TwoWay,
        crawrad_ir::BindingMode::OneWayToSource,
        crawrad_ir::BindingMode::OneTime,
    ] {
        assert_eq!(parse_binding_mode(binding_mode_name(mode)), mode);
    }
    assert_eq!(
        crawrad_registry::props::BINDING_MODES.len(),
        4,
        "the Mode list offers exactly the four modes"
    );
    for name in crawrad_registry::props::BINDING_MODES {
        assert_eq!(binding_mode_name(parse_binding_mode(name)), *name);
    }
}

#[test]
fn repeating_the_same_binding_gesture_adds_no_undo_step() {
    let mut d = Document::new("form_main", "Form");
    d.add_child("form_main", Node::new("Edit1", "Edit"))
        .unwrap();
    d.add_child("form_main", Node::new("dst1", "Dataset"))
        .unwrap();
    let mut doc = Doc::new(d);
    let reg = registry_with_dataset();
    let clip = Clipboard::default();
    let action = by_id("component.set_property").expect("set_property in catalogue");
    let pick = || {
        Args::texts("nodes", ["Edit1"])
            .and("key", Value::Str("binding.dataset".into()))
            .and("value", Value::Ref("dst1".into()))
    };
    let first = (action.execute)(&ctx_with(&doc, &reg, &clip, pick()));
    doc.apply(first.transaction.expect("the first pick binds"))
        .unwrap();
    let again = (action.execute)(&ctx_with(&doc, &reg, &clip, pick()));
    assert!(
        again.transaction.is_none(),
        "the same Dataset again changes nothing"
    );
}

/// No `crawrad-dbprobe` next to the IDE: the Field row keeps the text box and
/// says why under it, instead of staying silently a text box.
#[test]
fn field_row_says_why_when_the_columns_cannot_be_read() {
    let mut d = Document::new("form_main", "Form");
    let mut edit = Node::new("Edit1", "Edit");
    edit.set_binding(
        "text",
        crawrad_ir::binding::BindingDef::new("dst1", crawrad_ir::binding::BindingMode::TwoWay)
            .with_field("name"),
    );
    d.add_child("form_main", edit).unwrap();
    d.add_child("form_main", Node::new("dst1", "Dataset"))
        .unwrap();
    let reg = registry_with_dataset();
    let empty_set = std::collections::BTreeSet::new();
    let rows = inspector_model::rows_with_columns(
        &reg,
        Some(&d),
        &["Edit1".to_string()],
        "",
        inspector_model::Tab::Properties,
        &empty_set,
        &empty_set,
        &|_| Some(Err("crawrad-dbprobe not found next to the IDE".to_string())),
    );
    let field_row = rows
        .iter()
        .find_map(|r| match r {
            inspector_model::Row::Prop(p) if p.key == "binding.field" => Some(p),
            _ => None,
        })
        .expect("binding.field row exists");
    assert_eq!(field_row.editor.template(), "RowWhenBad");
    let cells = field_row.editor.cells();
    assert!(cells.contains(&inspector_model::Cell::Field {
        slot: inspector_model::Slot::Value,
        text: "name".to_string(),
    }));
    assert!(cells.contains(&inspector_model::Cell::Caption {
        slot: inspector_model::Slot::Note,
        text: "Columns not listed: crawrad-dbprobe not found next to the IDE".to_string(),
    }));
    // still free text: typing a field writes it
    assert!(matches!(
        field_row
            .editor
            .gesture(inspector_model::Slot::Value, inspector_model::Gesture::Typed("id")),
        inspector_model::Outcome::Set(Value::Str(ref s)) if s == "id"
    ));
}

#[test]
fn field_row_becomes_choice_when_dataset_columns_are_known() {
    let mut d = Document::new("form_main", "Form");
    let mut edit = Node::new("Edit1", "Edit");
    edit.set_binding(
        "text",
        crawrad_ir::binding::BindingDef::new("dst1", crawrad_ir::binding::BindingMode::TwoWay)
            .with_field("name"),
    );
    d.add_child("form_main", edit).unwrap();
    d.add_child("form_main", Node::new("dst1", "Dataset"))
        .unwrap();

    let reg = registry_with_dataset();
    let empty_set = std::collections::BTreeSet::new();

    let cols = vec!["id".to_string(), "name".to_string()];
    let rows = inspector_model::rows_with_columns(
        &reg,
        Some(&d),
        &["Edit1".to_string()],
        "",
        inspector_model::Tab::Properties,
        &empty_set,
        &empty_set,
        &|source| {
            if source == "dst1" {
                Some(Ok(cols.clone()))
            } else {
                None
            }
        },
    );

    let field_row = rows
        .iter()
        .find_map(|r| match r {
            inspector_model::Row::Prop(p) if p.key == "binding.field" => Some(p),
            _ => None,
        })
        .expect("binding.field row exists");

    assert_eq!(field_row.editor.template(), "RowChoice");
    let cells = field_row.editor.cells();
    let choice_cell = cells
        .into_iter()
        .find_map(|c| match c {
            inspector_model::Cell::Choice {
                options, current, ..
            } => Some((options, current)),
            _ => None,
        })
        .expect("Cell::Choice");
    assert_eq!(choice_cell.0, vec!["id", "name"]);
    assert_eq!(choice_cell.1, Some(1));

    // only Field takes the columns: Dataset and Mode keep their own editors
    for key in ["binding.dataset", "binding.mode"] {
        let row = rows
            .iter()
            .find_map(|r| match r {
                inspector_model::Row::Prop(p) if p.key == key => Some(p),
                _ => None,
            })
            .expect("Data row exists");
        let lists_columns = row.editor.cells().into_iter().any(
            |c| matches!(c, inspector_model::Cell::Choice { ref options, .. } if options == &cols),
        );
        assert!(!lists_columns, "{key} must not list the columns");
    }
}

#[test]
fn field_row_stays_text_when_columns_are_none() {
    let mut d = Document::new("form_main", "Form");
    let mut edit = Node::new("Edit1", "Edit");
    edit.set_binding(
        "text",
        crawrad_ir::binding::BindingDef::new("dst1", crawrad_ir::binding::BindingMode::TwoWay)
            .with_field("name"),
    );
    d.add_child("form_main", edit).unwrap();
    d.add_child("form_main", Node::new("dst1", "Dataset"))
        .unwrap();

    let reg = registry_with_dataset();
    let empty_set = std::collections::BTreeSet::new();

    let rows = inspector_model::rows_with_columns(
        &reg,
        Some(&d),
        &["Edit1".to_string()],
        "",
        inspector_model::Tab::Properties,
        &empty_set,
        &empty_set,
        &|_| None,
    );

    let field_row = rows
        .iter()
        .find_map(|r| match r {
            inspector_model::Row::Prop(p) if p.key == "binding.field" => Some(p),
            _ => None,
        })
        .expect("binding.field row exists");

    assert_ne!(field_row.editor.template(), "RowChoice");
}

#[test]
fn columns_closure_is_not_consulted_without_binding() {
    let mut d = Document::new("form_main", "Form");
    d.add_child("form_main", Node::new("Edit1", "Edit"))
        .unwrap();
    d.add_child("form_main", Node::new("dst1", "Dataset"))
        .unwrap();

    let reg = registry_with_dataset();
    let empty_set = std::collections::BTreeSet::new();

    let consulted = std::cell::Cell::new(false);
    let rows = inspector_model::rows_with_columns(
        &reg,
        Some(&d),
        &["Edit1".to_string()],
        "",
        inspector_model::Tab::Properties,
        &empty_set,
        &empty_set,
        &|_| {
            consulted.set(true);
            Some(Ok(vec!["id".to_string()]))
        },
    );

    assert!(
        !consulted.get(),
        "closure must not be consulted when there is no binding"
    );

    let field_row = rows
        .iter()
        .find_map(|r| match r {
            inspector_model::Row::Prop(p) if p.key == "binding.field" => Some(p),
            _ => None,
        })
        .expect("binding.field row exists");

    assert_ne!(field_row.editor.template(), "RowChoice");
}

#[test]
fn a_field_missing_from_the_columns_heads_the_field_list() {
    let reg = registry_with_dataset();
    let empty_set = std::collections::BTreeSet::new();
    for (field, expected) in [
        (Some("gone"), vec!["gone", "id", "name"]),
        (None, vec!["", "id", "name"]),
    ] {
        let mut d = Document::new("form_main", "Form");
        let mut edit = Node::new("Edit1", "Edit");
        let mut b =
            crawrad_ir::binding::BindingDef::new("dst1", crawrad_ir::binding::BindingMode::TwoWay);
        if let Some(f) = field {
            b = b.with_field(f);
        }
        edit.set_binding("text", b);
        d.add_child("form_main", edit).unwrap();
        d.add_child("form_main", Node::new("dst1", "Dataset"))
            .unwrap();
        let rows = inspector_model::rows_with_columns(
            &reg,
            Some(&d),
            &["Edit1".to_string()],
            "",
            inspector_model::Tab::Properties,
            &empty_set,
            &empty_set,
            &|_| Some(Ok(vec!["id".to_string(), "name".to_string()])),
        );
        let row = rows
            .iter()
            .find_map(|r| match r {
                inspector_model::Row::Prop(p) if p.key == "binding.field" => Some(p),
                _ => None,
            })
            .expect("binding.field row exists");
        let cell = row
            .editor
            .cells()
            .into_iter()
            .find_map(|c| match c {
                inspector_model::Cell::Choice {
                    options, current, ..
                } => Some((options, current)),
                _ => None,
            })
            .expect("Cell::Choice");
        assert_eq!(
            cell,
            (expected.iter().map(|s| s.to_string()).collect(), Some(0)),
            "{field:?}"
        );
    }
}

/// `Add Selection to Chat` needs the code editor in front AND a selection.
#[test]
fn add_selection_to_chat_needs_code_and_a_selection() {
    let (r, clipboard) = (Registry::builtin(), Clipboard::default());
    let a = by_id("ai.add_selection_to_chat").expect("in the catalog");
    let with = |code_in_front, selection| Context {
        text: EditorText {
            code_in_front,
            selection,
            ..Default::default()
        },
        ..ctx_without_document(&r, &clipboard)
    };
    assert!(a.enabled.allows(&with(true, true)));
    assert!(!a.enabled.allows(&with(true, false)), "nothing selected");
    assert!(
        !a.enabled.allows(&with(false, true)),
        "the designer is in front"
    );
}

/// The other chat actions of the editor: Add File needs only code in
/// front; Explain and Review also need a selection.
#[test]
fn editor_chat_actions_follow_code_and_selection() {
    let (r, clipboard) = (Registry::builtin(), Clipboard::default());
    let with = |code_in_front, selection| Context {
        text: EditorText {
            code_in_front,
            selection,
            ..Default::default()
        },
        ..ctx_without_document(&r, &clipboard)
    };
    for (id, needs_selection) in [
        ("ai.add_file_to_chat", false),
        ("ai.explain_selection", true),
        ("ai.review_selection", true),
    ] {
        let a = by_id(id).expect("in the catalog");
        assert!(a.enabled.allows(&with(true, true)), "{id}");
        assert_eq!(
            a.enabled.allows(&with(true, false)),
            !needs_selection,
            "{id}"
        );
        assert!(
            !a.enabled.allows(&with(false, true)),
            "{id}: the designer is in front"
        );
    }
}

/// **The Explorer's path actions carry the `path` they were given**:
/// the folder or file named by the argument reaches the effect, and no
/// argument means an empty path, never a guess.
#[test]
fn explorer_path_actions_carry_their_path_argument() {
    let (doc, reg, clip) = scenario();
    for id in [
        "project.set_active",
        "explorer.new_file",
        "explorer.new_form",
        "explorer.new_folder",
        "explorer.open",
        "explorer.reveal",
        "explorer.copy_path",
        "explorer.copy_rel_path",
        "explorer.rename",
        "explorer.delete",
    ] {
        let action = by_id(id).unwrap();
        let given = ctx_with(&doc, &reg, &clip, Args::text("path", "proj/src/area.rs"));
        let e = format!("{:?}", (action.execute)(&given));
        assert!(e.contains("proj/src/area.rs"), "{id} lost its path: {e}");
        let e = format!("{:?}", (action.execute)(&ctx(&doc, &reg, &clip)));
        assert!(!e.contains("area.rs"), "{id} invented a path: {e}");
    }
}

/// The Run split button: Run under the debugger, then Run Without Debugging,
/// each with the active keymap scheme's key.
#[test]
fn run_menu_rows_follow_the_keymap_scheme() {
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);
    let keys = |scheme| {
        menu::run_menu_rows(&c, scheme, &[], None)
            .into_iter()
            .filter(|row| {
                row.action.as_deref() == Some("project.run")
                    || row.action.as_deref() == Some("project.run_without_debugging")
            })
            .map(|row| {
                (
                    row.action.unwrap_or_default(),
                    row.shortcut.unwrap_or_default(),
                )
            })
            .collect::<Vec<_>>()
    };
    assert_eq!(
        keys(KeymapScheme::Classic),
        [
            ("project.run".into(), "F9".into()),
            (
                "project.run_without_debugging".into(),
                "Ctrl+Shift+F9".into()
            )
        ]
    );
    assert_eq!(
        keys(KeymapScheme::Modern),
        [
            ("project.run".into(), "F5".into()),
            ("project.run_without_debugging".into(), "Ctrl+F5".into())
        ]
    );
}

#[test]
fn run_menu_rows_includes_language_section() {
    let (d, r, a) = scenario();
    let c = ctx(&d, &r, &a);
    let codes = vec!["de".to_string(), "pt-BR".to_string()];
    let rows = menu::run_menu_rows(&c, KeymapScheme::Modern, &codes, Some("de"));

    // First two are run actions
    assert_eq!(rows[0].action.as_deref(), Some("project.run"));
    assert_eq!(
        rows[1].action.as_deref(),
        Some("project.run_without_debugging")
    );
    // Separator rule
    assert!(rows[2].separator);
    // Section title
    assert_eq!(rows[3].label, "Run in a language");
    assert_eq!(rows[3].tone, menu::Tone::Title);

    // Language options: System (unchecked), English (unchecked), Deutsch (checked), Português (unchecked)
    assert_eq!(rows[4].label, "   System language");
    assert_eq!(rows[5].label, "   English");
    assert_eq!(rows[6].label, "✓ Deutsch");
    assert_eq!(rows[7].label, "   Português (Brasil)");
}

#[test]
fn designer_language_rows_check_the_previewed_language_and_send_its_code() {
    let codes = vec!["de".to_string(), "pt-BR".to_string()];
    let rows = menu::designer_language_rows(&codes, Some("de"));
    let labels: Vec<&str> = rows.iter().map(|r| r.label.as_ref()).collect();
    assert_eq!(
        labels,
        ["   English (source)", "✓ Deutsch", "   Português (Brasil)"]
    );
    assert!(
        rows.iter()
            .all(|r| r.action.as_deref() == Some("designer.language"))
    );
    assert_eq!(rows[0].args.as_text("code"), Some(""));
    assert_eq!(rows[1].args.as_text("code"), Some("de"));
}

#[test]
fn shows_preview_language_is_true_only_when_project_has_languages() {
    assert!(!menu::shows_preview_language(&[]));
    assert!(menu::shows_preview_language(&["pt-BR".to_string()]));
}

/// Renames `Button1` (default handler `Button1_click`) with the given
/// `everywhere` and returns the handler after the rename and after Undo.
fn rename_handler_after(everywhere: Option<bool>) -> (String, String) {
    let (d, r, a) = scenario();
    let mut ir = d.ir().clone();
    ir.node_mut("Button1")
        .unwrap()
        .events
        .insert("on_click".into(), "Button1_click".into());
    let mut doc = Doc::new(ir);
    let action = by_id("component.rename").expect("action is in catalogue");
    let mut args = Args::text("node", "Button1").and("name", Value::Str("btn_ok".into()));
    if let Some(e) = everywhere {
        args = args.and("everywhere", Value::Bool(e));
    }
    let e = (action.execute)(&ctx_with(&doc, &r, &a, args));
    doc.apply(e.transaction.expect("produces transaction"))
        .unwrap();
    let handler = |doc: &Doc, name: &str| {
        let node = doc.ir().node("Button1").unwrap();
        assert_eq!(node.name(), name);
        node.events.get("on_click").cloned().unwrap_or_default()
    };
    let after = handler(&doc, "btn_ok");
    doc.undo().unwrap();
    (after, handler(&doc, "Button1"))
}

#[test]
fn rename_everywhere_false_keeps_handler() {
    let (after, undone) = rename_handler_after(Some(false));
    assert_eq!(
        (after.as_str(), undone.as_str()),
        ("Button1_click", "Button1_click")
    );
}

#[test]
fn rename_everywhere_defaults_to_true() {
    let (after, undone) = rename_handler_after(None);
    assert_eq!(
        (after.as_str(), undone.as_str()),
        ("btn_ok_click", "Button1_click")
    );
}

#[test]
fn project_build_all_enabled_only_for_workspace() {
    let (doc, reg, clip) = scenario();
    let action = by_id("project.build_all").expect("project.build_all in catalogue");
    let mut c = ctx(&doc, &reg, &clip);

    c.project.is_workspace = true;
    c.project.open = true;
    assert!(action.enabled.allows(&c));

    c.project.is_workspace = false;
    assert!(!action.enabled.allows(&c));

    c.project.is_workspace = true;
    c.project.open = false;
    assert!(!action.enabled.allows(&c));
}

#[test]
fn project_build_all_produces_build_all_effect() {
    let (doc, reg, clip) = scenario();
    let action = by_id("project.build_all").expect("project.build_all in catalogue");
    let mut c = ctx(&doc, &reg, &clip);
    c.project.is_workspace = true;
    let effect = (action.execute)(&c);
    assert_eq!(effect.file, Some(FileEffect::BuildAll));
}
