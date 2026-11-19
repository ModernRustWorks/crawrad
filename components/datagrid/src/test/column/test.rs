use super::*;

#[test]
fn test_ints_round_trip() {
    let input = vec![Some(42), None, Some(-100), Some(0), None];
    let col = Column::ints(input.clone());
    assert_eq!(col.len(), 5);
    assert!(!col.is_empty());
    assert!(!col.is_null(0));
    assert!(col.is_null(1));
    assert!(!col.is_null(2));
    assert!(!col.is_null(3));
    assert!(col.is_null(4));

    assert_eq!(col.get(0), Value::Int(42));
    assert_eq!(col.get(1), Value::Null);
    assert_eq!(col.get(2), Value::Int(-100));
    assert_eq!(col.get(3), Value::Int(0));
    assert_eq!(col.get(4), Value::Null);

    let no_nulls = vec![Some(1), Some(2), Some(3)];
    let col_clean = Column::ints(no_nulls);
    assert!(col_clean.validity().is_none());
    assert_eq!(col_clean.get(1), Value::Int(2));
}

#[test]
fn test_floats_round_trip() {
    let input = vec![Some(2.5), None, Some(-0.0), Some(f64::INFINITY)];
    let col = Column::floats(input);
    assert_eq!(col.len(), 4);
    assert!(!col.is_null(0));
    assert!(col.is_null(1));
    assert_eq!(col.get(0), Value::Float(2.5));
    assert_eq!(col.get(1), Value::Null);
    assert_eq!(col.get(2), Value::Float(-0.0));
    assert_eq!(col.get(3), Value::Float(f64::INFINITY));
}

#[test]
fn test_bools_round_trip() {
    let input = vec![Some(true), Some(false), None, Some(true), None];
    let col = Column::bools(input);
    assert_eq!(col.len(), 5);
    assert_eq!(col.get(0), Value::Bool(true));
    assert_eq!(col.get(1), Value::Bool(false));
    assert_eq!(col.get(2), Value::Null);
    assert_eq!(col.get(3), Value::Bool(true));
    assert_eq!(col.get(4), Value::Null);
}

#[test]
fn test_texts_plain_round_trip() {
    let input = vec![
        Some("hello".to_string()),
        None,
        Some("world".to_string()),
        Some("".to_string()),
    ];
    let col = Column::texts(input);
    assert_eq!(col.len(), 4);
    assert_eq!(col.get(0), Value::Text("hello"));
    assert_eq!(col.get(1), Value::Null);
    assert_eq!(col.get(2), Value::Text("world"));
    assert_eq!(col.get(3), Value::Text(""));
}

#[test]
fn test_dict_sorted_and_mapped() {
    let input = vec![
        Some("zebra".to_string()),
        Some("apple".to_string()),
        None,
        Some("mango".to_string()),
        Some("apple".to_string()),
        Some("zebra".to_string()),
    ];
    let col = Column::dict_from(input);
    match &col {
        Column::Dict { codes, dict, valid } => {
            assert_eq!(dict, &["apple", "mango", "zebra"]);
            assert!(valid.is_some());
            assert_eq!(codes[0], 2); // zebra
            assert_eq!(codes[1], 0); // apple
            assert_eq!(codes[2], 0); // null placeholder
            assert_eq!(codes[3], 1); // mango
            assert_eq!(codes[4], 0); // apple
            assert_eq!(codes[5], 2); // zebra
        }
        _ => panic!("expected dict column"),
    }
    assert_eq!(col.get(0), Value::Text("zebra"));
    assert_eq!(col.get(1), Value::Text("apple"));
    assert_eq!(col.get(2), Value::Null);
    assert_eq!(col.get(3), Value::Text("mango"));
    assert_eq!(col.get(4), Value::Text("apple"));
    assert_eq!(col.get(5), Value::Text("zebra"));
}

#[test]
fn test_texts_auto_rule() {
    // 20 rows, 4 distinct values (4 <= 20 / 4 = 5) -> Dict
    let dict_input = (0..20)
        .map(|i| Some(format!("city_{}", i % 4)))
        .collect::<Vec<_>>();
    let col_dict = Column::texts_auto(dict_input);
    assert!(matches!(col_dict, Column::Dict { .. }));

    // 20 rows, 10 distinct values (10 <= 20 / 4 = 5 is false) -> Plain Text
    let text_input = (0..20)
        .map(|i| Some(format!("uuid_{}", i % 10)))
        .collect::<Vec<_>>();
    let col_text = Column::texts_auto(text_input);
    assert!(matches!(col_text, Column::Text { .. }));
}

#[test]
fn test_dict_exact_capacity() {
    let mut items = Vec::with_capacity(10_000);
    for i in 0..10_000 {
        items.push(Some(format!("item_{}", i % 5)));
    }
    let col = Column::dict_from(items);
    match col {
        Column::Dict { dict, codes, .. } => {
            assert_eq!(dict.len(), 5);
            assert_eq!(dict.capacity(), dict.len());
            assert_eq!(codes.len(), 10_000);
        }
        _ => panic!("expected dict column"),
    }
}
