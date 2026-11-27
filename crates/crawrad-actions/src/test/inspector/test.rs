//! The tests of the Object Inspector family.
//!
//! They answer the owner's ruler: *can an agent do this by tool, with no
//! mouse?* Every assertion below runs with no window open — which is the whole
//! point of the command existing before the click.

use crate::{Args, Author, Context, Doc, InspectorCommand, Project, by_group, by_id, catalogue};
use crawrad_designer::clipboard::Clipboard;
use crawrad_ir::{Document, Rect};
use crawrad_registry::Registry;

/// A form with a `Memo` inside — it has `text`, which is the property
/// with a window behind it, and lacks `items`, which serves as a counterexample.
fn scenario() -> (Doc, Registry, Clipboard) {
    let registry = Registry::builtin();
    let mut ir = Document::new("form_main", "Form");
    let memo = registry
        .instantiate(
            "Memo",
            "Memo1".to_string(),
            Rect::new(8.0, 8.0, 120.0, 60.0),
        )
        .expect("catalogue has a Memo");
    ir.add_child("form_main", memo).expect("hang Memo on root");
    (Doc::new(ir), registry, Clipboard::default())
}

fn ctx<'a>(d: Option<&'a Doc>, r: &'a Registry, a: &'a Clipboard, args: Args) -> Context<'a> {
    Context {
        doc: d,
        registry: r,
        clipboard: a,
        args,
        project: Project::default(),
        author: Author::User,
        editor_dirty: false,
        text: Default::default(),
    }
}

/// Runs the action by id and returns the Inspector command it requested.
fn command(id: &str, args: Args) -> Option<InspectorCommand> {
    let (d, r, a) = scenario();
    let c = ctx(Some(&d), &r, &a, args);
    let action = by_id(id).unwrap_or_else(|| panic!("`{id}` is not in the catalogue"));
    (action.execute)(&c).inspector
}

/// The refusal sentence, for tests demanding that it EXISTS.
fn refusal(id: &str, args: Args) -> String {
    let (d, r, a) = scenario();
    let c = ctx(Some(&d), &r, &a, args);
    let e = (by_id(id).unwrap().execute)(&c);
    assert!(e.inspector.is_none(), "`{id}` acted when it should refuse");
    e.status.unwrap_or_default()
}

#[test]
fn the_three_openings_are_in_the_catalogue() {
    // Hand-written list on PURPOSE: it is the published contract, and one
    // derived from the catalogue would pass green on the day an action vanished.
    for id in [
        "inspector.open_editor",
        "inspector.close_editor",
        "inspector.toggle_group",
    ] {
        assert!(by_id(id).is_some(), "`{id}` vanished from the catalogue");
    }
}

#[test]
fn opening_the_editor_carries_the_node_and_the_key() {
    assert_eq!(
        command(
            "inspector.open_editor",
            Args::text("node", "Memo1").and("key", crawrad_ir::Value::Str("text".into()))
        ),
        Some(InspectorCommand::OpenEditor {
            node: "Memo1".to_string(),
            key: "text".to_string(),
        }),
    );
}

#[test]
fn closing_requests_no_argument() {
    assert_eq!(
        command("inspector.close_editor", Args::none()),
        Some(InspectorCommand::CloseEditor),
    );
}

#[test]
fn folding_a_group_carries_only_the_key() {
    assert_eq!(
        command("inspector.toggle_group", Args::text("key", "margins")),
        Some(InspectorCommand::ToggleGroup {
            key: "margins".to_string(),
        }),
    );
}

/// **Without both arguments it refuses, and states which was missing.**
///
/// Opening "the editor of nothing at all" is the blank window that nobody knows
/// where it came from — and refusing in silence is the usual dead field.
#[test]
fn without_node_or_without_key_opening_refuses_and_explains() {
    let sentence = refusal("inspector.open_editor", Args::text("key", "text"));
    assert!(
        sentence.contains("node"),
        "the refusal does not name `node`: {sentence:?}"
    );
    let sentence = refusal("inspector.open_editor", Args::text("node", "Memo1"));
    assert!(
        sentence.contains("key"),
        "the refusal does not name `key`: {sentence:?}"
    );
    let sentence = refusal("inspector.toggle_group", Args::none());
    assert!(
        sentence.contains("key"),
        "the refusal does not name `key`: {sentence:?}"
    );
}

