//! Predicate filters evaluated column-at-a-time into bitmaps.

use crate::bitmap::Bitmap;
use crate::column::Column;
use crate::store::{Store, StoreError};
use std::collections::HashSet;

/// Owned cell value used as the literal operand in a filter predicate.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Owned {
    Int(i64),
    Float(f64),
    Bool(bool),
    Text(String),
}

#[inline]
fn canonical_float(v: f64) -> f64 {
    if v.is_nan() {
        f64::NAN
    } else if v == 0.0 {
        0.0
    } else {
        v
    }
}

#[inline]
fn canonical_float_bits(v: f64) -> u64 {
    canonical_float(v).to_bits()
}

impl PartialEq for Owned {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Owned::Int(a), Owned::Int(b)) => a == b,
            (Owned::Float(a), Owned::Float(b)) => {
                canonical_float_bits(*a) == canonical_float_bits(*b)
            }
            (Owned::Bool(a), Owned::Bool(b)) => a == b,
            (Owned::Text(a), Owned::Text(b)) => a == b,
            _ => false,
        }
    }
}

impl Eq for Owned {}

impl std::hash::Hash for Owned {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::mem::discriminant(self).hash(state);
        match self {
            Owned::Int(v) => v.hash(state),
            Owned::Float(v) => canonical_float_bits(*v).hash(state),
            Owned::Bool(v) => v.hash(state),
            Owned::Text(v) => v.hash(state),
        }
    }
}

impl Owned {
    /// Compares two owned values in a canonical total order:
    /// Kind order: Int < Float < Bool < Text, then by value.
    /// Floats are compared using `total_cmp` on their canonical bit patterns.
    pub fn canonical_cmp(&self, other: &Self) -> std::cmp::Ordering {
        fn kind(val: &Owned) -> u8 {
            match val {
                Owned::Int(_) => 0,
                Owned::Float(_) => 1,
                Owned::Bool(_) => 2,
                Owned::Text(_) => 3,
            }
        }

        match (self, other) {
            (Owned::Int(a), Owned::Int(b)) => a.cmp(b),
            (Owned::Float(a), Owned::Float(b)) => {
                let ca = canonical_float(*a);
                let cb = canonical_float(*b);
                ca.total_cmp(&cb)
            }
            (Owned::Bool(a), Owned::Bool(b)) => a.cmp(b),
            (Owned::Text(a), Owned::Text(b)) => a.cmp(b),
            _ => kind(self).cmp(&kind(other)),
        }
    }
}

impl From<i64> for Owned {
    fn from(v: i64) -> Self {
        Owned::Int(v)
    }
}

impl From<i32> for Owned {
    fn from(v: i32) -> Self {
        Owned::Int(v as i64)
    }
}

impl From<f64> for Owned {
    fn from(v: f64) -> Self {
        Owned::Float(v)
    }
}

impl From<bool> for Owned {
    fn from(v: bool) -> Self {
        Owned::Bool(v)
    }
}

impl From<&str> for Owned {
    fn from(v: &str) -> Self {
        Owned::Text(v.to_string())
    }
}

impl From<String> for Owned {
    fn from(v: String) -> Self {
        Owned::Text(v)
    }
}

/// Filter predicate tree.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Filter {
    And(Vec<Filter>),
    Or(Vec<Filter>),
    Not(Box<Filter>),
    Eq(String, Owned),
    Lt(String, Owned),
    Le(String, Owned),
    Gt(String, Owned),
    Ge(String, Owned),
    Between(String, Owned, Owned),
    In(String, Vec<Owned>),
    IsNull(String),
    Contains {
        column: String,
        text: String,
        ignore_case: bool,
    },
    StartsWith {
        column: String,
        text: String,
        ignore_case: bool,
    },
    EqIgnoreCase {
        column: String,
        text: String,
    },
}

impl Filter {
    /// Predicate equality: `column == val`.
    pub fn eq(column: impl Into<String>, val: impl Into<Owned>) -> Self {
        Filter::Eq(column.into(), val.into())
    }

    /// Predicate less than: `column < val`.
    pub fn lt(column: impl Into<String>, val: impl Into<Owned>) -> Self {
        Filter::Lt(column.into(), val.into())
    }

    /// Predicate less than or equal: `column <= val`.
    pub fn le(column: impl Into<String>, val: impl Into<Owned>) -> Self {
        Filter::Le(column.into(), val.into())
    }

