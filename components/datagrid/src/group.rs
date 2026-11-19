//! Hash and dense mixed-radix grouping with aggregate totals.

use crate::column::{Column, Value};
use crate::ordinal;
use crate::sort::sort_pairs;
use crate::store::{Store, StoreError};
#[cfg(test)]
use std::collections::HashMap;

/// Aggregation operation to compute per group.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Aggregate {
    Count,
    Sum,
    Min,
    Max,
    Avg,
}

/// Descriptor for an aggregate total over a column.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Total {
    pub column: String,
    pub aggregate: Aggregate,
}

impl Total {
    /// Creates a count aggregation total descriptor.
    pub fn count(column: impl Into<String>) -> Self {
        Self {
            column: column.into(),
            aggregate: Aggregate::Count,
        }
    }

    /// Creates a sum aggregation total descriptor.
    pub fn sum(column: impl Into<String>) -> Self {
        Self {
            column: column.into(),
            aggregate: Aggregate::Sum,
        }
    }

    /// Creates a minimum aggregation total descriptor.
    pub fn min(column: impl Into<String>) -> Self {
        Self {
            column: column.into(),
            aggregate: Aggregate::Min,
        }
    }

    /// Creates a maximum aggregation total descriptor.
    pub fn max(column: impl Into<String>) -> Self {
        Self {
            column: column.into(),
            aggregate: Aggregate::Max,
        }
    }

    /// Creates an average aggregation total descriptor.
    pub fn avg(column: impl Into<String>) -> Self {
        Self {
            column: column.into(),
            aggregate: Aggregate::Avg,
        }
    }
}

/// A group of rows sharing the same key values and aggregated totals.
#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    pub key: Vec<Option<u64>>,
    pub rows: Vec<u32>,
    pub totals: Vec<Option<f64>>,
}

#[derive(Clone, Copy)]
struct TotalAccum {
    count: usize,
    sum: f64,
    non_null_count: usize,
    min: f64,
    max: f64,
}

impl TotalAccum {
    fn new() -> Self {
        Self {
            count: 0,
            sum: 0.0,
            non_null_count: 0,
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
}

#[cfg(test)]
struct GroupBuilder {
    key: Vec<Option<u64>>,
    rows: Vec<u32>,
    totals: Vec<TotalAccum>,
}

/// Grouping algorithm selection for benchmarking and testing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GroupMethod {
    Auto,
    #[cfg(test)]
    Dense,
    #[cfg(test)]
    Sort,
    #[cfg(test)]
    Hash,
}

impl Store {
    /// One level of grouping over `rows` by `columns`.
    /// Groups come out in key order (nulls first).
    ///
    /// When all grouping columns are Dict or Bool with total cardinality <= rows / 4,
    /// executes a direct dense mixed-radix grouping. Otherwise uses sort-based grouping.
    pub fn group(
        &self,
        rows: &[u32],
        columns: &[String],
        totals: &[Total],
    ) -> Result<Vec<Group>, StoreError> {
        self.group_with_method(rows, columns, totals, GroupMethod::Auto)
    }