/// **A node that does not exist does not open a window.**
///
/// An outdated id from a document already closed would open a window over nothing, and
/// whatever it saved would vanish into the void.
#[test]
fn nonexistent_node_does_not_open_window() {
    let sentence = refusal(
        "inspector.open_editor",
        Args::text("node", "Fantasma1").and("key", crawrad_ir::Value::Str("text".into())),
    );
    assert!(
        sentence.contains("Fantasma1"),
        "the refusal does not name the component: {sentence:?}"
    );
}

/// **A key the CLASS does not have does not open a window.**
///
/// The check is in the catalogue, not in the shell, because it applies equally to the
/// click and to the protocol (ADR 0005): `Memo` does not have `items`, and any window
/// that opened would save a property that nothing draws.
#[test]
fn key_outside_class_catalogue_does_not_open_window() {
    let sentence = refusal(
        "inspector.open_editor",
        Args::text("node", "Memo1").and("key", crawrad_ir::Value::Str("items".into())),
    );
    assert!(
        sentence.contains("items") && sentence.contains("Memo"),
        "the refusal does not state what mismatched: {sentence:?}"
    );
}

/// **Without a document there is no panel, and the predicate states that.**
#[test]
fn without_document_there_is_nothing_to_open() {
    let (_, r, a) = scenario();
    let c = ctx(None, &r, &a, Args::none());
    for action in catalogue()
        .into_iter()
        .filter(|x| x.id.starts_with("inspector."))
    {
        assert!(
            !action.enabled.allows(&c),
            "`{}` lit up without an open form — there is no property \
             in this state",
            action.id
        );
    }
}

/// **None of the three write to the document.**
///
/// This is what separates the family: an agent reading `Effect` to find out what
/// will happen must be able to conclude *"this does not touch the form"* without
/// executing anything. What the window WRITES returns through
/// `component.set_property`, not through here.
#[test]
fn opening_is_not_editing() {
    for (id, args) in [
        (
            "inspector.open_editor",
            Args::text("node", "Memo1").and("key", crawrad_ir::Value::Str("text".into())),
        ),
        ("inspector.close_editor", Args::none()),
        ("inspector.toggle_group", Args::text("key", "margins")),
    ] {
        let (d, r, a) = scenario();
        let c = ctx(Some(&d), &r, &a, args);
        let e = (by_id(id).unwrap().execute)(&c);
        assert!(e.transaction.is_none(), "`{id}` returned a transaction");
        assert!(e.selection.is_none(), "`{id}` modified the selection");
        assert!(e.file.is_none(), "`{id}` modified disk");
        assert!(e.layout.is_none(), "`{id}` modified panel arrangement");
        assert!(e.explorer.is_none(), "`{id}` modified Explorer");
    }
}

/// The group is `Inspector`, and **no menu bar item draws it**.
#[test]
fn the_group_belongs_to_the_panel_and_not_the_bar() {
    for action in catalogue()
        .into_iter()
        .filter(|x| x.id.starts_with("inspector."))
    {
        assert_eq!(action.group, "Inspector", "{} changed group", action.id);
    }
    for group in ["File", "Edit", "View", "Project", "Run", "Help"] {
        assert!(
            !by_group(group)
                .iter()
                .any(|a| a.id.starts_with("inspector.")),
            "an Inspector opening leaked into menu `{group}`"
        );
    }
}

