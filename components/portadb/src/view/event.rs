/// Events emitted when tabular state or selection changes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TableEvent {
    /// Row selection cursor moved.
    SelectionChanged {
        /// Previous selected row index.
        previous: Option<usize>,
        /// Current selected row index.
        current: Option<usize>,
    },
    /// The entire dataset was reloaded or replaced.
    RowsChanged,
    /// An existing row at `index` was modified.
    RowChanged {
        /// Index of the changed row.
        index: usize,
    },
    /// A new row was inserted at `index`.
    RowInserted {
        /// Index of the newly inserted row.
        index: usize,
    },
    /// A row was removed at `index`.
    RowRemoved {
        /// Index of the removed row.
        index: usize,
    },
}

/// Notification signal structure combining monotonic generation version with event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TableSignal {
    /// Monotonically increasing generation version.
    pub version: u64,
    /// Detailed event kind.
    pub kind: TableEvent,
}

// Backward-compatible type aliases
/// Backward-compatible alias for [`TableSignal`].
pub type ChangeSignal = TableSignal;

/// Backward-compatible alias for [`TableEvent`].
pub type ChangeKind = TableEvent;
