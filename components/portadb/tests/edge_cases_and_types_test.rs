use portadb::value::FromValue;
use portadb::{Capabilities, ConnectOptions, Engine, Error, Feature, Row, Value};

#[test]
fn test_row_strict_null_vs_missing_column_semantics() {
    let row = Row::new(
        vec!["id".to_string(), "middle_name".to_string()],
        vec![Value::Integer(100), Value::Null],
    );

    // 1. Column exists with value
    assert_eq!(row.get::<i64>("id").unwrap(), 100);
    assert_eq!(row.get::<Option<i64>>("id").unwrap(), Some(100));

    // 2. Column exists with SQL NULL
    // Must return Ok(None) when queried as Option<T>
    assert_eq!(row.get::<Option<String>>("middle_name").unwrap(), None);

    // If queried as non-optional String, must fail with NullValue error
    assert!(matches!(
        row.get::<String>("middle_name"),
        Err(Error::NullValue { .. })
    ));

    // 3. Column is completely absent from query result
    // Querying absent column as Option<T> MUST RETURN Error::ColumnNotFound!
    // It must NOT silently return Ok(None) to prevent silent typos!
    let absent_as_opt = row.get::<Option<String>>("non_existent_column");
    assert!(
        matches!(absent_as_opt, Err(Error::ColumnNotFound { .. })),
        "querying missing column as Option must return ColumnNotFound"
    );

    // 4. Case-insensitivity in column lookup
    assert_eq!(row.get::<i64>("ID").unwrap(), 100);
    assert_eq!(row.get::<i64>("Id").unwrap(), 100);
    assert!(row.has_column("MIDDLE_NAME"));
    assert!(row.has_column("middle_name"));
    assert!(!row.has_column("random_col"));
}

#[test]
fn test_from_value_exhaustive_conversions() {
    // Integer conversions
    let val_int = Value::Integer(42);
    assert_eq!(i64::from_value("c", &val_int).unwrap(), 42i64);
    assert_eq!(i32::from_value("c", &val_int).unwrap(), 42i32);
    assert_eq!(f64::from_value("c", &val_int).unwrap(), 42.0f64);

    // Real conversions
    let val_real = Value::Real(42.5);
    assert!((f64::from_value("c", &val_real).unwrap() - 42.5).abs() < 1e-6);

    // Text conversions
    let val_text = Value::Text("Hello, Connection!".to_string());
    assert_eq!(
        String::from_value("c", &val_text).unwrap(),
        "Hello, Connection!"
    );

    // Blob conversions
    let val_blob = Value::Blob(vec![0xCA, 0xFE, 0xBA, 0xBE]);
    assert_eq!(
        <Vec<u8>>::from_value("c", &val_blob).unwrap(),
        vec![0xCA, 0xFE, 0xBA, 0xBE]
    );

    // Boolean conversions
    assert!(bool::from_value("c", &Value::Boolean(true)).unwrap());
    assert!(!bool::from_value("c", &Value::Boolean(false)).unwrap());
    assert!(bool::from_value("c", &Value::Integer(1)).unwrap());
    assert!(!bool::from_value("c", &Value::Integer(0)).unwrap());

    // Type mismatch returns Error::TypeMismatch
    let mismatch = i64::from_value("c", &Value::Text("not an int".to_string()));
    assert!(matches!(mismatch, Err(Error::TypeMismatch { .. })));
}

#[test]
fn test_capabilities_feature_requirements() {
    let mut caps = Capabilities::redis();
    assert!(!caps.supports(Feature::Savepoints));
    assert!(!caps.supports(Feature::RelationalDdl));

    // Requiring unsupported feature returns UnsupportedFeature error
    let req_result = caps.require(Feature::Savepoints, "redis");
    assert!(matches!(req_result, Err(Error::UnsupportedFeature(_))));

    // Enabling feature satisfies requirement
    caps.savepoints = true;
    assert!(caps.supports(Feature::Savepoints));
    assert!(caps.require(Feature::Savepoints, "redis").is_ok());
}

#[tokio::test]
async fn test_engine_downcast_raw_escape_hatch() {
    let options = ConnectOptions::new("sqlite::memory:");
    let engine = Engine::connect(&options).await.expect("connect");

    // `mut` only matters with the `rusqlite` feature (`downcast_raw_mut`
    // below); without it the compiler warns. Both builds must compile.
    #[allow(unused_mut)]
    let mut conn = engine.acquire().await.expect("acquire");

    // Valid downcast if rusqlite is active
    #[cfg(feature = "rusqlite")]
    if conn.engine_name() == "rusqlite" {
        use portadb::drivers::RusqliteConnection;
        let downcasted = conn.downcast_raw::<RusqliteConnection>();
        assert!(downcasted.is_ok(), "downcast to RusqliteConnection");
        let downcasted_mut = conn.downcast_raw_mut::<RusqliteConnection>();
        assert!(downcasted_mut.is_ok(), "downcast mut to RusqliteConnection");
    }

    // Invalid downcast to wrong type must return Error::DowncastFailed
    let invalid = conn.downcast_raw::<u32>();
    assert!(matches!(invalid, Err(Error::DowncastFailed)));
}