    /// Internal grouping allowing selection of dense, sort-based, or hash-based path.
    pub(crate) fn group_with_method(
        &self,
        rows: &[u32],
        columns: &[String],
        totals: &[Total],
        method: GroupMethod,
    ) -> Result<Vec<Group>, StoreError> {
        // Validate group columns
        let mut group_cols = Vec::with_capacity(columns.len());
        for col_name in columns {
            let col = self
                .column(col_name)
                .ok_or_else(|| StoreError::ColumnNotFound(col_name.clone()))?;
            if matches!(col, Column::Text { .. }) {
                return Err(StoreError::NotGroupable(col_name.clone()));
            }
            group_cols.push(col);
        }

        // Validate total columns
        let mut total_cols = Vec::with_capacity(totals.len());
        for tot in totals {
            let col = self
                .column(&tot.column)
                .ok_or_else(|| StoreError::ColumnNotFound(tot.column.clone()))?;
            if tot.aggregate != Aggregate::Count
                && !matches!(col, Column::Int { .. } | Column::Float { .. })
            {
                return Err(StoreError::TypeMismatch {
                    column: tot.column.clone(),
                    expected: "Int or Float",
                });
            }
            total_cols.push(col);
        }

        let num_totals = totals.len();

        if columns.is_empty() {
            // Grand-total group over all rows
            let mut accumulators = vec![TotalAccum::new(); num_totals];
            let mut group_rows = Vec::with_capacity(rows.len());

            for &row_id in rows {
                group_rows.push(row_id);
                update_accumulators(&mut accumulators, totals, &total_cols, row_id as usize);
            }

            let finalized_totals = accumulators
                .iter()
                .zip(totals.iter())
                .map(|(accum, tot)| finalize_total(accum, tot.aggregate))
                .collect();

            return Ok(vec![Group {
                key: Vec::new(),
                rows: group_rows,
                totals: finalized_totals,
            }]);
        }

        if rows.is_empty() {
            return Ok(Vec::new());
        }

        match method {
            #[cfg(test)]
            GroupMethod::Dense => {
                let mut cards = Vec::with_capacity(group_cols.len());
                let mut total_card: usize = 1;
                for col in &group_cols {
                    let card = match col {
                        Column::Dict { dict, .. } => dict.len() + 1,
                        Column::Bool { .. } => 3,
                        _ => {
                            return Err(StoreError::NotGroupable(
                                "dense grouping requires Dict or Bool".into(),
                            ));
                        }
                    };
                    cards.push(card);
                    total_card = total_card.checked_mul(card).ok_or_else(|| {
                        StoreError::NotGroupable("cardinality overflow for dense grouping".into())
                    })?;
                }
                Ok(self.group_dense(rows, &group_cols, &cards, total_card, totals, &total_cols))
            }
            #[cfg(test)]
            GroupMethod::Sort => Ok(self.group_sort(rows, &group_cols, totals, &total_cols)),
            #[cfg(test)]
            GroupMethod::Hash => Ok(self.group_hash(rows, &group_cols, totals, &total_cols)),
            GroupMethod::Auto => {
                let mut can_dense = true;
                let mut cards = Vec::with_capacity(group_cols.len());
                let mut total_card: usize = 1;

                for col in &group_cols {
                    let card = match col {
                        Column::Dict { dict, .. } => dict.len() + 1,
                        Column::Bool { .. } => 3,
                        _ => {
                            can_dense = false;
                            break;
                        }
                    };
                    cards.push(card);
                    if let Some(prod) = total_card.checked_mul(card) {
                        total_card = prod;
                    } else {
                        can_dense = false;
                        break;
                    }
                }
                if can_dense && total_card > rows.len() / 4 {
                    can_dense = false;
                }

                if can_dense {
                    Ok(
                        self.group_dense(
                            rows,
                            &group_cols,
                            &cards,
                            total_card,
                            totals,
                            &total_cols,
                        ),
                    )
                } else {
                    Ok(self.group_sort(rows, &group_cols, totals, &total_cols))
                }
            }
        }
    }

