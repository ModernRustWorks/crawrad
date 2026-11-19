use crate::engine::Engine;
use crate::error::Error;
use crate::params::NamedParams;
use crate::value::Value;

/// High-throughput bulk insertion loader executing chunked batch operations inside a transaction.
pub struct BulkLoader<'a> {
    table: &'a str,
    columns: Vec<&'a str>,
    batch_size: usize,
}

impl<'a> BulkLoader<'a> {
    /// Constructs a new bulk loader for the specified table and columns.
    #[must_use]
    pub fn new(table: &'a str, columns: &[&'a str]) -> Self {
        Self {
            table,
            columns: columns.to_vec(),
            batch_size: 500,
        }
    }

    /// Configures the maximum number of rows per SQL batch.
    #[must_use]
    pub fn with_batch_size(mut self, size: usize) -> Self {
        self.batch_size = size.max(1);
        self
    }

    /// Inserts all rows into the database using chunked batch insertions inside an atomic transaction.
    /// Returns the total count of inserted rows.
    pub async fn insert_all(
        &self,
        engine: &Engine,
        rows_data: &[Vec<Value>],
    ) -> Result<u64, Error> {
        if rows_data.is_empty() || self.columns.is_empty() {
            return Ok(0);
        }

        let dialect = engine.dialect();
        let quoted_table = dialect.quote_identifier(self.table);
        let quoted_cols: Vec<String> = self
            .columns
            .iter()
            .map(|c| dialect.quote_identifier(c))
            .collect();
        let cols_header = quoted_cols.join(", ");

        let mut conn = engine.acquire().await?;
        let mut tx = conn.begin().await?;

        let mut total_affected = 0u64;

        for chunk in rows_data.chunks(self.batch_size) {
            let mut sql = format!("INSERT INTO {quoted_table} ({cols_header}) VALUES ");
            let mut params = NamedParams::new();
            let mut value_tuples = Vec::with_capacity(chunk.len());

            for (row_idx, row_values) in chunk.iter().enumerate() {
                let mut placeholders = Vec::with_capacity(self.columns.len());
                for (col_idx, val) in row_values.iter().enumerate() {
                    let param_name = format!("b_{row_idx}_{col_idx}");
                    placeholders.push(format!(":{param_name}"));
                    params.insert(&param_name, val.clone());
                }
                value_tuples.push(format!("({})", placeholders.join(", ")));
            }

            sql.push_str(&value_tuples.join(", "));
            let affected = tx.execute(&sql, &params).await?;
            total_affected += affected;
        }

        tx.commit().await?;
        Ok(total_affected)
    }
}
