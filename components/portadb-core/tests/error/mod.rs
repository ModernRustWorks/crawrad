use super::*;

#[test]
fn test_error_predicates_and_display() {
    let err_unique = Error::UniqueViolation {
        constraint: Some("users_email_key".into()),
        message: "duplicate key value".into(),
    };
    assert!(err_unique.is_unique_violation());
    assert!(!err_unique.is_foreign_key_violation());
    assert!(!err_unique.is_connection_lost());
    assert!(
        err_unique
            .to_string()
            .contains("unique constraint violation")
    );

    let err_fk = Error::ForeignKeyViolation {
        constraint: None,
        message: "fk failed".into(),
    };
    assert!(!err_fk.is_unique_violation());
    assert!(err_fk.is_foreign_key_violation());
    assert!(!err_fk.is_connection_lost());

    let err_conn = Error::ConnectionLost("server closed connection".into());
    assert!(!err_conn.is_unique_violation());
    assert!(!err_conn.is_foreign_key_violation());
    assert!(err_conn.is_connection_lost());

    let err_q = Error::query_error("syntax error near SELECT");
    assert!(matches!(err_q, Error::QueryError(_)));

    let io_err = std::io::Error::new(std::io::ErrorKind::BrokenPipe, "pipe broken");
    let err_eng = Error::from_engine(io_err);
    assert!(matches!(err_eng, Error::Engine(_)));
}