    /// Sort-based grouping over selected rows.
    fn group_sort(
        &self,
        rows: &[u32],
        group_cols: &[&Column],
        totals: &[Total],
        total_cols: &[&Column],
    ) -> Vec<Group> {
        let num_totals = totals.len();
        let num_cols = group_cols.len();
        let num_rows = rows.len();

        if num_rows == 0 {
            return Vec::new();
        }

        if num_cols == 1 {
            let col = group_cols[0];
            let (ords, null_mask) = ordinal::encode(col, rows);

            // Nulls come first in input order; the rest go through the same stable
            // adaptive sort as `Store::sort`, so ties keep their input order too.
            let mut null_rows = Vec::new();
            let mut keyed = Vec::with_capacity(num_rows);
            for (pos, &r) in rows.iter().enumerate() {
                if null_mask.as_ref().is_some_and(|m| m[pos]) {
                    null_rows.push(r);
                } else {
                    keyed.push((ords[pos], r));
                }
            }
            let pairs: Vec<(Option<u64>, u32)> = null_rows
                .into_iter()
                .map(|r| (None, r))
                .chain(sort_pairs(keyed).into_iter().map(|(o, r)| (Some(o), r)))
                .collect();

            let mut groups = Vec::new();
            let mut i = 0;
            while i < pairs.len() {
                let cur_ord = pairs[i].0;
                let mut j = i + 1;
                while j < pairs.len() && pairs[j].0 == cur_ord {
                    j += 1;
                }

                let mut group_rows = Vec::with_capacity(j - i);
                let mut accumulators = vec![TotalAccum::new(); num_totals];

                for item in &pairs[i..j] {
                    let r = item.1;
                    group_rows.push(r);
                    update_accumulators(&mut accumulators, totals, total_cols, r as usize);
                }

                let finalized_totals = accumulators
                    .iter()
                    .zip(totals.iter())
                    .map(|(accum, tot)| finalize_total(accum, tot.aggregate))
                    .collect();

                groups.push(Group {
                    key: vec![cur_ord],
                    rows: group_rows,
                    totals: finalized_totals,
                });

                i = j;
            }

            groups
        } else {
            let mut all_key_ords = Vec::with_capacity(num_cols);
            for col in group_cols {
                all_key_ords.push(compute_group_ordinals(col, rows));
            }

            let mut items: Vec<(Vec<Option<u64>>, u32, u32)> = Vec::with_capacity(num_rows);
            for (pos, &r) in rows.iter().enumerate() {
                let mut key = Vec::with_capacity(num_cols);
                for col_ords in &all_key_ords {
                    key.push(col_ords[pos]);
                }
                items.push((key, pos as u32, r));
            }

            items.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

            let mut groups = Vec::new();
            let mut i = 0;
            while i < items.len() {
                let mut j = i + 1;
                while j < items.len() && items[j].0 == items[i].0 {
                    j += 1;
                }

                let cur_key = items[i].0.clone();
                let mut group_rows = Vec::with_capacity(j - i);
                let mut accumulators = vec![TotalAccum::new(); num_totals];

                for item in &items[i..j] {
                    let r = item.2;
                    group_rows.push(r);
                    update_accumulators(&mut accumulators, totals, total_cols, r as usize);
                }

                let finalized_totals = accumulators
                    .iter()
                    .zip(totals.iter())
                    .map(|(accum, tot)| finalize_total(accum, tot.aggregate))
                    .collect();

                groups.push(Group {
                    key: cur_key,
                    rows: group_rows,
                    totals: finalized_totals,
                });

                i = j;
            }

            groups
        }
    }

