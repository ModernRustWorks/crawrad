//! Memory consumption analysis across columnar storage variants and naive representations.
//!
//! Computes exact buffer allocations and capacities (no external allocator hooks needed)
//! to establish the baseline "bytes per row" for each storage technique.

use datagrid::{Bitmap, Column};
use std::collections::HashMap;
use std::mem::size_of;

struct MemoryRow {
    category: &'static str,
    technique: &'static str,
    rows: usize,
    total_bytes: usize,
    bytes_per_row: f64,
    notes: &'static str,
}

fn measure_column_int(col: &Column) -> usize {
    match col {
        Column::Int { values, valid } => {
            let mut bytes = size_of::<Column>() + values.capacity() * size_of::<i64>();
            if let Some(mask) = valid {
                bytes += size_of::<Bitmap>() + std::mem::size_of_val(mask.words());
            }
            bytes
        }
        _ => 0,
    }
}

fn measure_column_float(col: &Column) -> usize {
    match col {
        Column::Float { values, valid } => {
            let mut bytes = size_of::<Column>() + values.capacity() * size_of::<f64>();
            if let Some(mask) = valid {
                bytes += size_of::<Bitmap>() + std::mem::size_of_val(mask.words());
            }
            bytes
        }
        _ => 0,
    }
}

fn measure_column_bool(col: &Column) -> usize {
    match col {
        Column::Bool { values, valid } => {
            let mut bytes =
                size_of::<Column>() + size_of::<Bitmap>() + std::mem::size_of_val(values.words());
            if let Some(mask) = valid {
                bytes += size_of::<Bitmap>() + std::mem::size_of_val(mask.words());
            }
            bytes
        }
        _ => 0,
    }
}

fn measure_column_text(col: &Column) -> usize {
    match col {
        Column::Text {
            bytes,
            offsets,
            valid,
        } => {
            let mut total = size_of::<Column>()
                + bytes.capacity() * size_of::<u8>()
                + offsets.capacity() * size_of::<u32>();
            if let Some(mask) = valid {
                total += size_of::<Bitmap>() + std::mem::size_of_val(mask.words());
            }
            total
        }
        _ => 0,
    }
}

fn measure_column_dict(col: &Column) -> usize {
    match col {
        Column::Dict { codes, dict, valid } => {
            let mut total = size_of::<Column>()
                + codes.capacity() * size_of::<u32>()
                + dict.capacity() * size_of::<String>();
            for s in dict {
                total += s.capacity();
            }
            if let Some(mask) = valid {
                total += size_of::<Bitmap>() + std::mem::size_of_val(mask.words());
            }
            total
        }
        _ => 0,
    }
}

#[derive(Clone)]
struct SampleRowStruct {
    _id: i64,
    _amount: f64,
    _category: i64,
    _extra: u64,
}

