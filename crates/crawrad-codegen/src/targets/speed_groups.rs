//! **The SpeedButtons a group can hold**, the same for both targets.
//!
//! A classic SpeedButton with a `GroupIndex` above 0 is one of a group: the
//! SpeedButtons of the same parent with the same index. Pressing one puts it
//! down and the others up; pressing the one that is down lifts it only with
//! `AllowAllUp`. The index can change at run time, so the generated code
//! compares indexes as it runs; what is known at generation is only who the
//! candidates are — the other SpeedButtons of the same parent.

use crawrad_ir::{Document, Node};
use crawrad_note::note;

#[note("targets-speed-groups-siblings")]
pub fn siblings(doc: &Document, node: &Node) -> Vec<String> {
    let parent = doc.parent_of(&node.id);
    doc.nodes
        .values()
        .filter(|n| n.class == "SpeedButton" && n.id != node.id && doc.parent_of(&n.id) == parent)
        .filter_map(|n| crate::identificador(n.name()).ok())
        .collect()
}

#[note("targets-speed-groups-press")]
pub fn press(doc: &Document, node: &Node, me: &str, prefix: &str) -> String {
    let mut others = String::new();
    for s in siblings(doc, node) {
        others.push_str(&format!(
            "                if {prefix}{s}.group_index == {prefix}{me}.group_index {{\n\
             \x20                   {prefix}{s}.down = false;\n\
             \x20               }}\n"
        ));
    }
    format!(
        "            if {prefix}{me}.down {{\n\
         \x20               if {prefix}{me}.allow_all_up {{\n\
         \x20                   {prefix}{me}.down = false;\n\
         \x20               }}\n\
         \x20           }} else {{\n\
         \x20               {prefix}{me}.down = true;\n\
         {others}\
         \x20           }}\n"
    )
}

#[note("targets-speed-groups-click-block")]
pub fn click_block(
    doc: &Document,
    node: &Node,
    me: &str,
    prefix: &str,
    clicked: &str,
    calls: &str,
) -> String {
    let body = press(doc, node, me, prefix);
    if calls.is_empty() {
        format!("        if {clicked} && {prefix}{me}.group_index > 0 {{\n{body}        }}\n")
    } else {
        let nested = body.replace("\n            ", "\n                ");
        format!(
            "        if {clicked} {{\n\
             \x20           if {prefix}{me}.group_index > 0 {{\n\
             \x20   {nested}\
             \x20           }}\n\
             {calls}\
             \x20       }}\n"
        )
    }
}