    /// Fast dense grouping over Dict and Bool columns using a mixed-radix index.
    fn group_dense(
        &self,
        rows: &[u32],
        group_cols: &[&Column],
        cards: &[usize],
        total_card: usize,
        totals: &[Total],
        total_cols: &[&Column],
    ) -> Vec<Group> {
        let num_totals = totals.len();
        let num_cols = group_cols.len();
        let num_rows = rows.len();

        let mut counts = vec![0u32; total_card];
        let mut accumulators = vec![TotalAccum::new(); total_card * num_totals];

        // Precompute slot indices for each column column-at-a-time
        let mut col_slots: Vec<Vec<usize>> = Vec::with_capacity(num_cols);
        for &col in group_cols {
            let mut slots = Vec::with_capacity(num_rows);
            match col {
                Column::Dict { codes, valid, .. } => {
                    if let Some(v_mask) = valid {
                        for &r in rows {
                            let r_idx = r as usize;
                            if !v_mask.get(r_idx) {
                                slots.push(0);
                            } else {
                                slots.push(codes[r_idx] as usize + 1);
                            }
                        }
                    } else {
                        for &r in rows {
                            slots.push(codes[r as usize] as usize + 1);
                        }
                    }
                }
                Column::Bool { values, valid, .. } => {
                    if let Some(v_mask) = valid {
                        for &r in rows {
                            let r_idx = r as usize;
                            if !v_mask.get(r_idx) {
                                slots.push(0);
                            } else if values.get(r_idx) {
                                slots.push(2);
                            } else {
                                slots.push(1);
                            }
                        }
                    } else {
                        for &r in rows {
                            if values.get(r as usize) {
                                slots.push(2);
                            } else {
                                slots.push(1);
                            }
                        }
                    }
                }
                _ => unreachable!("only Dict and Bool allowed in dense grouping"),
            }
            col_slots.push(slots);
        }

        // Compute mixed-radix group index per row
        // column at a time: one tight loop per grouping column
        let mut slots = col_slots.into_iter();
        let mut row_group_indices = slots.next().expect("at least one column");
        for (col, &card) in slots.zip(cards.iter().skip(1)) {
            for (idx, slot) in row_group_indices.iter_mut().zip(col) {
                *idx = *idx * card + slot;
            }
        }

        // Pass 1: count rows and accumulate totals per group index
        for (pos, &row_id) in rows.iter().enumerate() {
            let g_idx = row_group_indices[pos];
            counts[g_idx] += 1;
            let base = g_idx * num_totals;
            update_accumulators(
                &mut accumulators[base..base + num_totals],
                totals,
                total_cols,
                row_id as usize,
            );
        }

        // Identify non-empty slots in mixed-radix key order (nulls first)
        let mut non_empty_slots = Vec::new();
        for (idx, &cnt) in counts.iter().enumerate() {
            if cnt > 0 {
                non_empty_slots.push(idx);
            }
        }

        if non_empty_slots.is_empty() {
            return Vec::new();
        }

        let mut slot_to_group = vec![usize::MAX; total_card];
        let mut result_groups = Vec::with_capacity(non_empty_slots.len());

        for (g_num, &slot) in non_empty_slots.iter().enumerate() {
            slot_to_group[slot] = g_num;

            // Decode mixed-radix key
            let mut rem = slot;
            let mut key = vec![None; num_cols];
            for i in (0..num_cols).rev() {
                let card = cards[i];
                let k = rem % card;
                rem /= card;
                key[i] = if k == 0 { None } else { Some((k - 1) as u64) };
            }

            let base = slot * num_totals;
            let finalized_totals = accumulators[base..base + num_totals]
                .iter()
                .zip(totals.iter())
                .map(|(accum, tot)| finalize_total(accum, tot.aggregate))
                .collect();

            result_groups.push(Group {
                key,
                rows: Vec::with_capacity(counts[slot] as usize),
                totals: finalized_totals,
            });
        }

        // Pass 2: place row IDs into their respective groups (counting sort, preserves input order)
        for (pos, &row_id) in rows.iter().enumerate() {
            let g_idx = row_group_indices[pos];
            let g_num = slot_to_group[g_idx];
            result_groups[g_num].rows.push(row_id);
        }

        result_groups
    }

    /// Hash grouping path, kept as the reference the other paths are tested against.
    #[cfg(test)]
    fn group_hash(
        &self,
        rows: &[u32],
        group_cols: &[&Column],
        totals: &[Total],
        total_cols: &[&Column],
    ) -> Vec<Group> {
        let num_totals = totals.len();
        let mut groups: Vec<GroupBuilder> = Vec::new();

        if group_cols.len() == 1 {
            let col = group_cols[0];
            let key_ords = compute_group_ordinals(col, rows);
            let mut map: HashMap<Option<u64>, usize> = HashMap::new();

            for (pos, &row_id) in rows.iter().enumerate() {
                let ord = key_ords[pos];
                let group_idx = match map.get(&ord) {
                    Some(&idx) => idx,
                    None => {
                        let idx = groups.len();
                        groups.push(GroupBuilder {
                            key: vec![ord],
                            rows: Vec::new(),
                            totals: vec![TotalAccum::new(); num_totals],
                        });
                        map.insert(ord, idx);
                        idx
                    }
                };

                let grp = &mut groups[group_idx];
                grp.rows.push(row_id);
                update_accumulators(&mut grp.totals, totals, total_cols, row_id as usize);
            }
        } else {
            let num_cols = group_cols.len();
            let mut all_key_ords = Vec::with_capacity(num_cols);
            for col in group_cols {
                all_key_ords.push(compute_group_ordinals(col, rows));
            }

            let mut map: HashMap<Vec<Option<u64>>, usize> = HashMap::new();
            let mut key_buf = vec![None; num_cols];

            for (pos, &row_id) in rows.iter().enumerate() {
                for (k, col_ords) in all_key_ords.iter().enumerate() {
                    key_buf[k] = col_ords[pos];
                }

                let group_idx = match map.get(key_buf.as_slice()) {
                    Some(&idx) => idx,
                    None => {
                        let idx = groups.len();
                        groups.push(GroupBuilder {
                            key: key_buf.clone(),
                            rows: Vec::new(),
                            totals: vec![TotalAccum::new(); num_totals],
                        });
                        map.insert(key_buf.clone(), idx);
                        idx
                    }
                };

                let grp = &mut groups[group_idx];
                grp.rows.push(row_id);
                update_accumulators(&mut grp.totals, totals, total_cols, row_id as usize);
            }
        }

        // Groups come out in key order (None / nulls sort first)
        groups.sort_unstable_by(|a, b| a.key.cmp(&b.key));

        groups
            .into_iter()
            .map(|builder| {
                let finalized_totals = builder
                    .totals
                    .iter()
                    .zip(totals.iter())
                    .map(|(accum, tot)| finalize_total(accum, tot.aggregate))
                    .collect();
                Group {
                    key: builder.key,
                    rows: builder.rows,
                    totals: finalized_totals,
                }
            })
            .collect()
    }

