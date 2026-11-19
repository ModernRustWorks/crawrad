use super::*;

#[test]
fn columns_and_answer_work_on_sqlite_table() {
    let temp_dir = std::env::temp_dir();
    let db_path = temp_dir.join(format!("test_probe_{}.db", std::process::id()));
    let _ = std::fs::remove_file(&db_path);
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let options = portadb::ConnectOptions::new(&url);
        let engine = portadb::Engine::connect(&options).await.unwrap();
        engine
            .execute(
                "CREATE TABLE clients (id INTEGER PRIMARY KEY, name TEXT, active INTEGER);",
                &portadb::params![],
            )
            .await
            .unwrap();
    });

    let cols = columns(&url, "clients").unwrap();
    assert_eq!(cols, vec!["id", "name", "active"]);

    let err_missing = columns(&url, "missing_table").unwrap_err();
    assert_eq!(err_missing, "no table named 'missing_table'");

    let err_empty_url = columns("", "clients").unwrap_err();
    assert_eq!(err_empty_url, "URL cannot be empty");

    let (code_no_table, json_no_table) = answer(&["--url".to_string(), url.clone()]);
    assert_eq!(code_no_table, 1);
    let val: serde_json::Value = serde_json::from_str(&json_no_table).unwrap();
    assert!(val.get("error").is_some());

    let (code_ok, json_ok) = answer(&[
        "--url".to_string(),
        url.clone(),
        "--table".to_string(),
        "clients".to_string(),
    ]);
    assert_eq!(code_ok, 0);
    let val_ok: serde_json::Value = serde_json::from_str(&json_ok).unwrap();
    assert_eq!(
        val_ok["columns"],
        serde_json::json!(["id", "name", "active"])
    );

    let _ = std::fs::remove_file(&db_path);
}

#[test]
fn a_missing_database_file_is_reported_and_not_created() {
    let db_path = std::env::temp_dir().join(format!("test_probe_absent_{}.db", std::process::id()));
    let _ = std::fs::remove_file(&db_path);
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let err = columns(&url, "clients").unwrap_err();
    assert!(err.starts_with("database file not found"), "{err}");
    assert!(
        !db_path.exists(),
        "the probe created the database it was asked about"
    );
}

#[test]
fn sample_and_answer_with_rows() {
    let temp_dir = std::env::temp_dir();
    let db_path = temp_dir.join(format!("test_probe_sample_{}.db", std::process::id()));
    let _ = std::fs::remove_file(&db_path);
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let long_name = "a".repeat(300);

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let options = portadb::ConnectOptions::new(&url);
        let engine = portadb::Engine::connect(&options).await.unwrap();
        engine
            .execute(
                "CREATE TABLE clients (id INTEGER PRIMARY KEY, name TEXT, active INTEGER);",
                &portadb::params![],
            )
            .await
            .unwrap();
        engine
            .execute(
                "INSERT INTO clients (id, name, active) VALUES (1, NULL, 1);",
                &portadb::params![],
            )
            .await
            .unwrap();
        engine
            .execute(
                &format!("INSERT INTO clients (id, name, active) VALUES (2, '{long_name}', 0);"),
                &portadb::params![],
            )
            .await
            .unwrap();
        engine
            .execute(
                "INSERT INTO clients (id, name, active) VALUES (3, 'Alice', 1);",
                &portadb::params![],
            )
            .await
            .unwrap();
    });

    let s = sample(&url, "clients", 2).unwrap();
    assert_eq!(s.columns, vec!["id", "name", "active"]);
    assert_eq!(s.kinds, vec!["INTEGER", "TEXT", "INTEGER"]);
    assert_eq!(s.rows.len(), 2);
    assert_eq!(s.total, 3);
    assert_eq!(s.rows[0], vec![Some("1".into()), None, Some("1".into())]);
    assert_eq!(s.rows[1][0], Some("2".into()));
    assert!(s.rows[1][1].as_deref().unwrap().ends_with('…'));
    assert_eq!(s.rows[1][1].as_deref().unwrap().chars().count(), 201);
    assert_eq!(s.rows[1][2], Some("0".into()));

    let s_clamped_low = sample(&url, "clients", 0).unwrap();
    assert_eq!(s_clamped_low.rows.len(), 1);

    let s_clamped_high = sample(&url, "clients", 999).unwrap();
    assert_eq!(s_clamped_high.rows.len(), 3);

    let (code_ok, json_ok) = answer(&[
        "--url".to_string(),
        url.clone(),
        "--table".to_string(),
        "clients".to_string(),
        "--rows".to_string(),
        "2".to_string(),
    ]);
    assert_eq!(code_ok, 0);
    let val_ok: serde_json::Value = serde_json::from_str(&json_ok).unwrap();
    assert_eq!(
        val_ok["columns"],
        serde_json::json!(["id", "name", "active"])
    );
    assert_eq!(
        val_ok["kinds"],
        serde_json::json!(["INTEGER", "TEXT", "INTEGER"])
    );
    assert_eq!(val_ok["rows"].as_array().unwrap().len(), 2);
    assert_eq!(val_ok["total"], 3);

    let (code_eq_ok, json_eq_ok) = answer(&[
        "--url".to_string(),
        url.clone(),
        "--table".to_string(),
        "clients".to_string(),
        "--rows=2".to_string(),
    ]);
    assert_eq!(code_eq_ok, 0);
    let val_eq_ok: serde_json::Value = serde_json::from_str(&json_eq_ok).unwrap();
    assert_eq!(val_eq_ok["rows"].as_array().unwrap().len(), 2);
    assert_eq!(val_eq_ok["total"], 3);

    let (code_err, json_err) = answer(&[
        "--url".to_string(),
        url.clone(),
        "--table".to_string(),
        "clients".to_string(),
        "--rows".to_string(),
        "abc".to_string(),
    ]);
    assert_eq!(code_err, 1);
    let val_err: serde_json::Value = serde_json::from_str(&json_err).unwrap();
    assert_eq!(val_err["error"], "--rows needs a number");

    let (code_eq_err, json_eq_err) = answer(&[
        "--url".to_string(),
        url.clone(),
        "--table".to_string(),
        "clients".to_string(),
        "--rows=abc".to_string(),
    ]);
    assert_eq!(code_eq_err, 1);
    let val_eq_err: serde_json::Value = serde_json::from_str(&json_eq_err).unwrap();
    assert_eq!(val_eq_err["error"], "--rows needs a number");

    let _ = std::fs::remove_file(&db_path);
}