/// The splitter is an action, so an agent moves it with no mouse; a
/// fractional width lands on a whole pixel, and an integer from JSON works too.
#[test]
fn the_splitter_carries_a_whole_pixel_width() {
    let set = |v| command("inspector.set_name_width", Args::one("width", v));
    assert_eq!(
        set(crawrad_ir::Value::Float(150.6)),
        Some(InspectorCommand::SetNameWidth { width: 151 })
    );
    assert_eq!(
        set(crawrad_ir::Value::Int(90)),
        Some(InspectorCommand::SetNameWidth { width: 90 })
    );
}

#[test]
fn the_splitter_refuses_no_width_and_says_so() {
    for args in [
        Args::none(),
        Args::text("width", "wide"),
        Args::one("width", crawrad_ir::Value::Float(f64::NAN)),
    ] {
        assert!(refusal("inspector.set_name_width", args).contains("`width` is required"));
    }
}

/// The request is kept as asked and clamped to the row at drawing time: the
/// values never fall under their minimum, the names never under theirs, and a
/// dock too narrow for both keeps the names.
#[test]
fn the_name_width_is_clamped_to_the_row() {
    use crate::inspector_model::{NAME_WIDTH_MIN, VALUE_WIDTH_MIN, name_width};
    assert_eq!(name_width(128.0, 300.0), 128.0);
    assert_eq!(name_width(9999.0, 300.0), 300.0 - VALUE_WIDTH_MIN);
    assert_eq!(name_width(-50.0, 300.0), NAME_WIDTH_MIN);
    assert_eq!(name_width(128.0, 80.0), NAME_WIDTH_MIN);
}

/// The shell's splitter keeps what was asked and draws it clamped: the strip
/// follows the drawn width, not the request, and before the first layout the
/// request is drawn as it is.
#[test]
fn the_name_splitter_draws_the_clamped_width_and_keeps_the_request() {
    use crate::inspector_model::{NameSplitter, ROW_CHROME, SPLITTER_OFFSET, VALUE_WIDTH_MIN};
    let mut s = NameSplitter::default();
    s.ask(9999);
    let list = 300.0;
    let widest = list - ROW_CHROME - VALUE_WIDTH_MIN;
    assert_eq!(s.layout(list), (widest, widest + SPLITTER_OFFSET));
    assert_eq!(
        s.requested, 9999.0,
        "a narrow dock does not lose the request"
    );
    assert_eq!(s.layout(0.0).0, 9999.0);
}

/// A form with an `Image` — the `Picture` property the `…` is for.
fn picture_scenario() -> (Doc, Registry, Clipboard) {
    let registry = Registry::builtin();
    let mut ir = Document::new("form_main", "Form");
    let image = registry
        .instantiate(
            "Image",
            "Image1".to_string(),
            Rect::new(8.0, 80.0, 64.0, 64.0),
        )
        .expect("catalogue has an Image");
    ir.add_child("form_main", image)
        .expect("hang Image on root");
    (Doc::new(ir), registry, Clipboard::default())
}

fn pick(args: Args) -> crate::Effect {
    let (d, r, a) = picture_scenario();
    let c = ctx(Some(&d), &r, &a, args);
    (by_id("inspector.pick_picture")
        .expect("pick_picture vanished")
        .execute)(&c)
}

fn image_args() -> Args {
    Args::text("node", "Image1").and("key", crawrad_ir::Value::Str("picture".into()))
}

#[test]
fn picking_a_picture_carries_the_node_the_key_and_no_file() {
    assert_eq!(
        pick(image_args()).inspector,
        Some(InspectorCommand::PickPicture {
            node: "Image1".to_string(),
            key: "picture".to_string(),
            file: None,
        }),
    );
    // An empty `file` is the dialog too, not a file named "".
    assert_eq!(
        pick(image_args().and("file", crawrad_ir::Value::Str(String::new()))).inspector,
        Some(InspectorCommand::PickPicture {
            node: "Image1".to_string(),
            key: "picture".to_string(),
            file: None,
        }),
    );
}

#[test]
fn an_agent_names_the_file_and_skips_the_dialog() {
    assert_eq!(
        pick(image_args().and("file", crawrad_ir::Value::Str("logo.png".into()))).inspector,
        Some(InspectorCommand::PickPicture {
            node: "Image1".to_string(),
            key: "picture".to_string(),
            file: Some("logo.png".to_string()),
        }),
    );
}

