use super::*;

#[test]
fn test_int_order_extremes_and_round_trip() {
    let test_values = [
        i64::MIN,
        i64::MIN + 1,
        -1_000_000,
        -1,
        0,
        1,
        1_000_000,
        i64::MAX - 1,
        i64::MAX,
    ];

    // Strictly monotonic order preserved
    for w in test_values.windows(2) {
        assert!(int(w[0]) < int(w[1]), "failed for {} < {}", w[0], w[1]);
    }

    // Exact round trip
    for &v in &test_values {
        assert_eq!(int_back(int(v)), v);
    }
}

#[test]
fn test_float_order_extremes_and_round_trip() {
    let neg_inf = float(f64::NEG_INFINITY);
    let neg_one_point_five = float(-1.5);
    let neg_zero = float(-0.0);
    let pos_zero = float(0.0);
    let two = float(2.0);
    let pos_inf = float(f64::INFINITY);
    let nan = float(f64::NAN);

    // -inf < -1.5 < -0.0 == 0.0 < 2 < inf < NaN
    assert!(neg_inf < neg_one_point_five);
    assert!(neg_one_point_five < neg_zero);
    assert_eq!(neg_zero, pos_zero);
    assert!(pos_zero < two);
    assert!(two < pos_inf);
    assert!(pos_inf < nan);
    assert_eq!(nan, u64::MAX);

    // float_back round trip
    assert_eq!(float_back(neg_inf), f64::NEG_INFINITY);
    assert_eq!(float_back(neg_one_point_five), -1.5);
    assert_eq!(float_back(pos_zero), 0.0);
    assert_eq!(float_back(neg_zero), 0.0); // -0.0 canonicalized to 0.0
    assert_eq!(float_back(two), 2.0);
    assert_eq!(float_back(pos_inf), f64::INFINITY);
    assert!(float_back(nan).is_nan());
}

#[test]
fn test_bool_order() {
    assert_eq!(bool(false), 0);
    assert_eq!(bool(true), 1);
    assert!(bool(false) < bool(true));
}

#[test]
fn test_text_prefix_order() {
    assert!(text_prefix(b"apple") < text_prefix(b"banana"));
    assert_eq!(text_prefix(b"abcdefgh"), text_prefix(b"abcdefgh1"));
    assert!(text_prefix(b"abcdefgh1") < text_prefix(b"abcdefgi"));
}
