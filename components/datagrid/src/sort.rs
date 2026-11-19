//! Columnar multi-key sorting over row identifiers.

use crate::column::Column;
use crate::ordinal;
use crate::store::{Store, StoreError};

/// Below this threshold, `slice::sort_by_key` beats 8-pass radix (benches/techniques.rs: sort_one_int).
pub const SMALL_SORT: usize = 200_000;

/// Maximum rows for packed multi-key sorting before LSD radix wins (benches/techniques.rs: sort_three_keys).
pub const PACKED_MAX: usize = 2_000_000;

/// Sort ordering direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Order {
    Asc,
    Desc,
}

/// Placement of null entries during sorting.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Nulls {
    First,
    Last,
}

/// Sort key descriptor specifying column, direction, and null placement.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SortKey {
    pub column: String,
    pub order: Order,
    pub nulls: Nulls,
}

impl Store {
    /// Sorts a selection of row IDs by the given keys and returns a new permutation.
    ///
    /// Multi-key sort executes an LSD (Least Significant Digit) radix sort by keys,
    /// or packs keys into `u128` when eligible for single-sort acceleration.
    /// Each pass is a stable sort on that key's ordinal with null splitting.
    /// For plain Text keys, the radix pass sorts by the 8-byte prefix, followed by
    /// fixing runs of equal prefixes with a stable full-byte comparison sort.
    pub fn sort(&self, rows: &[u32], keys: &[SortKey]) -> Result<Vec<u32>, StoreError> {
        // Validate all column names first
        for key in keys {
            if self.column(&key.column).is_none() {
                return Err(StoreError::ColumnNotFound(key.column.clone()));
            }
        }

        if rows.len() <= 1 || keys.is_empty() {
            return Ok(rows.to_vec());
        }

        if keys.len() >= 2
            && rows.len() <= PACKED_MAX
            && let Some(sorted) = self.sort_packed(rows, keys)?
        {
            return Ok(sorted);
        }

        self.sort_lsd(rows, keys)
    }

    /// Sorts rows using the packed multi-key path if eligible, returning `Some(sorted_rows)`,
    /// or `None` if the input cannot be packed.
    pub(crate) fn sort_packed(
        &self,
        rows: &[u32],
        keys: &[SortKey],
    ) -> Result<Option<Vec<u32>>, StoreError> {
        if keys.len() < 2 || rows.len() > PACKED_MAX {
            return Ok(None);
        }

        let mut all_ords = Vec::with_capacity(keys.len());
        let mut all_null_masks = Vec::with_capacity(keys.len());
        let mut all_val_widths = Vec::with_capacity(keys.len());
        let mut all_min_ords = Vec::with_capacity(keys.len());
        let mut all_has_nulls = Vec::with_capacity(keys.len());
        let mut key_widths = Vec::with_capacity(keys.len());

        for key in keys {
            let col_idx = self
                .column_index(&key.column)
                .ok_or_else(|| StoreError::ColumnNotFound(key.column.clone()))?;
            let col = &self.columns[col_idx];

            if matches!(col, Column::Text { .. }) {
                return Ok(None);
            }

            let (ords, null_mask) = ordinal::encode(col, rows);

            let has_nulls = null_mask
                .as_ref()
                .is_some_and(|m| m.iter().any(|&is_null| is_null));

            // Ordinals are rebased on the smallest present one, so an Int column of
            // small values packs into a few bits instead of the full 64.
            let mut min_ord = u64::MAX;
            let mut max_ord = 0u64;
            for (pos, &ord) in ords.iter().enumerate() {
                if null_mask.as_ref().is_some_and(|m| m[pos]) {
                    continue;
                }
                min_ord = min_ord.min(ord);
                max_ord = max_ord.max(ord);
            }
            if min_ord > max_ord {
                min_ord = 0;
                max_ord = 0;
            }
            let span = max_ord - min_ord;

            let val_width = (64 - span.leading_zeros()).max(1) as usize;
            let key_width = val_width + if has_nulls { 1 } else { 0 };

            all_ords.push(ords);
            all_null_masks.push(null_mask);
            all_val_widths.push(val_width);
            all_min_ords.push(min_ord);
            all_has_nulls.push(has_nulls);
            key_widths.push(key_width);
        }

        let total_key_bits: usize = key_widths.iter().sum();
        if total_key_bits > 96 {
            return Ok(None);
        }

        let mut shifts = vec![0usize; keys.len()];
        let mut cur_shift = 32usize;
        for j in (0..keys.len()).rev() {
            shifts[j] = cur_shift;
            cur_shift += key_widths[j];
        }

        let mut pairs = Vec::with_capacity(rows.len());
        for (pos, &r) in rows.iter().enumerate() {
            let mut packed = pos as u128;
            for (j, key) in keys.iter().enumerate() {
                let is_null = all_null_masks[j].as_ref().is_some_and(|m| m[pos]);
                let val_width = all_val_widths[j];
                let has_nulls = all_has_nulls[j];

                let key_bits = if is_null {
                    match key.nulls {
                        Nulls::First => 0u128,
                        Nulls::Last => 1u128 << val_width,
                    }
                } else {
                    let mut ord = all_ords[j][pos] - all_min_ords[j];
                    if key.order == Order::Desc {
                        let mask = if val_width == 64 {
                            u64::MAX
                        } else {
                            (1u64 << val_width) - 1
                        };
                        ord = (!ord) & mask;
                    }
                    let val_part = ord as u128;
                    if has_nulls {
                        match key.nulls {
                            Nulls::First => (1u128 << val_width) | val_part,
                            Nulls::Last => val_part,
                        }
                    } else {
                        val_part
                    }
                };
                packed |= key_bits << shifts[j];
            }
            pairs.push((packed, r));
        }

        pairs.sort_unstable_by_key(|a| a.0);
        let sorted = pairs.into_iter().map(|(_, r)| r).collect();
        Ok(Some(sorted))
    }

