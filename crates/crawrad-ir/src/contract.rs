//! The property contract between runtimes — ADR 0002.
//!
//! The rule, mechanized here:
//!
//! > A property only enters the **common core** if **all** runtimes
//! > can honor it. What is specific to a target lives in namespaced form:
//! > `win32.PasswordChar`, `egui.desired_width`.
//!
//! It is the rule that the classic RAD IDE did not follow — and that is why a classic form does not open
//! in its cross-platform library. It only works if enforced from the very first component, so it is
//! verified by `Document::validate` and does not rely on human discipline.

use crawrad_note::note;
use std::fmt;

#[note("contract-targets")]
pub const TARGETS: &[&str] = &["win32", "egui", "makepad"];

#[note("contract-known-targets")]
fn known_targets() -> String {
    TARGETS.join(", ")
}

#[note("contract-common")]
pub const COMMON: &[&str] = &[
    "name",      // identifier (mirrors node id; exists for the inspector)
    "text",      // caption / textual content
    "rect",      // position and size in design units
    "enabled",   // enabled
    "visible",   // visible
    "font",      // font
    "tab_order", // tab order
    "tab_stop",  // participates in tab order
    "anchors",   // anchors (left/top/right/bottom)
    "align",     // alTop, alClient… — automatic alignment from classic RAD IDE
    "hint",      // hint / tooltip
    "tag",       // user-defined free integer, as in the classic RAD IDE
    // The actions trio — TAction/TActionList/TImageList from classic RAD IDE. Entered the
    // common core because they are application semantics, not toolkit semantics: "this
    // button triggers this action, which is named like this and has this icon" is a
    // sentence that any target can generate.
    "action",       // component triggers this action
    "images",       // icon list used by action list
    "image_index",  // which icon in that list
    "shortcut",     // action keyboard shortcut ("Ctrl+S")
    "category",     // action category, like Category in the classic RAD IDE
    "image_width",  // default width of icons in the list
    "image_height", // default height of icons in the list
    "images_data",  // icons of the list, as Value::List of Value::Map
    "items",        // tree collection: the Items of TreeView, and whatever comes identically
    // The `Interval` of `TTimer`, in milliseconds. Enters the common core, and the
    // written answer that ADR 0002 requires is: **who honors this, and with
    // which call?**
    //
    // - `win32`: `SetTimer(hwnd, id, ms, NULL)` — the argument IS milliseconds.
    // - `egui`: `request_repaint_after` plus frame clock, which is what
    //   the generator emits and what Preview runs.
    // - `slint`, the generation target planned in ADR 0004: `Timer::start` with a
    //   `Duration`.
    //
    // They are three names, not one. And what crosses over is the PHRASE — "call this
    // handler approximately every N milliseconds" —, which is application
    // semantics, in the same vein as `shortcut` and `action`, and not toolkit
    // vocabulary like `ColorDepth` and `BkColor` were.
    //
    // What is NOT universal is precision and delayed tick accumulation: each
    // target drifts differently, and Windows' own `WM_TIMER` has a resolution of
    // tens of milliseconds. This is quality of service, and not another
    // meaning — it does not change what the property means, so it does not
    // disqualify it. Whoever chooses what to do with missed ticks is each
    // generator, and it writes the choice in the code it emits.
    "interval",
    // Which item in the collection is selected, with `-1` for none — ItemIndex
    // from classic RAD IDE. Entered the common core when both adapters of the time
    // honored it — `CB_SETCURSEL` in RCL, the `ComboBox` index in egui. Were it for
    // one alone, it would be `win32.item_index` — that is the yardstick, and it barred this
    // field until both painted it.
    "item_index",
    // Checked or not — Checked of CheckBox and RadioButton. Both
    // adapters of the time honored it: `BM_SETCHECK` in RCL, the widget `bool` in
    // RXP. RCL departed in ADR 0013; the key remains, and RXP honors it.
    "checked",
    // Promoted from `win32.*` when RCL departed (ADR 0013): both have
    // meaning in any target, and egui honors both. `password_char`
    // stores the character classic RAD IDE stores; egui paints its bullet and
    // ignores WHICH character it is — the information stays in the document, for the target
    // that knows how to use it.
    "password_char",
    "readonly",
    // Which page of `PageControl` is open — `ActivePageIndex` from
    // classic RAD IDE. It is application semantics, not toolkit semantics: "this control shows
    // the third page" is a sentence that any target can generate, and in
    // Win32 it would be `TCM_SETCURSEL`. It goes into the document for the same reason it
    // goes into DFM: the page open when designing is the one that opens when running.
    "active_page",
    // Frame thickness of `Panel` — `BorderWidth` of `TPanel`.
    //
    // **Found through mutation, and the mutation was of something else.** The test that was supposed
    // to guard ADR 0002 in the registry scanned a handwritten list of classes,
    // and `Panel` was not in it; when the test began scanning the entire
    // catalog, it turned up that `border_width` was neither here nor in namespaced
    // space. Real consequence: every `Panel` coming from the palette carried a
    // property that `validate()` rejects, so any document with a
    // panel failed in `Doc::preview` — the RTA path.
    //
    // Enters common, and the answer the ADR requires is: `win32` has
    // `TPanel.BorderWidth`, and egui paints the outline with the requested
    // thickness. Both honor it — and began honoring it in practice along with this
    // line; a key in common that nobody reads is exactly what ADR 0002
    // does not admit.
    "border_width",
    // The four margins of `TControl` and the switch directing `Align`
    // to honor them — `Margins` and `AlignWithMargins` from classic RAD IDE.
    //
    // The answer ADR 0002 requires — *who honors this, and with which call?*
    // — is uncommon, and is what qualifies both for the common core:
    // **no target makes any call. What honors it is the core.**
    //
    // Both do not describe an effect the toolkit needs to produce: they are
    // input to a calculation that ends in `Rect`, and that calculation runs once, in
    // `crawrad-designer` (`layout::align_children`), which saves the already
    // inset rectangle to the document. The target receives the rectangle without knowing that
    // margins exist — exactly as already happens with `align`, whose result
    // the generator emits as anchors and not as a cascade (`crawrad-codegen`, `rxp.rs`).
    //
    // They go into the document for the same reason they go into DFM: without them
    // saved, reopening the form would lose the INTENT and leave only the
    // rectangle, and recalculating alignment the next time the form changed
    // size would push everything right back against the border. And they stay in common, and not in
    // namespaced space, because "this control wants 3 padding around it" is an
    // application sentence that every target can generate — in the same vein as
    // `shortcut` and `interval`, and not toolkit vocabulary like
    // `ColorDepth` was.
    "margins",
    "align_with_margins",
    // The `ScrollBars` of `TMemo` and components with configurable scrollbar:
    // "ssNone", "ssHorizontal", "ssVertical", "ssBoth".
    "scroll_bars",
    // ScrollBar and ScrollBox properties (TScrollBar and TScrollBox from classic RAD IDE):
    "kind",
    "min",
    "max",
    "position",
    "small_change",
    "large_change",
    "auto_scroll",
    "horz_scroll_bar",
    "vert_scroll_bar",
];

