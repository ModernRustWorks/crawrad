//! **Where a control stands in the Tab sequence**, for both targets.
//!
//! Classic RAD walks the focus container by container: a form's controls in
//! `TabOrder`, and inside a panel, the panel's controls in theirs. So a
//! control's place is the `TabOrder` of each container down to it, then its
//! own; sorting the places (ties kept in document order) gives the sequence.

use crawrad_ir::{Document, Node, Value};
use crawrad_note::note;

#[note("targets-tab-places-takes-focus")]
const TAKES_FOCUS: &str = "on_enter";

#[note("targets-tab-places-own")]
fn own(n: &Node) -> Option<i64> {
    match n.props.get("tab_order")? {
        Value::Int(i) => Some(*i),
        _ => Some(0),
    }
}

#[note("targets-tab-places-place")]
pub fn place(reg: &crawrad_registry::Registry, doc: &Document, n: &Node) -> Option<Vec<i64>> {
    let desc = reg.get(&n.class)?;
    let has_tab_stop = desc.prop("tab_stop").is_some();
    let takes_focus = desc.events.iter().any(|e| e.name == TAKES_FOCUS);
    let stops = !matches!(n.props.get("tab_stop"), Some(Value::Bool(false)));
    let mine = own(n).filter(|_| has_tab_stop && takes_focus && stops)?;
    let mut place = vec![mine];
    let mut at = doc.parent_of(&n.id);
    while let Some(p) = at.filter(|p| **p != doc.root) {
        if let Some(order) = doc.node(p).and_then(own) {
            place.push(order);
        }
        at = doc.parent_of(p);
    }
    place.reverse();
    Some(place)
}

#[note("targets-tab-places-literal")]
pub fn literal(place: &[i64]) -> String {
    let list: Vec<String> = place.iter().map(i64::to_string).collect();
    format!("&[{}]", list.join(", "))
}