    /// Predicate greater than: `column > val`.
    pub fn gt(column: impl Into<String>, val: impl Into<Owned>) -> Self {
        Filter::Gt(column.into(), val.into())
    }

    /// Predicate greater than or equal: `column >= val`.
    pub fn ge(column: impl Into<String>, val: impl Into<Owned>) -> Self {
        Filter::Ge(column.into(), val.into())
    }

    /// Predicate between inclusive: `column >= lo && column <= hi`.
    pub fn between(column: impl Into<String>, lo: impl Into<Owned>, hi: impl Into<Owned>) -> Self {
        Filter::Between(column.into(), lo.into(), hi.into())
    }

    /// Predicate membership: `column IN values`.
    pub fn in_list<I, T>(column: impl Into<String>, values: I) -> Self
    where
        I: IntoIterator<Item = T>,
        T: Into<Owned>,
    {
        Filter::In(column.into(), values.into_iter().map(Into::into).collect())
    }

    /// Predicate is null: `column IS NULL`.
    pub fn is_null(column: impl Into<String>) -> Self {
        Filter::IsNull(column.into())
    }

    /// Case-sensitive substring match: `column CONTAINS text`.
    pub fn contains(column: impl Into<String>, text: impl Into<String>) -> Self {
        Filter::Contains {
            column: column.into(),
            text: text.into(),
            ignore_case: false,
        }
    }

    /// Case-sensitive prefix match: `column STARTS WITH text`.
    pub fn starts_with(column: impl Into<String>, text: impl Into<String>) -> Self {
        Filter::StartsWith {
            column: column.into(),
            text: text.into(),
            ignore_case: false,
        }
    }

    /// Case-insensitive substring match.
    pub fn contains_ignore_case(column: impl Into<String>, text: impl Into<String>) -> Self {
        Filter::Contains {
            column: column.into(),
            text: text.into(),
            ignore_case: true,
        }
    }

    /// Case-insensitive prefix match.
    pub fn starts_with_ignore_case(column: impl Into<String>, text: impl Into<String>) -> Self {
        Filter::StartsWith {
            column: column.into(),
            text: text.into(),
            ignore_case: true,
        }
    }

    /// Exact case-insensitive text match.
    pub fn eq_ignore_case(column: impl Into<String>, text: impl Into<String>) -> Self {
        Filter::EqIgnoreCase {
            column: column.into(),
            text: text.into(),
        }
    }

    /// Combines two filters with logical AND. If `self` is an `And`, appends `other`
    /// instead of nesting.
    pub fn and(self, other: Filter) -> Self {
        match self {
            Filter::And(mut children) => {
                children.push(other);
                Filter::And(children)
            }
            _ => Filter::And(vec![self, other]),
        }
    }

    /// Combines two filters with logical OR. If `self` is an `Or`, appends `other`
    /// instead of nesting.
    pub fn or(self, other: Filter) -> Self {
        match self {
            Filter::Or(mut children) => {
                children.push(other);
                Filter::Or(children)
            }
            _ => Filter::Or(vec![self, other]),
        }
    }

