//! A dialog's controls as data: each state names its fields, says what each one
//! shows and takes what the user did to it. The shell's one binder reads and
//! paints every dialog that implements [`DialogFields`] — no per-control glue.

/// What kind of control a field is, and so how the shell reads and paints it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FieldKind {
    /// A text box: shows and takes [`FieldValue::Text`].
    Text,
    /// A check box: shows and takes [`FieldValue::On`].
    Check,
    /// A drop-down with these items: shows and takes [`FieldValue::Index`].
    Drop(&'static [&'static str]),
    /// Radio buttons, one per name, top to bottom: shows and takes [`FieldValue::Index`].
    Radio(&'static [&'static str]),
    /// A label: shows [`FieldValue::Text`].
    Label,
    /// A button: may show a caption ([`FieldValue::Text`]); a click arrives as `On(true)`.
    Button,
}

/// A field's value, as the control holds it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldValue {
    Text(String),
    On(bool),
    Index(usize),
}

/// A dialog state whose controls the shell binds by name. A name may be a path
/// (`row.label`) to a control inside a template.
pub trait DialogFields {
    /// The fields, by widget name, in the order the dialog paints them.
    fn fields(&self) -> &'static [(&'static str, FieldKind)];
    /// What a field shows now; `None` leaves the control as it is.
    fn value(&self, field: &str) -> Option<FieldValue>;
    /// What the user did to a field.
    fn input(&mut self, field: &str, value: FieldValue);
}