// Were deliberately left **out** of here, and the rule above is the reason:
// `ColorDepth` (cd32Bit) is comctl32 vocabulary and `BkColor` is the transparency
// color that only Win32 ImageList has — neither means
// anything in egui. They live as `win32.ColorDepth` and `win32.BkColor`, and the
// file preserves them for whoever understands them.

#[note("contract-propkey")]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PropKey {
    /// From common core: portable across all runtimes.
    Common(String),
    /// Specific to a target. Runtimes that do not know it must **preserve it**
    /// in the file and ignore it at runtime.
    Target { target: String, name: String },
}

impl PropKey {
    #[note("contract-propkey-parse")]
    pub fn parse(raw: &str) -> Result<Self, ContractError> {
        match raw.split_once('.') {
            None => {
                if raw.is_empty() {
                    return Err(ContractError::MalformedKey(raw.to_string()));
                }
                Ok(Self::Common(raw.to_string()))
            }
            Some((target, name)) => {
                if name.is_empty() || name.contains('.') {
                    return Err(ContractError::MalformedKey(raw.to_string()));
                }
                if !TARGETS.contains(&target) {
                    return Err(ContractError::UnknownTarget {
                        target: target.to_string(),
                        key: raw.to_string(),
                    });
                }
                Ok(Self::Target {
                    target: target.to_string(),
                    name: name.to_string(),
                })
            }
        }
    }

    #[note("contract-propkey-applies-to")]
    pub fn applies_to(&self, target: &str) -> bool {
        match self {
            Self::Common(_) => true,
            Self::Target { target: t, .. } => t == target,
        }
    }
}

impl fmt::Display for PropKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Common(n) => write!(f, "{n}"),
            Self::Target { target, name } => write!(f, "{target}.{name}"),
        }
    }
}

#[note("contract-contracterror")]
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ContractError {
    #[error(
        "property '{0}' is not in the common core: either it goes into crawrad_ir::contract::COMMON \
         (which requires every runtime to honor it), or it needs a target prefix, \
         such as 'win32.{0}'"
    )]
    NotCommon(String),

    // THE TARGET LIST IS NOT WRITTEN IN THE PHRASE, and it once was: the text said
    // `(known: win32, egui)` and aged the day `makepad` entered —
    // an error message that lies about what is accepted sends whoever reads it
    // looking in the wrong place. `TARGETS` is the source, and the phrase reads it.
    #[error(
        "unknown target '{target}' in property '{key}' (known: {})",
        known_targets()
    )]
    UnknownTarget { target: String, key: String },

    #[error("malformed property key: '{0}'")]
    MalformedKey(String),
}
