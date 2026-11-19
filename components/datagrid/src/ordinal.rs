//! Value ordinal encoding and decoding for sorting and grouping.

use crate::bitmap::Bitmap;
use crate::column::Column;

/// Encodes a 64-bit integer into a total-order ordinal `u64`.
#[inline]
pub(crate) fn int(v: i64) -> u64 {
    (v as u64) ^ (1 << 63)
}

/// Decodes an ordinal `u64` back into an `i64`.
#[inline]
pub(crate) fn int_back(o: u64) -> i64 {
    (o ^ (1 << 63)) as i64
}

/// Encodes an `f64` into a total-order ordinal `u64`.
///
/// Canonicalizes -0.0 to 0.0 so they share identical ordinals.
/// Any NaN maps to `u64::MAX` (ordered after +inf).
#[inline]
pub(crate) fn float(v: f64) -> u64 {
    let mut val = v;
    if val == 0.0 {
        val = 0.0;
    }
    if val.is_nan() {
        u64::MAX
    } else {
        let b = val.to_bits();
        if (b >> 63) == 1 { !b } else { b | (1 << 63) }
    }
}

/// Decodes an ordinal `u64` back into an `f64`.
#[inline]
pub(crate) fn float_back(o: u64) -> f64 {
    if o == u64::MAX {
        f64::NAN
    } else if (o >> 63) == 1 {
        let b = o & !(1 << 63);
        f64::from_bits(b)
    } else {
        let b = !o;
        f64::from_bits(b)
    }
}

/// Encodes a boolean into an ordinal `u64` (0 for false, 1 for true).
#[inline]
pub(crate) fn bool(v: bool) -> u64 {
    if v { 1 } else { 0 }
}

/// Extracts an 8-byte big-endian prefix ordinal from text bytes.
#[inline]
pub(crate) fn text_prefix(bytes: &[u8]) -> u64 {
    let mut buf = [0u8; 8];
    let take = bytes.len().min(8);
    buf[..take].copy_from_slice(&bytes[..take]);
    u64::from_be_bytes(buf)
}

/// Helper executing tight loops over `rows`, populating `out` ordinals and optional `nulls`.
#[inline]
fn fill<F: Fn(usize) -> u64>(
    rows: &[u32],
    valid: Option<&Bitmap>,
    f: F,
    out: &mut Vec<u64>,
    nulls: &mut Option<Vec<bool>>,
) {
    if let Some(mask) = valid {
        let mut null_vec = Vec::with_capacity(rows.len());
        for &r in rows {
            let idx = r as usize;
            null_vec.push(!mask.get(idx));
            out.push(f(idx));
        }
        *nulls = Some(null_vec);
    } else {
        for &r in rows {
            out.push(f(r as usize));
        }
        *nulls = None;
    }
}

/// Encodes a column over the selection `rows` column-at-a-time.
///
/// Matches the column variant once and fills ordinals in ascending order (no `Order` applied).
/// Returns a validity mask only if the underlying column contains nulls.
pub(crate) fn encode(col: &Column, rows: &[u32]) -> (Vec<u64>, Option<Vec<bool>>) {
    let mut ords = Vec::with_capacity(rows.len());
    let mut nulls = None;

    match col {
        Column::Int { values, valid } => {
            fill(
                rows,
                valid.as_ref(),
                |idx| int(values[idx]),
                &mut ords,
                &mut nulls,
            );
        }
        Column::Float { values, valid } => {
            fill(
                rows,
                valid.as_ref(),
                |idx| float(values[idx]),
                &mut ords,
                &mut nulls,
            );
        }
        Column::Bool { values, valid } => {
            fill(
                rows,
                valid.as_ref(),
                |idx| bool(values.get(idx)),
                &mut ords,
                &mut nulls,
            );
        }
        Column::Dict { codes, valid, .. } => {
            fill(
                rows,
                valid.as_ref(),
                |idx| codes[idx] as u64,
                &mut ords,
                &mut nulls,
            );
        }
        Column::Text {
            bytes,
            offsets,
            valid,
        } => {
            fill(
                rows,
                valid.as_ref(),
                |idx| {
                    let start = offsets[idx] as usize;
                    let end = offsets[idx + 1] as usize;
                    text_prefix(&bytes[start..end])
                },
                &mut ords,
                &mut nulls,
            );
        }
    }

    (ords, nulls)
}

#[cfg(test)]
#[path = "test/ordinal/test.rs"]
mod test;