#[test]
fn sample_missing_database_file_is_reported_and_not_created() {
    let temp_dir = std::env::temp_dir();
    let db_path = temp_dir.join(format!(
        "test_probe_sample_absent_{}.db",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&db_path);
    let url = format!("sqlite://{}", db_path.to_string_lossy());

    let err = sample(&url, "clients", 2).unwrap_err();
    assert!(err.starts_with("database file not found"), "{err}");
    assert!(
        !db_path.exists(),
        "the probe created the database it was asked about"
    );
    let _ = std::fs::remove_file(&db_path);
}

/// A Dataset's `sql`: the probe reads the query's columns, its first rows in
/// the query's order, and its count — and `--sql` answers the same.
#[test]
fn sample_sql_reads_a_query() {
    let db_path = std::env::temp_dir().join(format!("test_probe_sql_{}.db", std::process::id()));
    let _ = std::fs::remove_file(&db_path);
    let url = format!("sqlite://{}", db_path.to_string_lossy());
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let engine = portadb::Engine::connect(&portadb::ConnectOptions::new(&url))
            .await
            .unwrap();
        for sql in [
            "CREATE TABLE contacts (id INTEGER PRIMARY KEY, name TEXT, city TEXT);",
            "INSERT INTO contacts VALUES (1, 'Zoe', 'Rio'), (2, 'Ana', 'Lima'), (3, 'Bia', NULL);",
        ] {
            engine.execute(sql, &portadb::params![]).await.unwrap();
        }
    });
    let s = sample_sql(&url, " SELECT name, city FROM contacts ORDER BY name; ", 2).unwrap();
    assert_eq!(s.columns, vec!["name", "city"]);
    assert_eq!(s.kinds, vec!["TEXT", "TEXT"]);
    assert_eq!(s.total, 3);
    assert_eq!(
        s.rows,
        vec![
            vec![Some("Ana".into()), Some("Lima".into())],
            vec![Some("Bia".into()), None],
        ]
    );
    let args: Vec<String> = [
        "--url",
        &url,
        "--sql",
        "SELECT name FROM contacts",
        "--rows",
        "8",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect();
    let (code, json) = answer(&args);
    assert_eq!(code, 0, "{json}");
    assert!(json.contains("\"total\":3"), "{json}");
    assert!(
        sample_sql(&url, "   ", 8).is_err(),
        "an empty query is refused"
    );
    let _ = std::fs::remove_file(&db_path);
}
