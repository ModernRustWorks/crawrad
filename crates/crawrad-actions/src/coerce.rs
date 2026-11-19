//! Property coercion logic for `component.set_property`.
//!
//! Driven ONLY by the property's declared type and descriptor.

use crawrad_ir::{Color, Edges, Font, Rect, Value};
use crawrad_note::note;
use crawrad_registry::{PropDesc, PropKind, Registry};
use std::collections::{BTreeMap, BTreeSet};

#[note("coerce-coerce-to-property")]
pub fn coerce_to_property(
    value: &Value,
    desc: &PropDesc,
    ir: &crawrad_ir::Document,
    registry: &Registry,
) -> Result<Value, String> {
    // 1. Value::Null passes through unchanged.
    if matches!(value, Value::Null) {
        return Ok(Value::Null);
    }

    // 2. Decode serde tagged Map {"t": ..., "v": ...} back to Value if present.
    let decoded_storage;
    let mut current_val = value;
    if let Value::Map(m) = current_val
        && m.len() == 2
        && m.contains_key("t")
        && m.contains_key("v")
    {
        let json = to_json(current_val);
        if let Ok(decoded) = serde_json::from_value::<Value>(json) {
            decoded_storage = decoded;
            current_val = &decoded_storage;
        }
    }

    if matches!(current_val, Value::Null) {
        return Ok(Value::Null);
    }

    // 3. A value already in the shape the property stores passes as it is,
    // except where the type itself carries a check (enum options, integer
    // range, a reference that must exist, collection items given as strings)
    let checked = matches!(
        desc.kind,
        PropKind::Enum(_)
            | PropKind::Int { .. }
            | PropKind::Reference(_)
            | PropKind::Set(_)
            | PropKind::Collection { .. }
    );
    if !checked && std::mem::discriminant(current_val) == std::mem::discriminant(&desc.default) {
        return Ok(current_val.clone());
    }

    // 4. Convert and validate according to `desc.kind` into `desc.default`'s shape.
    match &desc.kind {
        PropKind::String | PropKind::FieldOf(_) => match current_val {
            Value::Str(s) => Ok(Value::Str(s.clone())),
            Value::Int(i) => Ok(Value::Str(i.to_string())),
            Value::Float(f) => Ok(Value::Str(f.to_string())),
            Value::Bool(b) => Ok(Value::Str(b.to_string())),
            _ => Err(err_shape(desc, "a string", &short_describe(current_val))),
        },
        PropKind::Multiline => match current_val {
            Value::Str(s) => Ok(Value::Str(s.clone())),
            Value::Int(i) => Ok(Value::Str(i.to_string())),
            Value::Float(f) => Ok(Value::Str(f.to_string())),
            Value::Bool(b) => Ok(Value::Str(b.to_string())),
            Value::List(l) => {
                let mut lines = Vec::with_capacity(l.len());
                for item in l {
                    if let Value::Str(s) = item {
                        lines.push(s.as_str());
                    } else {
                        return Err(err_shape(
                            desc,
                            "a string or list of strings",
                            &short_describe(current_val),
                        ));
                    }
                }
                Ok(Value::Str(lines.join("\n")))
            }
            _ => Err(err_shape(
                desc,
                "a string or list of strings",
                &short_describe(current_val),
            )),
        },
        PropKind::Bool => match current_val {
            Value::Bool(b) => Ok(Value::Bool(*b)),
            Value::Str(s) if s.eq_ignore_ascii_case("true") => Ok(Value::Bool(true)),
            Value::Str(s) if s.eq_ignore_ascii_case("false") => Ok(Value::Bool(false)),
            _ => Err(err_shape(desc, "a boolean", &short_describe(current_val))),
        },
        PropKind::Int { min, max } => {
            let parsed_int = match current_val {
                Value::Int(i) => Some(*i),
                Value::Float(f)
                    if f.fract() == 0.0 && *f >= i64::MIN as f64 && *f <= i64::MAX as f64 =>
                {
                    Some(*f as i64)
                }
                Value::Str(s) => s.trim().parse::<i64>().ok(),
                _ => None,
            };
            let shape = format!("an integer in {min}..={max}");
            let Some(val) = parsed_int else {
                return Err(err_shape(desc, &shape, &short_describe(current_val)));
            };
            if val < *min || val > *max {
                return Err(err_shape(desc, &shape, &short_describe(current_val)));
            }
            Ok(Value::Int(val))
        }
        PropKind::Float { min, max, .. } => {
            let parsed_float = match current_val {
                Value::Float(f) => Some(*f),
                Value::Int(i) => Some(*i as f64),
                Value::Str(s) => s.trim().parse::<f64>().ok(),
                _ => None,
            };
            let shape = format!("a number in {min}..={max}");
            let Some(val) = parsed_float else {
                return Err(err_shape(desc, &shape, &short_describe(current_val)));
            };
            if val < *min || val > *max || val.is_nan() {
                return Err(err_shape(desc, &shape, &short_describe(current_val)));
            }
            Ok(Value::Float(val))
        }
        PropKind::Color => match current_val {
            Value::Color(c) => Ok(Value::Color(*c)),
            Value::Str(s) => Color::from_hex(s).map(Value::Color).ok_or_else(|| {
                err_shape(
                    desc,
                    "\"#rrggbb\" or {r,g,b,a}",
                    &short_describe(current_val),
                )
            }),
            Value::Map(_) => {
                let mut obj = match to_json(current_val) {
                    serde_json::Value::Object(map) => map,
                    _ => {
                        return Err(err_shape(
                            desc,
                            "\"#rrggbb\" or {r,g,b,a}",
                            &short_describe(current_val),
                        ));
                    }
                };
                if !obj.contains_key("a") {
                    obj.insert("a".to_string(), serde_json::json!(255));
                }
                serde_json::from_value::<Color>(serde_json::Value::Object(obj))
                    .map(Value::Color)
                    .map_err(|_| {
                        err_shape(
                            desc,
                            "\"#rrggbb\" or {r,g,b,a}",
                            &short_describe(current_val),
                        )
                    })
            }
            _ => Err(err_shape(
                desc,
                "\"#rrggbb\" or {r,g,b,a}",
                &short_describe(current_val),
            )),
        },
        PropKind::Edges => match current_val {
            Value::Edges(e) => Ok(Value::Edges(*e)),
            Value::Int(i) => Ok(Value::Edges(Edges::uniform(*i as f32))),
            Value::Float(f) => Ok(Value::Edges(Edges::uniform(*f as f32))),
            Value::List(l) if l.len() == 4 => {
                let to_f32 = |v: &Value| match v {
                    Value::Int(i) => Some(*i as f32),
                    Value::Float(f) => Some(*f as f32),
                    _ => None,
                };
                if let (Some(left), Some(top), Some(right), Some(bottom)) =
                    (to_f32(&l[0]), to_f32(&l[1]), to_f32(&l[2]), to_f32(&l[3]))
                {
                    Ok(Value::Edges(Edges {
                        left,
                        top,
                        right,
                        bottom,
                    }))
                } else {
                    Err(err_shape(
                        desc,
                        "a number, [left, top, right, bottom], or {left, top, right, bottom}",
                        &short_describe(current_val),
                    ))
                }
            }
            Value::Map(_) => serde_json::from_value::<Edges>(to_json(current_val))
                .map(Value::Edges)
                .map_err(|_| {
                    err_shape(
                        desc,
                        "a number, [left, top, right, bottom], or {left, top, right, bottom}",
                        &short_describe(current_val),
                    )
                }),
            _ => Err(err_shape(
                desc,
                "a number, [left, top, right, bottom], or {left, top, right, bottom}",
                &short_describe(current_val),
            )),
        },
        PropKind::Rect => match current_val {
            Value::Rect(r) => Ok(Value::Rect(*r)),
            Value::List(l) if l.len() == 4 => {
                let to_f32 = |v: &Value| match v {
                    Value::Int(i) => Some(*i as f32),
                    Value::Float(f) => Some(*f as f32),
                    _ => None,
                };
                if let (Some(x), Some(y), Some(w), Some(h)) =
                    (to_f32(&l[0]), to_f32(&l[1]), to_f32(&l[2]), to_f32(&l[3]))
                {
                    Ok(Value::Rect(Rect::new(x, y, w, h)))
                } else {
                    Err(err_shape(
                        desc,
                        "[x, y, w, h] or {x, y, w, h}",
                        &short_describe(current_val),
                    ))
                }
            }
            Value::Map(_) => serde_json::from_value::<Rect>(to_json(current_val))
                .map(Value::Rect)
                .map_err(|_| {
                    err_shape(
                        desc,
                        "[x, y, w, h] or {x, y, w, h}",
                        &short_describe(current_val),
                    )
                }),
            _ => Err(err_shape(
                desc,
                "[x, y, w, h] or {x, y, w, h}",
                &short_describe(current_val),
            )),
        },
        PropKind::Font => match current_val {
            Value::Font(f) => Ok(Value::Font(f.clone())),
            Value::Str(s) => s.parse::<Font>().map(Value::Font).map_err(|_| {
                err_shape(
                    desc,
                    "an object {name, size, bold, italic, underline, strike_out}",
                    &short_describe(current_val),
                )
            }),
            Value::Map(_) => serde_json::from_value::<Font>(to_json(current_val))
                .map(Value::Font)
                .map_err(|_| {
                    err_shape(
                        desc,
                        "an object {name, size, bold, italic, underline, strike_out}",
                        &short_describe(current_val),
                    )
                }),
            _ => Err(err_shape(
                desc,
                "an object {name, size, bold, italic, underline, strike_out}",
                &short_describe(current_val),
            )),
        },
        PropKind::Set(flags) => match current_val {
            Value::Map(m) => {
                for (k, v) in m {
                    if !flags.contains(&k.as_str()) {
                        return Err(err_shape(
                            desc,
                            &format!("a map of flags in [{}]", flags.join(", ")),
                            &format!("unknown flag \"{k}\""),
                        ));
                    }
                    if !matches!(v, Value::Bool(_)) {
                        return Err(err_shape(
                            desc,
                            &format!("a map of boolean flags in [{}]", flags.join(", ")),
                            &short_describe(v),
                        ));
                    }
                }
                Ok(Value::Map(m.clone()))
            }
            Value::List(l) => {
                let mut set_flags = BTreeSet::new();
                for item in l {
                    if let Value::Str(s) = item {
                        if !flags.contains(&s.as_str()) {
                            return Err(err_shape(
                                desc,
                                &format!("a list of flags in [{}]", flags.join(", ")),
                                &format!("unknown flag \"{s}\""),
                            ));
                        }
                        set_flags.insert(s.as_str());
                    } else {
                        return Err(err_shape(
                            desc,
                            &format!("a list of flags in [{}]", flags.join(", ")),
                            &short_describe(item),
                        ));
                    }
                }
                let mut out_map = BTreeMap::new();
                for flag in *flags {
                    out_map.insert(flag.to_string(), Value::Bool(set_flags.contains(flag)));
                }
                Ok(Value::Map(out_map))
            }
            _ => Err(err_shape(
                desc,
                &format!("a list of flags in [{}]", flags.join(", ")),
                &short_describe(current_val),
            )),
        },
        PropKind::Enum(options) => {
            let opts = if !options.is_empty() {
                *options
            } else if !desc.options.is_empty() {
                desc.options
            } else {
                &[]
            };
            let s = match current_val {
                Value::Str(s) | Value::Enum(s) => s.as_str(),
                _ => {
                    return Err(err_shape(
                        desc,
                        &format!("one of [{}]", opts.join(", ")),
                        &short_describe(current_val),
                    ));
                }
            };
            if !opts.is_empty() && !opts.contains(&s) {
                return Err(err_shape(
                    desc,
                    &format!("one of [{}]", opts.join(", ")),
                    &short_describe(current_val),
                ));
            }
            match &desc.default {
                Value::Enum(_) => Ok(Value::Enum(s.to_string())),
                _ => Ok(Value::Str(s.to_string())),
            }
        }
        PropKind::Collection { .. } => match current_val {
            Value::List(l) => {
                let mut items = Vec::with_capacity(l.len());
                for item in l {
                    match coerce_collection_item(item) {
                        Ok(ci) => items.push(ci),
                        Err(()) => {
                            return Err(err_shape(
                                desc,
                                "a list of strings or {\"text\": ...} objects",
                                &short_describe(current_val),
                            ));
                        }
                    }
                }
                Ok(Value::List(items))
            }
            _ => Err(err_shape(
                desc,
                "a list of strings or {\"text\": ...} objects",
                &short_describe(current_val),
            )),
        },
        PropKind::Reference(class) => {
            let class_name = if !class.is_empty() {
                Some(*class)
            } else {
                desc.ref_class
            };
            let shape = match class_name {
                Some(w) if !w.is_empty() => format!("a component name or id of class {w}"),
                _ => "a component name or id".to_string(),
            };
            let s = match current_val {
                Value::Str(s) | Value::Ref(s) => s.as_str(),
                _ => {
                    return Err(err_shape(desc, &shape, &short_describe(current_val)));
                }
            };
            if s.is_empty() || s == crawrad_ir::value::NO_REFERENCE {
                return Ok(Value::Ref(String::new()));
            }
            let node = ir.node(s).or_else(|| ir.node_by_name(s));
            let Some(node) = node else {
                return Err(err_shape(desc, &shape, &short_describe(current_val)));
            };
            if let Some(wanted) = class_name
                && !wanted.is_empty()
            {
                if wanted == "Control" {
                    if node.id == ir.root {
                        return Err(err_shape(
                            desc,
                            &shape,
                            &format!("root form \"{}\"", node.name()),
                        ));
                    }
                } else {
                    let matches_class = registry
                        .get(&node.class)
                        .is_some_and(|d| d.answers_to(wanted));
                    if !matches_class {
                        return Err(err_shape(
                            desc,
                            &shape,
                            &format!("\"{}\" of class {}", node.name(), node.class),
                        ));
                    }
                }
            }
            Ok(Value::Ref(node.id.clone()))
        }
        PropKind::DateTime => match current_val {
            Value::DateTime(dt) => Ok(Value::DateTime(*dt)),
            Value::Str(s) => crawrad_ir::DateTime::parse(s)
                .map(Value::DateTime)
                .map_err(|_| {
                    err_shape(
                        desc,
                        "a date string (YYYY-MM-DD HH:MM:SS)",
                        &short_describe(current_val),
                    )
                }),
            Value::Map(_) => serde_json::from_value::<crawrad_ir::DateTime>(to_json(current_val))
                .map(Value::DateTime)
                .map_err(|_| {
                    err_shape(
                        desc,
                        "an object {year, month, day, hour, minute, second}",
                        &short_describe(current_val),
                    )
                }),
            _ => Err(err_shape(
                desc,
                "a date string or object",
                &short_describe(current_val),
            )),
        },
        PropKind::FilePath => match current_val {
            Value::FilePath(fp) => Ok(Value::FilePath(fp.clone())),
            Value::Str(s) => Ok(Value::FilePath(crawrad_ir::FilePath::new(s.clone()))),
            Value::Map(_) => serde_json::from_value::<crawrad_ir::FilePath>(to_json(current_val))
                .map(Value::FilePath)
                .map_err(|_| {
                    err_shape(
                        desc,
                        "a file path string or object",
                        &short_describe(current_val),
                    )
                }),
            _ => Err(err_shape(
                desc,
                "a file path string or object",
                &short_describe(current_val),
            )),
        },
        PropKind::Cursor => match current_val {
            Value::Cursor(c) => Ok(Value::Cursor(*c)),
            Value::Str(s) => s
                .parse::<crawrad_ir::Cursor>()
                .map(Value::Cursor)
                .map_err(|_| err_shape(desc, "a cursor name string", &short_describe(current_val))),
            _ => Err(err_shape(
                desc,
                "a cursor name string",
                &short_describe(current_val),
            )),
        },
        PropKind::ModalResult => match current_val {
            Value::ModalResult(m) => Ok(Value::ModalResult(*m)),
            Value::Str(s) => s
                .parse::<crawrad_ir::ModalResult>()
                .map(Value::ModalResult)
                .map_err(|_| {
                    err_shape(desc, "a modal result string", &short_describe(current_val))
                }),
            _ => Err(err_shape(
                desc,
                "a modal result string",
                &short_describe(current_val),
            )),
        },
        PropKind::Shortcut => match current_val {
            Value::Shortcut(s) => Ok(Value::Shortcut(*s)),
            Value::Str(s) => s
                .parse::<crawrad_ir::Shortcut>()
                .map(Value::Shortcut)
                .map_err(|_| {
                    err_shape(
                        desc,
                        "a shortcut string like 'Ctrl+S'",
                        &short_describe(current_val),
                    )
                }),
            _ => Err(err_shape(
                desc,
                "a shortcut string like 'Ctrl+S'",
                &short_describe(current_val),
            )),
        },
        PropKind::Picture => match current_val {
            Value::Picture(p) => Ok(Value::Picture(p.clone())),
            Value::Str(s) => s
                .parse::<crawrad_ir::Picture>()
                .map(Value::Picture)
                .map_err(|_| {
                    err_shape(
                        desc,
                        "a picture string (path or base64)",
                        &short_describe(current_val),
                    )
                }),
            _ => Err(err_shape(
                desc,
                "a picture string (path or base64)",
                &short_describe(current_val),
            )),
        },
    }
}

