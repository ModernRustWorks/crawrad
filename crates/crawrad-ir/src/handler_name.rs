//! Derived event handler naming rules.

use crawrad_note::note;
#[note("handler-name-default-handler-name")]
pub fn default_handler_name(component: &str, event: &str) -> String {
    let suffix = event.strip_prefix("on_").unwrap_or(event);
    format!("{component}_{suffix}")
}

#[note("handler-name-legacy-handler-name")]
pub fn legacy_handler_name(component: &str, event: &str) -> String {
    let suffix: String = event
        .strip_prefix("on_")
        .unwrap_or(event)
        .split('_')
        .map(|p| {
            let mut c = p.chars();
            match c.next() {
                Some(a) => a.to_uppercase().collect::<String>() + c.as_str(),
                None => String::new(),
            }
        })
        .collect();
    format!("{component}{suffix}")
}

#[note("handler-name-is-derived-handler-name")]
pub fn is_derived_handler_name(component: &str, event: &str, name: &str) -> bool {
    default_handler_name(component, event) == name || legacy_handler_name(component, event) == name
}

#[note("handler-name-is-handler-ident")]
pub fn is_handler_ident(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(c) if c.is_ascii_alphabetic() || c == '_' => {
            name != "_" && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
        }
        _ => false,
    }
}

#[cfg(test)]
#[path = "test/handler_name/test.rs"]
mod test;