    /// The display value of a group key (for headers).
    pub fn key_value(&self, column: &str, ordinal: Option<u64>) -> Result<Value<'_>, StoreError> {
        let ord = match ordinal {
            Some(o) => o,
            None => return Ok(Value::Null),
        };

        let col = self
            .column(column)
            .ok_or_else(|| StoreError::ColumnNotFound(column.to_string()))?;

        match col {
            Column::Bool { .. } => Ok(Value::Bool(ord == 1)),
            Column::Dict { dict, .. } => {
                let code = ord as usize;
                Ok(Value::Text(&dict[code]))
            }
            Column::Int { .. } => Ok(Value::Int(ordinal::int_back(ord))),
            Column::Float { .. } => Ok(Value::Float(ordinal::float_back(ord))),
            Column::Text { .. } => Err(StoreError::NotGroupable(column.to_string())),
        }
    }
}

/// Precomputes group ordinals column-at-a-time using `ordinal::encode`.
fn compute_group_ordinals(col: &Column, rows: &[u32]) -> Vec<Option<u64>> {
    let (ords, nulls) = ordinal::encode(col, rows);
    if let Some(mask) = nulls {
        ords.into_iter()
            .zip(mask)
            .map(|(o, is_null)| if is_null { None } else { Some(o) })
            .collect()
    } else {
        ords.into_iter().map(Some).collect()
    }
}

#[inline]
fn update_accumulators(
    accumulators: &mut [TotalAccum],
    totals: &[Total],
    total_cols: &[&Column],
    row: usize,
) {
    for (acc, (tot, col)) in accumulators
        .iter_mut()
        .zip(totals.iter().zip(total_cols.iter()))
    {
        acc.count += 1;
        if tot.aggregate != Aggregate::Count {
            if col.is_null(row) {
                continue;
            }
            let f = match col {
                Column::Int { values, .. } => values[row] as f64,
                Column::Float { values, .. } => values[row],
                _ => continue,
            };
            acc.non_null_count += 1;
            acc.sum += f;
            if f < acc.min {
                acc.min = f;
            }
            if f > acc.max {
                acc.max = f;
            }
        }
    }
}

#[inline]
fn finalize_total(accum: &TotalAccum, agg: Aggregate) -> Option<f64> {
    match agg {
        Aggregate::Count => Some(accum.count as f64),
        Aggregate::Sum => {
            if accum.non_null_count == 0 {
                None
            } else {
                Some(accum.sum)
            }
        }
        Aggregate::Avg => {
            if accum.non_null_count == 0 {
                None
            } else {
                Some(accum.sum / accum.non_null_count as f64)
            }
        }
        Aggregate::Min => {
            if accum.non_null_count == 0 {
                None
            } else {
                Some(accum.min)
            }
        }
        Aggregate::Max => {
            if accum.non_null_count == 0 {
                None
            } else {
                Some(accum.max)
            }
        }
    }
}

#[cfg(test)]
#[path = "test/group/test.rs"]
mod test;