    /// Returns a normalized form of this filter tree:
    /// - Flattens nested `And` inside `And` and `Or` inside `Or`.
    /// - An `And` or `Or` with one child becomes that child.
    /// - `Not(Not(x))` becomes `x`.
    /// - `In` lists are sorted canonically and deduplicated.
    /// - Exact duplicate children of an `And` or `Or` are dropped, keeping the first occurrence.
    pub fn normalized(&self) -> Filter {
        match self {
            Filter::And(children) => {
                let mut flat = Vec::with_capacity(children.len());
                for child in children {
                    let norm = child.normalized();
                    match norm {
                        Filter::And(sub_children) => flat.extend(sub_children),
                        other => flat.push(other),
                    }
                }
                let mut unique = Vec::with_capacity(flat.len());
                let mut seen = HashSet::new();
                for child in flat {
                    if seen.insert(child.clone()) {
                        unique.push(child);
                    }
                }
                if unique.len() == 1 {
                    unique.into_iter().next().expect("exactly one child")
                } else {
                    Filter::And(unique)
                }
            }
            Filter::Or(children) => {
                let mut flat = Vec::with_capacity(children.len());
                for child in children {
                    let norm = child.normalized();
                    match norm {
                        Filter::Or(sub_children) => flat.extend(sub_children),
                        other => flat.push(other),
                    }
                }
                let mut unique = Vec::with_capacity(flat.len());
                let mut seen = HashSet::new();
                for child in flat {
                    if seen.insert(child.clone()) {
                        unique.push(child);
                    }
                }
                if unique.len() == 1 {
                    unique.into_iter().next().expect("exactly one child")
                } else {
                    Filter::Or(unique)
                }
            }
            Filter::Not(child) => {
                let norm = child.normalized();
                match norm {
                    Filter::Not(inner) => *inner,
                    other => Filter::Not(Box::new(other)),
                }
            }
            Filter::In(col, list) => {
                let mut sorted_list = list.clone();
                sorted_list.sort_by(Owned::canonical_cmp);
                sorted_list.dedup();
                Filter::In(col.clone(), sorted_list)
            }
            Filter::Eq(col, val) => Filter::Eq(col.clone(), val.clone()),
            Filter::Lt(col, val) => Filter::Lt(col.clone(), val.clone()),
            Filter::Le(col, val) => Filter::Le(col.clone(), val.clone()),
            Filter::Gt(col, val) => Filter::Gt(col.clone(), val.clone()),
            Filter::Ge(col, val) => Filter::Ge(col.clone(), val.clone()),
            Filter::Between(col, lo, hi) => Filter::Between(col.clone(), lo.clone(), hi.clone()),
            Filter::IsNull(col) => Filter::IsNull(col.clone()),
            Filter::Contains {
                column,
                text,
                ignore_case,
            } => Filter::Contains {
                column: column.clone(),
                text: text.clone(),
                ignore_case: *ignore_case,
            },
            Filter::StartsWith {
                column,
                text,
                ignore_case,
            } => Filter::StartsWith {
                column: column.clone(),
                text: text.clone(),
                ignore_case: *ignore_case,
            },
            Filter::EqIgnoreCase { column, text } => Filter::EqIgnoreCase {
                column: column.clone(),
                text: text.clone(),
            },
        }
    }
}

impl Store {
    /// Evaluates a filter tree against the store, returning a bitmap of matching rows.
    pub fn filter(&self, f: &Filter) -> Result<Bitmap, StoreError> {
        let rows = self.rows();

        match f {
            Filter::And(children) => {
                if children.is_empty() {
                    return Ok(Bitmap::new(rows, true));
                }
                let mut iter = children.iter();
                let first = iter.next().expect("children not empty");
                let mut acc = self.filter(first)?;
                for child in iter {
                    let next = self.filter(child)?;
                    acc = acc.and(&next);
                }
                Ok(acc)
            }
            Filter::Or(children) => {
                if children.is_empty() {
                    return Ok(Bitmap::new(rows, false));
                }
                let mut iter = children.iter();
                let first = iter.next().expect("children not empty");
                let mut acc = self.filter(first)?;
                for child in iter {
                    let next = self.filter(child)?;
                    acc = acc.or(&next);
                }
                Ok(acc)
            }
            Filter::Not(child) => {
                let bm = self.filter(child)?;
                Ok(bm.not())
            }
            Filter::IsNull(col_name) => {
                let col = self
                    .column(col_name)
                    .ok_or_else(|| StoreError::ColumnNotFound(col_name.clone()))?;
                match col.validity() {
                    Some(valid) => Ok(valid.not()),
                    None => Ok(Bitmap::new(rows, false)),
                }
            }
            Filter::Eq(col_name, val) => self.eval_cmp(col_name, val, CmpOp::Eq),
            Filter::Lt(col_name, val) => self.eval_cmp(col_name, val, CmpOp::Lt),
            Filter::Le(col_name, val) => self.eval_cmp(col_name, val, CmpOp::Le),
            Filter::Gt(col_name, val) => self.eval_cmp(col_name, val, CmpOp::Gt),
            Filter::Ge(col_name, val) => self.eval_cmp(col_name, val, CmpOp::Ge),
            Filter::Between(col_name, lo, hi) => self.eval_between(col_name, lo, hi),
            Filter::In(col_name, list) => self.eval_in(col_name, list),
            Filter::Contains {
                column,
                text,
                ignore_case,
            } => self.eval_string_match(column, text, *ignore_case, StringMatchKind::Contains),
            Filter::StartsWith {
                column,
                text,
                ignore_case,
            } => self.eval_string_match(column, text, *ignore_case, StringMatchKind::StartsWith),
            Filter::EqIgnoreCase { column, text } => {
                self.eval_string_match(column, text, true, StringMatchKind::Exact)
            }
        }
    }