#[test]
fn picking_refuses_what_opening_refuses() {
    for (args, word) in [
        (Args::text("key", "picture"), "node"),
        (Args::text("node", "Image1"), "key"),
        (
            Args::text("node", "Ghost1").and("key", crawrad_ir::Value::Str("picture".into())),
            "Ghost1",
        ),
        (
            Args::text("node", "Image1").and("key", crawrad_ir::Value::Str("nonsense".into())),
            "nonsense",
        ),
    ] {
        let e = pick(args);
        assert!(
            e.inspector.is_none(),
            "pick_picture acted when it should refuse"
        );
        let sentence = e.status.unwrap_or_default();
        assert!(
            sentence.contains(word),
            "the refusal does not name `{word}`: {sentence:?}"
        );
    }
}

mod picture_row {
    use crate::Args;
    use crate::inspector_model::editors::picture::PictureEditor;
    use crate::inspector_model::{
        ACTION_PICK_PICTURE, Cell, Gesture, Outcome, PropEditor, Slot, icon,
    };
    use crawrad_ir::{Picture, Value};

    const PNG: &[u8] = b"\x89PNG\r\n\x1a\n\0\0\0\rIHDR";

    fn clear_text(e: &PictureEditor) -> String {
        e.cells()
            .into_iter()
            .find_map(|c| match c {
                Cell::Press {
                    slot: Slot::Clear,
                    text,
                } => Some(text),
                _ => None,
            })
            .expect("a picture row has a Clear cell")
    }

    #[test]
    fn an_empty_picture_draws_no_clear_and_ignores_it() {
        let e = PictureEditor::new("Image1".into(), "picture", Picture::empty());
        assert_eq!(clear_text(&e), "");
        assert!(matches!(
            e.gesture(Slot::Clear, Gesture::Pressed),
            Outcome::Ignore
        ));
    }

    #[test]
    fn a_held_picture_draws_the_clear_and_it_empties_the_property() {
        let held = Picture::from_bytes(PNG.to_vec()).with_source("logo.png");
        let e = PictureEditor::new("Image1".into(), "picture", held);
        assert_eq!(clear_text(&e), icon::CLEAR);
        match e.gesture(Slot::Clear, Gesture::Pressed) {
            Outcome::Set(Value::Picture(p)) => assert!(p.is_empty()),
            other => panic!("the `×` did not empty the picture: {other:?}"),
        }
    }

    #[test]
    fn the_ellipsis_runs_pick_picture_with_the_node_and_the_key() {
        let e = PictureEditor::new("Image1".into(), "picture", Picture::empty());
        match e.gesture(Slot::Button, Gesture::Pressed) {
            Outcome::Run(id, args) => {
                assert_eq!(id, ACTION_PICK_PICTURE);
                assert_eq!(
                    args,
                    Args::text("node", "Image1").and("key", Value::Str("picture".into()))
                );
            }
            other => panic!("the `…` did not run the action: {other:?}"),
        }
    }
}

/// The window `inspector.open_editor` shows is the row editor's to say.
#[test]
fn the_window_an_opening_shows_comes_from_the_row_editor() {
    use crate::inspector_model::{Tab, Window, rows, window_for};
    let (d, r, _) = scenario();
    let none = std::collections::BTreeSet::new();
    let memo = rows(
        &r,
        Some(d.ir()),
        &["Memo1".to_string()],
        "",
        Tab::Properties,
        &none,
        &none,
    );
    assert_eq!(window_for(&memo, "text"), Window::Lines);
    assert_eq!(window_for(&memo, "align"), Window::Pad);

    let form_rows = rows(
        &r,
        Some(d.ir()),
        &["form_main".to_string()],
        "",
        Tab::Properties,
        &none,
        &none,
    );
    assert_eq!(window_for(&form_rows, "color"), Window::Color);

    let mut ir = Document::new("form_main", "Form");
    let list = r
        .instantiate(
            "ListBox",
            "ListBox1".to_string(),
            Rect::new(8.0, 8.0, 120.0, 60.0),
        )
        .expect("catalogue has a ListBox");
    ir.add_child("form_main", list)
        .expect("hang ListBox on root");
    let list = rows(
        &r,
        Some(&ir),
        &["ListBox1".to_string()],
        "",
        Tab::Properties,
        &none,
        &none,
    );
    assert_eq!(window_for(&list, "items"), Window::Items);
}

