use super::*;

#[test]
fn test_database_from_url() {
    assert_eq!(Database::from_url(":memory:"), Some(Database::Sqlite));
    assert_eq!(
        Database::from_url("sqlite::memory:"),
        Some(Database::Sqlite)
    );
    assert_eq!(
        Database::from_url("sqlite://:memory:"),
        Some(Database::Sqlite)
    );
    assert_eq!(
        Database::from_url("file:shared_x?mode=memory&cache=shared"),
        Some(Database::Sqlite)
    );
    assert_eq!(
        Database::from_url("sqlite://test.db"),
        Some(Database::Sqlite)
    );
    assert_eq!(
        Database::from_url("postgres://user:pass@localhost/db"),
        Some(Database::Postgres)
    );
    assert_eq!(
        Database::from_url("mysql://localhost/db"),
        Some(Database::MySql)
    );
    assert_eq!(
        Database::from_url("mssql://localhost/db"),
        Some(Database::SqlServer)
    );
    assert_eq!(Database::from_url("unknown://test"), None);
}