    fn eval_cmp(&self, col_name: &str, target: &Owned, op: CmpOp) -> Result<Bitmap, StoreError> {
        let col = self
            .column(col_name)
            .ok_or_else(|| StoreError::ColumnNotFound(col_name.to_string()))?;
        let rows = self.rows();

        let raw_bm = match (col, target) {
            (Column::Int { values, .. }, Owned::Int(t)) => {
                build_bitmap(rows, |r| op.holds(values[r], *t))
            }
            (Column::Int { values, .. }, Owned::Float(t)) => {
                build_bitmap(rows, |r| op.holds(values[r] as f64, *t))
            }
            (Column::Float { values, .. }, Owned::Float(t)) => {
                build_bitmap(rows, |r| op.holds(values[r], *t))
            }
            (Column::Bool { values, .. }, Owned::Bool(t)) => build_bitmap(rows, |r| {
                let v = values.get(r);
                match op {
                    CmpOp::Eq => v == *t,
                    CmpOp::Lt => !v && *t,
                    CmpOp::Le => !v || *t,
                    CmpOp::Gt => v && !*t,
                    CmpOp::Ge => v || !*t,
                }
            }),
            (Column::Text { bytes, offsets, .. }, Owned::Text(t)) => build_bitmap(rows, |r| {
                let s = get_text_slice(bytes, offsets, r);
                match op {
                    CmpOp::Eq => s == t.as_str(),
                    CmpOp::Lt => s < t.as_str(),
                    CmpOp::Le => s <= t.as_str(),
                    CmpOp::Gt => s > t.as_str(),
                    CmpOp::Ge => s >= t.as_str(),
                }
            }),
            (Column::Dict { codes, dict, .. }, Owned::Text(t)) => {
                // Evaluated once per dictionary entry
                let dict_matches: Vec<bool> = dict
                    .iter()
                    .map(|entry| match op {
                        CmpOp::Eq => entry == t,
                        CmpOp::Lt => entry < t,
                        CmpOp::Le => entry <= t,
                        CmpOp::Gt => entry > t,
                        CmpOp::Ge => entry >= t,
                    })
                    .collect();
                build_bitmap(rows, |r| {
                    let code = codes[r] as usize;
                    dict_matches[code]
                })
            }
            _ => {
                return Err(StoreError::TypeMismatch {
                    column: col_name.to_string(),
                    expected: expected_type_name(col),
                });
            }
        };

        if let Some(valid) = col.validity() {
            Ok(raw_bm.and(valid))
        } else {
            Ok(raw_bm)
        }
    }

    fn eval_between(&self, col_name: &str, lo: &Owned, hi: &Owned) -> Result<Bitmap, StoreError> {
        let col = self
            .column(col_name)
            .ok_or_else(|| StoreError::ColumnNotFound(col_name.to_string()))?;
        let rows = self.rows();

        let raw_bm = match (col, lo, hi) {
            (Column::Int { values, .. }, Owned::Int(l), Owned::Int(h)) => build_bitmap(rows, |r| {
                let v = values[r];
                v >= *l && v <= *h
            }),
            (Column::Int { values, .. }, Owned::Float(l), Owned::Float(h)) => {
                build_bitmap(rows, |r| {
                    let v = values[r] as f64;
                    v >= *l && v <= *h
                })
            }
            (Column::Float { values, .. }, Owned::Float(l), Owned::Float(h)) => {
                build_bitmap(rows, |r| {
                    let v = values[r];
                    v >= *l && v <= *h
                })
            }
            (Column::Bool { values, .. }, Owned::Bool(l), Owned::Bool(h)) => {
                build_bitmap(rows, |r| {
                    let v = values.get(r);
                    let ge_l = v || !*l;
                    let le_h = !v || *h;
                    ge_l && le_h
                })
            }
            (Column::Text { bytes, offsets, .. }, Owned::Text(l), Owned::Text(h)) => {
                build_bitmap(rows, |r| {
                    let s = get_text_slice(bytes, offsets, r);
                    s >= l.as_str() && s <= h.as_str()
                })
            }
            (Column::Dict { codes, dict, .. }, Owned::Text(l), Owned::Text(h)) => {
                let dict_matches: Vec<bool> =
                    dict.iter().map(|entry| entry >= l && entry <= h).collect();
                build_bitmap(rows, |r| {
                    let code = codes[r] as usize;
                    dict_matches[code]
                })
            }
            _ => {
                return Err(StoreError::TypeMismatch {
                    column: col_name.to_string(),
                    expected: expected_type_name(col),
                });
            }
        };

        if let Some(valid) = col.validity() {
            Ok(raw_bm.and(valid))
        } else {
            Ok(raw_bm)
        }
    }