// ---------------------------------------------------------------------------
// FONT INSPECTOR TESTS
// ---------------------------------------------------------------------------

#[test]
fn picking_a_font_action_carries_font_or_dialog_and_requires_document() {
    let (d, r, a) = scenario();
    let action = by_id("inspector.pick_font").expect("pick_font must be in catalogue");
    let c = ctx(
        Some(&d),
        &r,
        &a,
        Args::text("node", "Memo1").and("key", crawrad_ir::Value::Str("font".into())),
    );
    assert_eq!(
        (action.execute)(&c).inspector,
        Some(InspectorCommand::PickFont {
            node: "Memo1".to_string(),
            key: "font".to_string(),
            font: None,
        }),
    );

    let c_with_font = ctx(
        Some(&d),
        &r,
        &a,
        Args::text("node", "Memo1")
            .and("key", crawrad_ir::Value::Str("font".into()))
            .and("font", crawrad_ir::Value::Str("Arial, 12.0pt, Bold".into())),
    );
    assert_eq!(
        (action.execute)(&c_with_font).inspector,
        Some(InspectorCommand::PickFont {
            node: "Memo1".to_string(),
            key: "font".to_string(),
            font: Some("Arial, 12.0pt, Bold".to_string()),
        }),
    );

    let c_no_doc = Context {
        doc: None,
        registry: &r,
        clipboard: &a,
        args: Args::none(),
        project: Project::default(),
        author: Author::User,
        editor_dirty: false,
        text: Default::default(),
    };
    assert!(
        !action.enabled.allows(&c_no_doc),
        "without an open document, pick_font must be disabled"
    );
}

#[test]
fn font_editor_shut_has_three_cells_and_runs_toggle_and_pick() {
    use crate::inspector_model::editors::font::FontEditor;
    use crate::inspector_model::{
        ACTION_PICK_FONT, ACTION_TOGGLE_GROUP, Cell, Gesture, Outcome, PropEditor, Slot, icon,
    };
    use crawrad_ir::{Font, Value};

    let initial = Font::new("Segoe UI", 9.0).bold(true);
    let editor = FontEditor::new("Btn1".into(), "font", initial.clone(), false);

    assert_eq!(editor.template(), "RowFoldOpen");

    let cells = editor.cells();
    assert_eq!(cells.len(), 3);
    assert_eq!(
        cells[0],
        Cell::Press {
            slot: Slot::Fold,
            text: icon::FOLD_CLOSED.to_string(),
        }
    );
    assert_eq!(
        cells[1],
        Cell::Caption {
            slot: Slot::Value,
            text: Value::Font(initial).describe(),
        }
    );
    assert_eq!(
        cells[2],
        Cell::Press {
            slot: Slot::Button,
            text: icon::OPEN_EDITOR.to_string(),
        }
    );

    // Chevron runs ACTION_TOGGLE_GROUP
    match editor.gesture(Slot::Fold, Gesture::Pressed) {
        Outcome::Run(id, args) => {
            assert_eq!(id, ACTION_TOGGLE_GROUP);
            assert_eq!(args, Args::text("key", "font"));
        }
        other => panic!("chevron did not run toggle_group: {other:?}"),
    }

    // Ellipsis runs ACTION_PICK_FONT with node and key
    match editor.gesture(Slot::Button, Gesture::Pressed) {
        Outcome::Run(id, args) => {
            assert_eq!(id, ACTION_PICK_FONT);
            assert_eq!(
                args,
                Args::text("node", "Btn1").and("key", Value::Str("font".into()))
            );
        }
        other => panic!("ellipsis did not run pick_font: {other:?}"),
    }
}

