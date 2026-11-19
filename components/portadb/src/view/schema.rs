/// Data types supported for tabular columns in UI views.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DataType {
    /// 64-bit signed integer.
    Integer,
    /// 64-bit floating point number.
    Real,
    /// UTF-8 Text.
    Text,
    /// Binary data blob.
    Blob,
    /// Boolean flag.
    Boolean,
}

/// Metadata describing a single column definition for visual tables and forms.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnDef {
    /// Column name / identifier.
    pub name: String,
    /// Data type of the column.
    pub data_type: DataType,
    /// Whether SQL NULL is permitted.
    pub nullable: bool,
    /// Part of the table's primary key; what `Dataset::post`/`delete` use to find the row.
    pub primary_key: bool,
}

impl ColumnDef {
    /// Constructs a new column definition.
    #[must_use]
    pub fn new(name: impl Into<String>, data_type: DataType, nullable: bool) -> Self {
        Self {
            name: name.into(),
            data_type,
            nullable,
            primary_key: false,
        }
    }

    /// Marks this column as part of the table's primary key.
    #[must_use]
    pub fn primary_key(mut self) -> Self {
        self.primary_key = true;
        self
    }
}

/// Tabular schema descriptor declaring column metadata.
///
/// Can be loaded or defined at visual design-time without an active database connection,
/// allowing IDE visual inspectors, grids, and forms to know columns prior to runtime.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct TableSchema {
    columns: Vec<ColumnDef>,
}

impl TableSchema {
    /// Constructs an empty table schema.
    #[must_use]
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }

    /// Appends a column definition using builder style.
    #[must_use]
    pub fn with_column(mut self, column: ColumnDef) -> Self {
        self.columns.push(column);
        self
    }

    /// Appends a column definition (alias for backward compatibility).
    #[must_use]
    pub fn with_field(self, column: ColumnDef) -> Self {
        self.with_column(column)
    }

    /// Returns a slice of all declared columns.
    #[must_use]
    pub fn columns(&self) -> &[ColumnDef] {
        &self.columns
    }

    /// Returns a slice of all declared columns (alias for backward compatibility).
    #[must_use]
    pub fn fields(&self) -> &[ColumnDef] {
        &self.columns
    }

    /// Returns the primary key column definitions in column declaration order.
    #[must_use]
    pub fn primary_key(&self) -> Vec<&ColumnDef> {
        self.columns.iter().filter(|c| c.primary_key).collect()
    }

    /// Finds a column definition by case-insensitive name.
    #[must_use]
    pub fn find_column(&self, name: &str) -> Option<&ColumnDef> {
        self.columns
            .iter()
            .find(|c| c.name.eq_ignore_ascii_case(name))
    }

    /// Returns true if the schema contains the given column name.
    #[must_use]
    pub fn has_column(&self, name: &str) -> bool {
        self.find_column(name).is_some()
    }

    /// Finds a column definition by case-insensitive name (alias for backward compatibility).
    #[must_use]
    pub fn find_field(&self, name: &str) -> Option<&ColumnDef> {
        self.find_column(name)
    }

    /// Returns true if the schema contains the given column name (alias for backward compatibility).
    #[must_use]
    pub fn has_field(&self, name: &str) -> bool {
        self.has_column(name)
    }
}

// Backward-compatible type aliases
/// Backward-compatible alias for [`ColumnDef`].
pub type FieldDef = ColumnDef;

/// Backward-compatible alias for [`DataType`].
pub type FieldType = DataType;

/// Backward-compatible alias for [`TableSchema`].
pub type SchemaDescriptor = TableSchema;