    /// Sorts a selection of row IDs using the LSD radix sort path.
    pub(crate) fn sort_lsd(&self, rows: &[u32], keys: &[SortKey]) -> Result<Vec<u32>, StoreError> {
        let mut current_rows = rows.to_vec();

        // Sort by LAST key first, FIRST key last (LSD multi-key sort)
        for key in keys.iter().rev() {
            let col_idx = self
                .column_index(&key.column)
                .expect("column existence was verified");
            let col = &self.columns[col_idx];
            let is_plain_text = matches!(col, Column::Text { .. });

            let (ords, null_mask) = ordinal::encode(col, &current_rows);

            let mut ords = ords;
            if key.order == Order::Desc {
                for o in &mut ords {
                    *o = !*o;
                }
            }

            let mut null_rows = Vec::new();
            let mut non_null_pairs = Vec::with_capacity(current_rows.len());

            if let Some(mask) = null_mask {
                for (pos, &r) in current_rows.iter().enumerate() {
                    if mask[pos] {
                        null_rows.push(r);
                    } else {
                        non_null_pairs.push((ords[pos], r));
                    }
                }
            } else {
                for (pos, &r) in current_rows.iter().enumerate() {
                    non_null_pairs.push((ords[pos], r));
                }
            }

            let mut sorted_non_null_pairs = sort_pairs(non_null_pairs);

            // Plain text keys: radix pass sorted by 8-byte prefix;
            // now fix runs of equal prefix with stable comparison sort on full bytes
            if is_plain_text && sorted_non_null_pairs.len() > 1 {
                let mut i = 0;
                while i < sorted_non_null_pairs.len() {
                    let mut j = i + 1;
                    let prefix = sorted_non_null_pairs[i].0;
                    while j < sorted_non_null_pairs.len() && sorted_non_null_pairs[j].0 == prefix {
                        j += 1;
                    }
                    if j - i > 1 {
                        sorted_non_null_pairs[i..j].sort_by(|&(_, ra), &(_, rb)| {
                            let sa = get_text(col, ra as usize);
                            let sb = get_text(col, rb as usize);
                            let c = sa.as_bytes().cmp(sb.as_bytes());
                            match key.order {
                                Order::Asc => c,
                                Order::Desc => c.reverse(),
                            }
                        });
                    }
                    i = j;
                }
            }

            let mut sorted_non_null: Vec<u32> =
                sorted_non_null_pairs.into_iter().map(|(_, r)| r).collect();

            match key.nulls {
                Nulls::First => {
                    null_rows.append(&mut sorted_non_null);
                    current_rows = null_rows;
                }
                Nulls::Last => {
                    sorted_non_null.append(&mut null_rows);
                    current_rows = sorted_non_null;
                }
            }
        }

        Ok(current_rows)
    }
}