#[test]
fn font_editor_open_has_seven_sub_rows_in_exact_order() {
    use crate::inspector_model::PropEditor;
    use crate::inspector_model::editors::font::FontEditor;
    use crawrad_ir::Font;

    let editor = FontEditor::new("Btn1".into(), "font", Font::default(), true);
    let children = editor.children();
    assert_eq!(children.len(), 7);

    assert_eq!(children[0].0, "Name");
    assert_eq!(children[0].1.template(), "SubChoice");

    assert_eq!(children[1].0, "Size");
    assert_eq!(children[1].1.template(), "SubText");

    assert_eq!(children[2].0, "Bold");
    assert_eq!(children[2].1.template(), "SubCheck");

    assert_eq!(children[3].0, "Italic");
    assert_eq!(children[3].1.template(), "SubCheck");

    assert_eq!(children[4].0, "Underline");
    assert_eq!(children[4].1.template(), "SubCheck");

    assert_eq!(children[5].0, "StrikeOut");
    assert_eq!(children[5].1.template(), "SubCheck");

    assert_eq!(children[6].0, "Color");
    assert_eq!(children[6].1.template(), "SubColor");
}

#[test]
fn font_sub_rows_write_whole_font_and_validate_inputs() {
    use crate::inspector_model::editors::font::FontEditor;
    use crate::inspector_model::{Cell, Gesture, Outcome, PropEditor, Slot};
    use crawrad_ir::{Color, Font, Value};

    let initial = Font::new("Arial", 10.0)
        .bold(false)
        .italic(false)
        .underline(false)
        .strike_out(false)
        .color(Color::transparent());
    let editor = FontEditor::new("Btn1".into(), "font", initial.clone(), true);
    let children = editor.children();

    // 1. Name chooses index
    let name_editor = &children[0].1;
    let available_options = match &name_editor.cells()[0] {
        Cell::Choice { options, .. } => options.clone(),
        other => panic!("expected Choice cell: {other:?}"),
    };
    if !available_options.is_empty() {
        match name_editor.gesture(Slot::Value, Gesture::Chose(0)) {
            Outcome::Set(Value::Font(f)) => {
                assert_eq!(f.name, available_options[0]);
                assert_eq!(f.size, initial.size);
                assert_eq!(f.bold, initial.bold);
            }
            other => panic!("Name choice did not return Outcome::Set(Font): {other:?}"),
        }
    }

    // 2. Size typed 12
    let size_editor = &children[1].1;
    match size_editor.gesture(Slot::Value, Gesture::Typed("12")) {
        Outcome::Set(Value::Font(f)) => {
            assert_eq!(f.size, 12.0);
            assert_eq!(f.name, initial.name);
        }
        other => panic!("Size did not set font with size 12: {other:?}"),
    }

    // Size 0 is clamped to the smallest size, as every number of the panel is
    match size_editor.gesture(Slot::Value, Gesture::Typed("0")) {
        Outcome::Set(Value::Font(f)) => assert_eq!(f.size, 1.0),
        other => panic!("Size 0 was not clamped: {other:?}"),
    }

    // Size abc refused
    match size_editor.gesture(Slot::Value, Gesture::Typed("abc")) {
        Outcome::Refuse(_) => {}
        other => panic!("Size 'abc' was not refused: {other:?}"),
    }

    // 3. Bold toggled
    let bold_editor = &children[2].1;
    match bold_editor.gesture(Slot::Value, Gesture::Toggled(true)) {
        Outcome::Set(Value::Font(f)) => {
            assert!(f.bold);
            assert_eq!(f.name, initial.name);
        }
        other => panic!("Bold did not toggle: {other:?}"),
    }

    // 4. Italic toggled
    let italic_editor = &children[3].1;
    match italic_editor.gesture(Slot::Value, Gesture::Toggled(true)) {
        Outcome::Set(Value::Font(f)) => {
            assert!(f.italic);
            assert_eq!(f.name, initial.name);
        }
        other => panic!("Italic did not toggle: {other:?}"),
    }

    // 5. Underline toggled
    let underline_editor = &children[4].1;
    match underline_editor.gesture(Slot::Value, Gesture::Toggled(true)) {
        Outcome::Set(Value::Font(f)) => {
            assert!(f.underline);
            assert_eq!(f.name, initial.name);
        }
        other => panic!("Underline did not toggle: {other:?}"),
    }

    // 6. StrikeOut toggled
    let strike_editor = &children[5].1;
    match strike_editor.gesture(Slot::Value, Gesture::Toggled(true)) {
        Outcome::Set(Value::Font(f)) => {
            assert!(f.strike_out);
            assert_eq!(f.name, initial.name);
        }
        other => panic!("StrikeOut did not toggle: {other:?}"),
    }

    // 7. Color typed #FF0000
    let color_editor = &children[6].1;
    match color_editor.gesture(Slot::Value, Gesture::Typed("#FF0000")) {
        Outcome::Set(Value::Font(f)) => {
            assert_eq!(f.color, Color::rgb(0xFF, 0x00, 0x00));
            assert_eq!(f.name, initial.name);
        }
        other => panic!("Color did not set #FF0000: {other:?}"),
    }

    // Color zzz refused
    match color_editor.gesture(Slot::Value, Gesture::Typed("zzz")) {
        Outcome::Refuse(why) => {
            assert!(why.contains("is not a colour"));
        }
        other => panic!("Color 'zzz' was not refused: {other:?}"),
    }
}