fn main() {
    let rows = 100_000;
    let mut table: Vec<MemoryRow> = Vec::new();

    // 1. Integer columns: Column::Int vs Vec<Option<i64>>
    let int_dense = Column::ints((0..rows as i64).map(Some).collect());
    let bytes = measure_column_int(&int_dense);
    table.push(MemoryRow {
        category: "Numeric (Int)",
        technique: "Column::Int (dense, no nulls)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "values: Vec<i64>, valid: None",
    });

    let int_nulls = Column::ints(
        (0..rows as i64)
            .map(|i| if i % 10 == 0 { None } else { Some(i) })
            .collect(),
    );
    let bytes = measure_column_int(&int_nulls);
    table.push(MemoryRow {
        category: "Numeric (Int)",
        technique: "Column::Int (10% nulls)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "values: Vec<i64> + validity Bitmap (1 bit/row)",
    });

    let naive_options: Vec<Option<i64>> = (0..rows as i64)
        .map(|i| if i % 10 == 0 { None } else { Some(i) })
        .collect();
    let bytes = size_of::<Vec<Option<i64>>>() + naive_options.capacity() * size_of::<Option<i64>>();
    table.push(MemoryRow {
        category: "Numeric (Int)",
        technique: "Vec<Option<i64>> (10% nulls)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "size_of::<Option<i64>>() = 16 bytes (padding)",
    });

    // 2. Float columns
    let float_dense = Column::floats((0..rows).map(|i| Some(i as f64 + 0.5)).collect());
    let bytes = measure_column_float(&float_dense);
    table.push(MemoryRow {
        category: "Numeric (Float)",
        technique: "Column::Float (dense, no nulls)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "values: Vec<f64>",
    });

    // 3. Boolean columns
    let bool_dense = Column::bools((0..rows).map(|i| Some(i % 2 == 0)).collect());
    let bytes = measure_column_bool(&bool_dense);
    table.push(MemoryRow {
        category: "Boolean",
        technique: "Column::Bool (dense, no nulls)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "Dense Bitmap (1 bit/row = 0.125 B/row)",
    });

    let bool_nulls = Column::bools(
        (0..rows)
            .map(|i| if i % 10 == 0 { None } else { Some(i % 2 == 0) })
            .collect(),
    );
    let bytes = measure_column_bool(&bool_nulls);
    table.push(MemoryRow {
        category: "Boolean",
        technique: "Column::Bool (10% nulls)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "Values Bitmap + validity Bitmap (2 bits/row)",
    });

    let naive_bools: Vec<bool> = (0..rows).map(|i| i % 2 == 0).collect();
    let bytes = size_of::<Vec<bool>>() + naive_bools.capacity() * size_of::<bool>();
    table.push(MemoryRow {
        category: "Boolean",
        technique: "Vec<bool> (standard)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "1 byte per boolean",
    });

    // 4. Text columns: Plain vs Dict
    let strings_low: Vec<Option<String>> = (0..rows)
        .map(|i| Some(format!("Cat_{:02}", i % 5)))
        .collect();
    let text_plain_low = Column::texts(strings_low.clone());
    let bytes = measure_column_text(&text_plain_low);
    table.push(MemoryRow {
        category: "Text (Low Card)",
        technique: "Column::Text (plain, 5 distinct)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "bytes buffer + offsets (Vec<u32>)",
    });

    let text_dict_low = Column::dict_from(strings_low);
    let bytes = measure_column_dict(&text_dict_low);
    table.push(MemoryRow {
        category: "Text (Low Card)",
        technique: "Column::Dict (dict, 5 distinct)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "codes: Vec<u32> (4 B/row) + tiny dictionary",
    });

    let strings_med: Vec<Option<String>> = (0..rows)
        .map(|i| Some(format!("City_{:04}", i % 500)))
        .collect();
    let text_plain_med = Column::texts(strings_med.clone());
    let bytes = measure_column_text(&text_plain_med);
    table.push(MemoryRow {
        category: "Text (Medium Card)",
        technique: "Column::Text (plain, 500 distinct)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "bytes buffer + offsets (Vec<u32>)",
    });

    let text_dict_med = Column::dict_from(strings_med);
    let bytes = measure_column_dict(&text_dict_med);
    table.push(MemoryRow {
        category: "Text (Medium Card)",
        technique: "Column::Dict (dict, 500 distinct)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "codes: Vec<u32> + 500 dictionary entries",
    });

    let strings_high: Vec<Option<String>> =
        (0..rows).map(|i| Some(format!("Item_{:08}", i))).collect();
    let text_plain_high = Column::texts(strings_high.clone());
    let bytes = measure_column_text(&text_plain_high);
    table.push(MemoryRow {
        category: "Text (High Card)",
        technique: "Column::Text (plain, all distinct)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "bytes buffer + offsets (Vec<u32>)",
    });

    let text_dict_high = Column::dict_from(strings_high);
    let bytes = measure_column_dict(&text_dict_high);
    table.push(MemoryRow {
        category: "Text (High Card)",
        technique: "Column::Dict (dict, all distinct)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "codes: Vec<u32> + N dictionary strings (heavy)",
    });

    // 5. Row-oriented struct vs 4 columnar vectors
    let row_structs: Vec<SampleRowStruct> = (0..rows)
        .map(|i| SampleRowStruct {
            _id: i as i64,
            _amount: i as f64 * 0.1,
            _category: (i % 50) as i64,
            _extra: i as u64,
        })
        .collect();
    let bytes =
        size_of::<Vec<SampleRowStruct>>() + row_structs.capacity() * size_of::<SampleRowStruct>();
    table.push(MemoryRow {
        category: "Storage Layout",
        technique: "Vec<RowStruct> (4 fields: 3x i64 + 1x f64)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "32 bytes per row contiguous struct",
    });

    let col_4_bytes = rows * size_of::<i64>() * 3 + rows * size_of::<f64>();
    table.push(MemoryRow {
        category: "Storage Layout",
        technique: "Columnar Store (4 columns: 3x Int + 1x Float)",
        rows,
        total_bytes: col_4_bytes,
        bytes_per_row: col_4_bytes as f64 / rows as f64,
        notes: "32 bytes per row split into 4 column buffers",
    });

    // 6. Filter selection representations
    let filter_bm = Bitmap::new(rows, true);
    let bytes = size_of::<Bitmap>() + std::mem::size_of_val(filter_bm.words());
    table.push(MemoryRow {
        category: "Filter Result",
        technique: "Bitmap (arbitrary selectivity)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "Fixed 0.125 B/row regardless of match count",
    });

    let match_1p: Vec<u32> = (0..(rows / 100) as u32).collect();
    let bytes = size_of::<Vec<u32>>() + match_1p.capacity() * size_of::<u32>();
    table.push(MemoryRow {
        category: "Filter Result",
        technique: "Vec<u32> (1% selectivity)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "4 bytes per matching ID (0.04 B/row total)",
    });

    let match_50p: Vec<u32> = (0..(rows / 2) as u32).collect();
    let bytes = size_of::<Vec<u32>>() + match_50p.capacity() * size_of::<u32>();
    table.push(MemoryRow {
        category: "Filter Result",
        technique: "Vec<u32> (50% selectivity)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "4 bytes per matching ID (2.00 B/row total)",
    });

    let match_99p: Vec<u32> = (0..(rows * 99 / 100) as u32).collect();
    let bytes = size_of::<Vec<u32>>() + match_99p.capacity() * size_of::<u32>();
    table.push(MemoryRow {
        category: "Filter Result",
        technique: "Vec<u32> (99% selectivity)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "4 bytes per matching ID (3.96 B/row total)",
    });

    // 7. Index structures
    let mut hash_index: HashMap<i64, u32> = HashMap::with_capacity(rows);
    for i in 0..rows as i64 {
        hash_index.insert(i, i as u32);
    }
    // HashBrown table has capacity * (size_of::<(K, V)>() + 1 byte control byte)
    let hash_cap = hash_index.capacity();
    let bytes = size_of::<HashMap<i64, u32>>() + hash_cap * (size_of::<(i64, u32)>() + 1);
    table.push(MemoryRow {
        category: "Lookup Index",
        technique: "HashMap<i64, u32> (point index)",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "Hash table bucket array + control bytes",
    });

    let sorted_permutation: Vec<u32> = (0..rows as u32).collect();
    let bytes = size_of::<Vec<u32>>() + sorted_permutation.capacity() * size_of::<u32>();
    table.push(MemoryRow {
        category: "Lookup Index",
        technique: "Sorted Permutation Vec<u32>",
        rows,
        total_bytes: bytes,
        bytes_per_row: bytes as f64 / rows as f64,
        notes: "4 bytes per row ID for binary search",
    });

    // Print table
    println!("\n=== CrawRAD Storage Techniques: Memory Footprint (Bytes per Row) ===");
    println!("Evaluated on {} rows per variant.\n", rows);

    println!(
        "+-{:-<20}-+-{:-<45}-+-{:-<10}-+-{:-<13}-+-{:-<13}-+-{:-<45}-+",
        "", "", "", "", "", ""
    );
    println!(
        "| {:<20} | {:<45} | {:>10} | {:>13} | {:>13} | {:<45} |",
        "Category", "Technique", "Rows", "Total Bytes", "Bytes / Row", "Notes / Layout Details"
    );
    println!(
        "+-{:-<20}-+-{:-<45}-+-{:-<10}-+-{:-<13}-+-{:-<13}-+-{:-<45}-+",
        "", "", "", "", "", ""
    );

    for r in &table {
        println!(
            "| {:<20} | {:<45} | {:>10} | {:>13} | {:>13.4} | {:<45} |",
            r.category, r.technique, r.rows, r.total_bytes, r.bytes_per_row, r.notes
        );
    }
    println!(
        "+-{:-<20}-+-{:-<45}-+-{:-<10}-+-{:-<13}-+-{:-<13}-+-{:-<45}-+",
        "", "", "", "", "", ""
    );
}