    fn eval_in(&self, col_name: &str, list: &[Owned]) -> Result<Bitmap, StoreError> {
        let col = self
            .column(col_name)
            .ok_or_else(|| StoreError::ColumnNotFound(col_name.to_string()))?;
        let rows = self.rows();

        let raw_bm = match col {
            Column::Int { values, .. } => {
                let mut set = HashSet::with_capacity(list.len());
                for item in list {
                    match item {
                        Owned::Int(v) => {
                            set.insert(*v);
                        }
                        _ => {
                            return Err(StoreError::TypeMismatch {
                                column: col_name.to_string(),
                                expected: "Int",
                            });
                        }
                    }
                }
                build_bitmap(rows, |r| set.contains(&values[r]))
            }
            Column::Float { values, .. } => {
                let mut float_targets = Vec::with_capacity(list.len());
                for item in list {
                    match item {
                        Owned::Float(v) => float_targets.push(*v),
                        _ => {
                            return Err(StoreError::TypeMismatch {
                                column: col_name.to_string(),
                                expected: "Float",
                            });
                        }
                    }
                }
                build_bitmap(rows, |r| {
                    let v = values[r];
                    float_targets.contains(&v)
                })
            }
            Column::Bool { values, .. } => {
                let mut has_true = false;
                let mut has_false = false;
                for item in list {
                    match item {
                        Owned::Bool(b) => {
                            if *b {
                                has_true = true;
                            } else {
                                has_false = true;
                            }
                        }
                        _ => {
                            return Err(StoreError::TypeMismatch {
                                column: col_name.to_string(),
                                expected: "Bool",
                            });
                        }
                    }
                }
                build_bitmap(rows, |r| {
                    let v = values.get(r);
                    (v && has_true) || (!v && has_false)
                })
            }
            Column::Text { bytes, offsets, .. } => {
                let mut set = HashSet::with_capacity(list.len());
                for item in list {
                    match item {
                        Owned::Text(s) => {
                            set.insert(s.as_str());
                        }
                        _ => {
                            return Err(StoreError::TypeMismatch {
                                column: col_name.to_string(),
                                expected: "Text",
                            });
                        }
                    }
                }
                build_bitmap(rows, |r| {
                    let s = get_text_slice(bytes, offsets, r);
                    set.contains(s)
                })
            }
            Column::Dict { codes, dict, .. } => {
                let mut set = HashSet::with_capacity(list.len());
                for item in list {
                    match item {
                        Owned::Text(s) => {
                            set.insert(s.as_str());
                        }
                        _ => {
                            return Err(StoreError::TypeMismatch {
                                column: col_name.to_string(),
                                expected: "Text",
                            });
                        }
                    }
                }
                // Evaluated once per dictionary entry
                let dict_matches: Vec<bool> = dict
                    .iter()
                    .map(|entry| set.contains(entry.as_str()))
                    .collect();
                build_bitmap(rows, |r| {
                    let code = codes[r] as usize;
                    dict_matches[code]
                })
            }
        };

        if let Some(valid) = col.validity() {
            Ok(raw_bm.and(valid))
        } else {
            Ok(raw_bm)
        }
    }