#[test]
fn font_name_current_is_none_when_family_is_not_installed() {
    use crate::inspector_model::editors::font::FontEditor;
    use crate::inspector_model::{Cell, PropEditor};
    use crawrad_ir::Font;

    let uninstalled = Font::new("NoSuchFontFamilyNeverEverInstalled12345", 9.0);
    let editor = FontEditor::new("Btn1".into(), "font", uninstalled, true);
    let children = editor.children();
    let name_row = &children[0].1;

    let cells = name_row.cells();
    match &cells[0] {
        Cell::Choice { current, .. } => {
            assert_eq!(
                *current, None,
                "uninstalled family must not select an arbitrary index"
            );
        }
        other => panic!("expected Choice cell on Name row: {other:?}"),
    }
}

#[test]
fn font_conversion_round_trips_and_preserves_alpha_rule() {
    use crate::inspector_model::{font_from_choice, font_to_choice};
    use crawrad_ir::{Color, Font};

    // Standard font round-trip
    let font = Font::new("Verdana", 11.5)
        .bold(true)
        .italic(true)
        .underline(false)
        .strike_out(true)
        .color(Color::rgb(0x12, 0x34, 0x56));

    let choice = font_to_choice(&font);
    assert_eq!(choice.name, "Verdana");
    assert_eq!(choice.size, 11.5);
    assert!(choice.bold);
    assert!(choice.italic);
    assert!(!choice.underline);
    assert!(choice.strike_out);
    assert_eq!(choice.rgb, [0x12, 0x34, 0x56]);

    let round_trip = font_from_choice(&choice, &font);
    assert_eq!(round_trip, font);

    // Alpha 0 (theme colour) preserved when RGB is unchanged by dialog
    let theme_font = Font::new("Segoe UI", 9.0).color(Color::transparent());
    let theme_choice = font_to_choice(&theme_font);
    assert_eq!(theme_choice.rgb, [0, 0, 0]);

    let theme_result = font_from_choice(&theme_choice, &theme_font);
    assert_eq!(
        theme_result.color.a, 0,
        "when dialog returns unchanged rgb [0, 0, 0], theme alpha 0 must be preserved"
    );

    // When RGB changes, alpha becomes 255
    let mut modified_choice = theme_choice;
    modified_choice.rgb = [0xFF, 0x00, 0x00];
    let colored_result = font_from_choice(&modified_choice, &theme_font);
    assert_eq!(colored_result.color, Color::rgb(0xFF, 0x00, 0x00));
    assert_eq!(colored_result.color.a, 255);
}