fn coerce_collection_item(item: &Value) -> Result<Value, ()> {
    match item {
        Value::Str(s) => Ok(crawrad_ir::Item::new(s).to_value()),
        Value::Map(m) => {
            let mut map = m.clone();
            if let Some(children) = map.get("children") {
                if let Value::List(child_list) = children {
                    let new_children: Result<Vec<Value>, ()> =
                        child_list.iter().map(coerce_collection_item).collect();
                    map.insert("children".to_string(), Value::List(new_children?));
                } else {
                    return Err(());
                }
            }
            Ok(Value::Map(map))
        }
        _ => Err(()),
    }
}

fn to_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Str(s) => serde_json::Value::String(s.clone()),
        Value::Int(i) => serde_json::Value::Number((*i).into()),
        Value::Float(f) => serde_json::Number::from_f64(*f)
            .map(serde_json::Value::Number)
            .unwrap_or(serde_json::Value::Null),
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::List(l) => serde_json::Value::Array(l.iter().map(to_json).collect()),
        Value::Map(m) => {
            let mut obj = serde_json::Map::with_capacity(m.len());
            for (k, val) in m {
                obj.insert(k.clone(), to_json(val));
            }
            serde_json::Value::Object(obj)
        }
        Value::Null => serde_json::Value::Null,
        other => serde_json::to_value(other).unwrap_or(serde_json::Value::Null),
    }
}

fn short_describe(v: &Value) -> String {
    match v {
        Value::Str(s) => format!("\"{s}\""),
        Value::Int(i) => i.to_string(),
        Value::Float(f) => f.to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Enum(e) => format!("\"{e}\""),
        Value::Ref(r) => format!("reference \"{r}\""),
        Value::List(l) => format!("a list of {} items", l.len()),
        Value::Map(m) => format!("an object with {} keys", m.len()),
        Value::Null => "null".to_string(),
        other => other.describe(),
    }
}

fn err_shape(desc: &PropDesc, shape: &str, got: &str) -> String {
    format!("property '{}' expects {shape}: got {got}", desc.key)
}
