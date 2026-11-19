use super::*;
use std::time::Duration;

#[test]
fn test_connect_options_builder() {
    let opts = ConnectOptions::new("sqlite://app.db")
        .with_max_connections(10)
        .with_connect_timeout(Duration::from_secs(5));

    assert_eq!(opts.url, "sqlite://app.db");
    assert_eq!(opts.max_connections, Some(10));
    assert_eq!(opts.connect_timeout, Some(Duration::from_secs(5)));
}
