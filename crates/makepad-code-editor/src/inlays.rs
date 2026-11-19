use crate::widgets::{BlockWidget, InlineWidget};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InlineInlay {
    /// Text after a line's end (the debugger's values).
    Text(String),
    /// A hint inside a line (a type rust-analyzer infers, a parameter name).
    Hint(String),
    Widget(InlineWidget),
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockInlay {
    Widget(BlockWidget),
    Lens(String),
}