/// The `…` of a `Font` writes the agent's font; an empty answer is Cancel and
/// a font that does not read is refused.
#[test]
fn pick_font_reads_the_argument() {
    use crate::inspector_model::pick_font;
    let node = "n".to_string();
    let given = pick_font(None, &node, "font", Some("Verdana, 12.0pt, Bold"));
    let f = given.expect("a font was given").expect("it reads");
    assert_eq!((f.name.as_str(), f.size, f.bold), ("Verdana", 12.0, true));
    assert!(matches!(
        pick_font(None, &node, "font", Some(",,")),
        Some(Err(_))
    ));
    assert_eq!(
        pick_font(None, &node, "font", Some("")),
        None,
        "empty is Cancel"
    );
}

/// Each style box shows ITS flag of the font, and only it.
#[test]
fn font_style_boxes_show_their_own_flag() {
    use crate::inspector_model::editors::font::FontEditor;
    use crate::inspector_model::{Cell, PropEditor};
    use crawrad_ir::Font;
    let font = Font::new("Arial", 9.0).strike_out(true);
    let editor = FontEditor::new("Btn1".into(), "font", font, true);
    let shown: Vec<bool> = editor.children()[2..6]
        .iter()
        .map(|(_, e)| match e.cells().as_slice() {
            [Cell::Check { on, .. }] => *on,
            other => panic!("a style row is one check: {other:?}"),
        })
        .collect();
    assert_eq!(
        shown,
        [false, false, false, true],
        "Bold, Italic, Underline, StrikeOut"
    );
}

/// `view.get_inspector` reads the rows a person sees, sub-rows of an open
/// `Font` included, and the notice of an empty panel.
#[test]
fn rows_json_reports_what_the_panel_shows() {
    use crate::inspector_model::{Tab, rows, rows_json};
    let (d, r, _) = scenario();
    let none = std::collections::BTreeSet::new();
    let open: std::collections::BTreeSet<String> = ["font".to_string()].into();
    let list = rows(
        &r,
        Some(d.ir()),
        &["Memo1".to_string()],
        "",
        Tab::Properties,
        &open,
        &none,
    );
    let json = rows_json(&list);
    let shown = json["rows"].as_array().expect("rows");
    let font = shown
        .iter()
        .position(|row| row["key"] == "font")
        .expect("a Font row");
    assert_eq!(shown[font]["rect"], serde_json::Value::Null);
    assert_eq!(shown[font]["depth"], 0);
    assert_eq!(shown[font]["template"], "RowFoldOpen");
    // The shut row says the font, never the name of its type.
    assert_eq!(shown[font]["cells"][1]["text"], "(default)");
    let set = crawrad_ir::Value::Font(crawrad_ir::Font::default());
    assert_eq!(set.describe(), "Segoe UI, 9.0pt");
    assert_eq!(shown[font + 1]["label"], "Name");
    assert_eq!(shown[font + 1]["depth"], 1);
    assert_eq!(shown[font + 1]["cells"][0]["kind"], "choice");
    assert_eq!(shown[font + 3]["label"], "Bold");
    assert_eq!(
        shown[font + 3]["cells"][0],
        serde_json::json!({ "slot": "Value", "kind": "check", "on": false })
    );
    assert!(shown.iter().any(|row| row["category"].is_string()));

    // Measured on-screen geometry is attached when provided.
    use crate::inspector_model::rows_json_with_rects;
    let with_rect = rows_json_with_rects(&list, &[(font, [10.0, 20.0, 100.0, 24.0])]);
    assert_eq!(
        with_rect["rows"][font]["rect"],
        serde_json::json!([10.0, 20.0, 100.0, 24.0])
    );

    let empty = rows_json(&rows(&r, None, &[], "", Tab::Properties, &none, &none));
    assert!(empty["rows"][0]["notice"].is_string());
}