    fn eval_string_match(
        &self,
        col_name: &str,
        needle: &str,
        ignore_case: bool,
        kind: StringMatchKind,
    ) -> Result<Bitmap, StoreError> {
        let col = self
            .column(col_name)
            .ok_or_else(|| StoreError::ColumnNotFound(col_name.to_string()))?;
        let rows = self.rows();

        let raw_bm = match col {
            Column::Text { bytes, offsets, .. } => {
                let needle_lower = if ignore_case {
                    Some(needle.to_lowercase())
                } else {
                    None
                };
                build_bitmap(rows, |r| {
                    let s = get_text_slice(bytes, offsets, r);
                    match kind {
                        StringMatchKind::Contains => {
                            if let Some(ref nl) = needle_lower {
                                s.to_lowercase().contains(nl)
                            } else {
                                s.contains(needle)
                            }
                        }
                        StringMatchKind::StartsWith => {
                            if let Some(ref nl) = needle_lower {
                                s.to_lowercase().starts_with(nl)
                            } else {
                                s.starts_with(needle)
                            }
                        }
                        StringMatchKind::Exact => {
                            if let Some(ref nl) = needle_lower {
                                s.to_lowercase() == *nl
                            } else {
                                s == needle
                            }
                        }
                    }
                })
            }
            Column::Dict { codes, dict, .. } => {
                // Evaluated once per dictionary entry
                let needle_lower = if ignore_case {
                    Some(needle.to_lowercase())
                } else {
                    None
                };
                let dict_matches: Vec<bool> = dict
                    .iter()
                    .map(|entry| match kind {
                        StringMatchKind::Contains => {
                            if let Some(ref nl) = needle_lower {
                                entry.to_lowercase().contains(nl)
                            } else {
                                entry.contains(needle)
                            }
                        }
                        StringMatchKind::StartsWith => {
                            if let Some(ref nl) = needle_lower {
                                entry.to_lowercase().starts_with(nl)
                            } else {
                                entry.starts_with(needle)
                            }
                        }
                        StringMatchKind::Exact => {
                            if let Some(ref nl) = needle_lower {
                                entry.to_lowercase() == *nl
                            } else {
                                entry == needle
                            }
                        }
                    })
                    .collect();

                build_bitmap(rows, |r| {
                    let code = codes[r] as usize;
                    dict_matches[code]
                })
            }
            _ => {
                return Err(StoreError::TypeMismatch {
                    column: col_name.to_string(),
                    expected: "Text",
                });
            }
        };

        if let Some(valid) = col.validity() {
            Ok(raw_bm.and(valid))
        } else {
            Ok(raw_bm)
        }
    }
}

#[derive(Clone, Copy)]
enum CmpOp {
    Eq,
    Lt,
    Le,
    Gt,
    Ge,
}

impl CmpOp {
    /// `value <op> target` for any ordered value.
    fn holds<T: PartialOrd>(self, value: T, target: T) -> bool {
        match self {
            CmpOp::Eq => value == target,
            CmpOp::Lt => value < target,
            CmpOp::Le => value <= target,
            CmpOp::Gt => value > target,
            CmpOp::Ge => value >= target,
        }
    }
}

#[derive(Clone, Copy)]
enum StringMatchKind {
    Contains,
    StartsWith,
    Exact,
}

fn expected_type_name(col: &Column) -> &'static str {
    match col {
        Column::Int { .. } => "Int",
        Column::Float { .. } => "Float",
        Column::Bool { .. } => "Bool",
        Column::Text { .. } | Column::Dict { .. } => "Text",
    }
}

#[inline]
fn get_text_slice<'a>(bytes: &'a [u8], offsets: &[u32], row: usize) -> &'a str {
    let start = offsets[row] as usize;
    let end = offsets[row + 1] as usize;
    let slice = &bytes[start..end];
    std::str::from_utf8(slice).expect("valid utf-8 in text column")
}

#[inline]
fn build_bitmap<F>(rows: usize, mut predicate: F) -> Bitmap
where
    F: FnMut(usize) -> bool,
{
    if rows == 0 {
        return Bitmap::new(0, false);
    }
    let num_words = rows.div_ceil(64);
    let mut words = vec![0u64; num_words];

    for (w, word_slot) in words.iter_mut().enumerate() {
        let base = w * 64;
        let limit = (rows - base).min(64);
        let mut word = 0u64;
        for bit in 0..limit {
            if predicate(base + bit) {
                word |= 1u64 << bit;
            }
        }
        *word_slot = word;
    }

    Bitmap::from_words(words, rows)
}

#[cfg(test)]
#[path = "test/filter/test.rs"]
mod test;

/// `!filter` negates a predicate with logical NOT.
impl std::ops::Not for Filter {
    type Output = Filter;

    fn not(self) -> Filter {
        Filter::Not(Box::new(self))
    }
}
