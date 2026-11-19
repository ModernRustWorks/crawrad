use crate::dialect::Dialect;
use crate::params::NamedParams;
use crate::value::Value;

/// AST node representing an equality filter in an SQL WHERE clause.
#[derive(Debug, Clone)]
pub struct WhereClause {
    column: String,
    param_name: String,
    value: Value,
}

/// Dynamic, type-safe SQL SELECT query builder.
#[derive(Debug, Clone)]
pub struct SelectBuilder {
    columns: Vec<String>,
    table: Option<String>,
    wheres: Vec<WhereClause>,
    order_by: Option<(String, bool)>,
    limit: Option<usize>,
    offset: Option<usize>,
}

impl SelectBuilder {
    /// Constructs a query selecting the given columns.
    #[must_use]
    pub fn new(columns: &[&str]) -> Self {
        Self {
            columns: columns.iter().map(|s| (*s).to_string()).collect(),
            table: None,
            wheres: Vec::new(),
            order_by: None,
            limit: None,
            offset: None,
        }
    }

    /// Sets the target table to select from.
    #[must_use]
    pub fn from(mut self, table: impl Into<String>) -> Self {
        self.table = Some(table.into());
        self
    }

    /// Appends an equality condition: `column = :param`.
    #[must_use]
    pub fn where_eq(mut self, column: impl Into<String>, value: impl Into<Value>) -> Self {
        let col = column.into();
        let param_name = format!("p_{}", self.wheres.len() + 1);
        self.wheres.push(WhereClause {
            column: col,
            param_name,
            value: value.into(),
        });
        self
    }

    /// Sets an ORDER BY clause.
    #[must_use]
    pub fn order_by(mut self, column: impl Into<String>, ascending: bool) -> Self {
        self.order_by = Some((column.into(), ascending));
        self
    }

    /// Sets an ORDER BY ASCENDING clause.
    #[must_use]
    pub fn order_by_asc(self, column: impl Into<String>) -> Self {
        self.order_by(column, true)
    }

    /// Sets an ORDER BY DESCENDING clause.
    #[must_use]
    pub fn order_by_desc(self, column: impl Into<String>) -> Self {
        self.order_by(column, false)
    }

    /// Limits the maximum number of rows returned.
    #[must_use]
    pub fn limit(mut self, limit: usize) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Sets the row offset.
    #[must_use]
    pub fn offset(mut self, offset: usize) -> Self {
        self.offset = Some(offset);
        self
    }

    /// Builds the SQL string and parameter map using the target [`Dialect`].
    #[must_use]
    pub fn build(&self, dialect: &(dyn Dialect + Send + Sync)) -> (String, NamedParams) {
        let mut sql = String::new();
        sql.push_str("SELECT ");

        if self.columns.is_empty() {
            sql.push('*');
        } else {
            let cols: Vec<String> = self
                .columns
                .iter()
                .map(|c| dialect.quote_identifier(c))
                .collect();
            sql.push_str(&cols.join(", "));
        }

        if let Some(ref tbl) = self.table {
            sql.push_str(" FROM ");
            sql.push_str(&dialect.quote_identifier(tbl));
        }

        let mut params = NamedParams::new();
        if !self.wheres.is_empty() {
            sql.push_str(" WHERE ");
            let clauses: Vec<String> = self
                .wheres
                .iter()
                .map(|w| {
                    params.insert(&w.param_name, w.value.clone());
                    format!(
                        "{} = :{}",
                        dialect.quote_identifier(&w.column),
                        w.param_name
                    )
                })
                .collect();
            sql.push_str(&clauses.join(" AND "));
        }

        if let Some((ref col, asc)) = self.order_by {
            let quoted = dialect.quote_identifier(col);
            let dir = if asc { "ASC" } else { "DESC" };
            sql.push_str(&format!(" ORDER BY {quoted} {dir}"));
        }

        if let Some(lim) = self.limit {
            sql.push_str(&format!(" LIMIT {lim}"));
        }

        if let Some(off) = self.offset {
            sql.push_str(&format!(" OFFSET {off}"));
        }

        (sql, params)
    }
}

/// Fluent query factory inspired by `Connection.FluentSQL`.
pub struct Query;

impl Query {
    /// Begins a fluent `SELECT` query selecting the given columns.
    #[must_use]
    pub fn select(columns: &[&str]) -> SelectBuilder {
        SelectBuilder::new(columns)
    }
}
