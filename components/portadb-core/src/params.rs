use crate::value::Value;

/// A collection of named SQL parameters.
///
/// Prevents string interpolation and SQL injection by binding values safely
/// to parameters identified by name (e.g., `:id`, `:username`).
#[derive(Debug, Default, Clone, PartialEq)]
pub struct NamedParams {
    items: Vec<(String, Value)>,
}

impl NamedParams {
    /// Creates an empty collection of named parameters.
    #[must_use]
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Creates a collection with pre-allocated capacity.
    #[must_use]
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            items: Vec::with_capacity(capacity),
        }
    }

    /// Adds or replaces a named parameter.
    pub fn insert(&mut self, name: impl Into<String>, value: impl Into<Value>) {
        let name_str = name.into();
        let stripped = name_str.strip_prefix(':').unwrap_or(&name_str).to_string();
        if let Some(existing) = self.items.iter_mut().find(|(k, _)| k == &stripped) {
            existing.1 = value.into();
        } else {
            self.items.push((stripped, value.into()));
        }
    }

    /// Retrieves a parameter by name (with or without leading `:`).
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&Value> {
        let stripped = name.strip_prefix(':').unwrap_or(name);
        self.items
            .iter()
            .find(|(k, _)| k == stripped)
            .map(|(_, v)| v)
    }

    /// Returns the number of parameters.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Returns true if there are no parameters.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Iterates over all `(name, &Value)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.items.iter().map(|(k, v)| (k.as_str(), v))
    }
}

/// Convenience macro for constructing [`NamedParams`].
///
/// # Examples
///
/// ```rust
/// use portadb_core::params;
///
/// let empty = params![];
/// let p = params![
///     ("id", 42i64),
///     ("name", "Alice"),
///     ("active", true),
/// ];
/// ```
#[macro_export]
macro_rules! params {
    () => {
        $crate::NamedParams::new()
    };
    ($( ($name:expr, $value:expr) ),* $(,)?) => {{
        let mut p = $crate::NamedParams::new();
        $(
            p.insert($name, $value);
        )*
        p
    }};
}

#[cfg(test)]
#[path = "../tests/params/mod.rs"]
mod test;