#[inline]
fn get_text(col: &Column, row: usize) -> &str {
    match col {
        Column::Text { bytes, offsets, .. } => {
            let start = offsets[row] as usize;
            let end = offsets[row + 1] as usize;
            std::str::from_utf8(&bytes[start..end]).expect("valid utf-8 in text column")
        }
        _ => unreachable!("get_text only called on plain Text column"),
    }
}

/// Adaptively sorts `(ordinal, row)` pairs stably by ordinal:
/// 1. O(n) scan: if already non-decreasing, returns unchanged.
/// 2. If non-increasing, stably reverses runs of equal keys.
/// 3. If size <= SMALL_SORT, uses stable `slice::sort_by_key`.
/// 4. Otherwise, falls back to 8-pass LSD radix sort.
pub(crate) fn sort_pairs(mut pairs: Vec<(u64, u32)>) -> Vec<(u64, u32)> {
    let n = pairs.len();
    if n <= 1 {
        return pairs;
    }

    let mut is_non_decreasing = true;
    let mut is_non_increasing = true;
    for window in pairs.windows(2) {
        if window[1].0 < window[0].0 {
            is_non_decreasing = false;
        }
        if window[1].0 > window[0].0 {
            is_non_increasing = false;
        }
        if !is_non_decreasing && !is_non_increasing {
            break;
        }
    }

    if is_non_decreasing {
        return pairs;
    }

    if is_non_increasing {
        let mut start = 0;
        while start < n {
            let mut end = start + 1;
            let ord = pairs[start].0;
            while end < n && pairs[end].0 == ord {
                end += 1;
            }
            pairs[start..end].reverse();
            start = end;
        }
        pairs.reverse();
        return pairs;
    }

    if n <= SMALL_SORT {
        pairs.sort_by_key(|&(ord, _)| ord);
        return pairs;
    }

    lsd_radix_sort(pairs)
}

/// Stable 8-pass LSD radix sort over `(ordinal, row)` pairs, skipping passes
/// where all keys share the same byte. Returns the sorted `(ordinal, row)` pairs.
fn lsd_radix_sort(mut pairs: Vec<(u64, u32)>) -> Vec<(u64, u32)> {
    let n = pairs.len();
    if n <= 1 {
        return pairs;
    }

    let mut temp = vec![(0u64, 0u32); n];

    for pass in 0..8 {
        let shift = pass * 8;
        let mut counts = [0usize; 256];
        for &(ord, _) in &pairs {
            let byte = ((ord >> shift) & 0xFF) as usize;
            counts[byte] += 1;
        }

        // Skip pass if this byte is identical across all rows
        let mut is_constant = false;
        for &c in &counts {
            if c == n {
                is_constant = true;
                break;
            }
        }
        if is_constant {
            continue;
        }

        let mut offsets = [0usize; 256];
        let mut sum = 0;
        for i in 0..256 {
            offsets[i] = sum;
            sum += counts[i];
        }

        for &p in &pairs {
            let byte = ((p.0 >> shift) & 0xFF) as usize;
            temp[offsets[byte]] = p;
            offsets[byte] += 1;
        }

        std::mem::swap(&mut pairs, &mut temp);
    }

    pairs
}

#[cfg(test)]
#[path = "test/sort/test.rs"]
mod test;
