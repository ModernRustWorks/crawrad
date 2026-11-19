use crate::column::Column;
use crate::filter::{Filter, Owned};
use crate::group::{Aggregate, Total};
use crate::query::{GroupLevel, Query};
use crate::sort::{Nulls, Order, SortKey};
use crate::store::Store;
use crate::view::{GroupNode, View};
use std::hash::{DefaultHasher, Hash, Hasher};

struct XorShift64(u64);

impl XorShift64 {
    fn new(seed: u64) -> Self {
        let s = if seed == 0 { 0x853c49e6748fea9b } else { seed };
        Self(s)
    }

    fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    fn next_range(&mut self, min: u64, max: u64) -> u64 {
        assert!(min <= max);
        let range = max - min + 1;
        min + (self.next_u64() % range)
    }

    fn next_bool(&mut self) -> bool {
        (self.next_u64() & 1) == 1
    }

    fn next_f64(&mut self) -> f64 {
        match self.next_range(0, 11) {
            0 => 0.0,
            1 => -0.0,
            2 => f64::NAN,
            3 => -f64::NAN,
            4 => 10.0,
            5 => 20.0,
            6 => 30.0,
            7 => -15.5,
            8 => 50.0,
            9 => 0.0,
            10 => -0.0,
            _ => self.next_range(1, 100) as f64,
        }
    }

    #[cfg(feature = "serde")]
    fn next_finite_f64(&mut self) -> f64 {
        match self.next_range(0, 7) {
            0 => 0.0,
            1 => -0.0,
            2 => 10.0,
            3 => 20.0,
            4 => -15.5,
            5 => 50.0,
            6 => 0.0,
            _ => self.next_range(1, 100) as f64,
        }
    }
}

fn hash_val<T: Hash>(val: &T) -> u64 {
    let mut hasher = DefaultHasher::new();
    val.hash(&mut hasher);
    hasher.finish()
}

fn assert_totals_match(actual: &[Option<f64>], expected: &[Option<f64>]) {
    assert_eq!(actual.len(), expected.len());
    for (a, e) in actual.iter().zip(expected.iter()) {
        match (a, e) {
            (None, None) => {}
            (Some(va), Some(ve)) => {
                let match_ok = va == ve
                    || (va.is_nan() && ve.is_nan())
                    || (va - ve).abs() <= 1e-9 * va.abs().max(ve.abs()).max(1.0);
                assert!(match_ok, "total mismatch: actual {} vs expected {}", va, ve);
            }
            _ => panic!(
                "total presence mismatch: actual {:?} vs expected {:?}",
                a, e
            ),
        }
    }
}

fn assert_group_trees_equal(g1: &[GroupNode], g2: &[GroupNode]) {
    assert_eq!(g1.len(), g2.len());
    for (n1, n2) in g1.iter().zip(g2.iter()) {
        assert_eq!(n1.key, n2.key);
        assert_eq!(n1.level, n2.level);
        assert_eq!(n1.row_count, n2.row_count);
        assert_eq!(n1.rows, n2.rows);
        assert_totals_match(&n1.totals, &n2.totals);
        assert_group_trees_equal(&n1.children, &n2.children);
    }
}

#[test]
fn test_owned_float_equality_and_hashing() {
    let f_zero = Owned::Float(0.0);
    let f_neg_zero = Owned::Float(-0.0);
    assert_eq!(f_zero, f_neg_zero);
    assert_eq!(hash_val(&f_zero), hash_val(&f_neg_zero));

    let f_nan1 = Owned::Float(f64::NAN);
    let f_nan2 = Owned::Float(-f64::NAN);
    assert_eq!(f_nan1, f_nan2);
    assert_eq!(hash_val(&f_nan1), hash_val(&f_nan2));

    let f_val = Owned::Float(42.5);
    assert_ne!(f_zero, f_val);
}

#[test]
fn test_owned_from_conversions() {
    assert_eq!(Owned::from(42i64), Owned::Int(42));
    assert_eq!(Owned::from(42i32), Owned::Int(42));
    assert_eq!(Owned::from(2.5f64), Owned::Float(2.5));
    assert_eq!(Owned::from(true), Owned::Bool(true));
    assert_eq!(Owned::from("hello"), Owned::Text("hello".into()));
    assert_eq!(
        Owned::from("world".to_string()),
        Owned::Text("world".into())
    );
}

#[test]
fn test_builder_produces_same_value_as_handwritten() {
    let built = Query::new()
        .filter(Filter::gt("score", 30.0).and(!Filter::is_null("tag")))
        .sort_asc("score")
        .sort_desc_nulls_first("name")
        .group("region")
        .with_total(Total::sum("amount"))
        .group("city");

    let handwritten = Query {
        filter: Some(Filter::And(vec![
            Filter::Gt("score".into(), Owned::Float(30.0)),
            Filter::Not(Box::new(Filter::IsNull("tag".into()))),
        ])),
        sort: vec![
            SortKey {
                column: "score".into(),
                order: Order::Asc,
                nulls: Nulls::Last,
            },
            SortKey {
                column: "name".into(),
                order: Order::Desc,
                nulls: Nulls::First,
            },
        ],
        group_by: vec![
            GroupLevel {
                column: "region".into(),
                totals: vec![Total {
                    column: "amount".into(),
                    aggregate: Aggregate::Sum,
                }],
            },
            GroupLevel {
                column: "city".into(),
                totals: Vec::new(),
            },
        ],
        summary: Vec::new(),
    };

    assert_eq!(built, handwritten);
    assert_eq!(hash_val(&built), hash_val(&handwritten));
}

#[test]
fn test_all_filter_builder_helpers() {
    assert_eq!(Filter::eq("c", 1), Filter::Eq("c".into(), Owned::Int(1)));
    assert_eq!(Filter::lt("c", 2), Filter::Lt("c".into(), Owned::Int(2)));
    assert_eq!(Filter::le("c", 3), Filter::Le("c".into(), Owned::Int(3)));
    assert_eq!(Filter::gt("c", 4), Filter::Gt("c".into(), Owned::Int(4)));
    assert_eq!(Filter::ge("c", 5), Filter::Ge("c".into(), Owned::Int(5)));
    assert_eq!(
        Filter::between("c", 1, 10),
        Filter::Between("c".into(), Owned::Int(1), Owned::Int(10))
    );
    assert_eq!(
        Filter::in_list("c", [1, 2, 3]),
        Filter::In(
            "c".into(),
            vec![Owned::Int(1), Owned::Int(2), Owned::Int(3)]
        )
    );
    assert_eq!(Filter::is_null("c"), Filter::IsNull("c".into()));
    assert_eq!(
        Filter::contains("c", "foo"),
        Filter::Contains {
            column: "c".into(),
            text: "foo".into(),
            ignore_case: false,
        }
    );
    assert_eq!(
        Filter::starts_with("c", "foo"),
        Filter::StartsWith {
            column: "c".into(),
            text: "foo".into(),
            ignore_case: false,
        }
    );
    assert_eq!(
        Filter::contains_ignore_case("c", "foo"),
        Filter::Contains {
            column: "c".into(),
            text: "foo".into(),
            ignore_case: true,
        }
    );
    assert_eq!(
        Filter::starts_with_ignore_case("c", "foo"),
        Filter::StartsWith {
            column: "c".into(),
            text: "foo".into(),
            ignore_case: true,
        }
    );

    // Chaining .and() appends to an existing And instead of nesting
    let f_and = Filter::eq("a", 1)
        .and(Filter::eq("b", 2))
        .and(Filter::eq("c", 3));
    assert_eq!(
        f_and,
        Filter::And(vec![
            Filter::Eq("a".into(), Owned::Int(1)),
            Filter::Eq("b".into(), Owned::Int(2)),
            Filter::Eq("c".into(), Owned::Int(3)),
        ])
    );

    // Chaining .or() appends to an existing Or
    let f_or = Filter::eq("a", 1)
        .or(Filter::eq("b", 2))
        .or(Filter::eq("c", 3));
    assert_eq!(
        f_or,
        Filter::Or(vec![
            Filter::Eq("a".into(), Owned::Int(1)),
            Filter::Eq("b".into(), Owned::Int(2)),
            Filter::Eq("c".into(), Owned::Int(3)),
        ])
    );
}

#[test]
fn test_all_total_builder_helpers() {
    assert_eq!(
        Total::count("amount"),
        Total {
            column: "amount".into(),
            aggregate: Aggregate::Count,
        }
    );
    assert_eq!(
        Total::sum("amount"),
        Total {
            column: "amount".into(),
            aggregate: Aggregate::Sum,
        }
    );
    assert_eq!(
        Total::min("amount"),
        Total {
            column: "amount".into(),
            aggregate: Aggregate::Min,
        }
    );
    assert_eq!(
        Total::max("amount"),
        Total {
            column: "amount".into(),
            aggregate: Aggregate::Max,
        }
    );
    assert_eq!(
        Total::avg("amount"),
        Total {
            column: "amount".into(),
            aggregate: Aggregate::Avg,
        }
    );
}

#[test]
fn test_query_sort_builder_helpers() {
    let q = Query::new()
        .sort_asc("c1")
        .sort_desc("c2")
        .sort_asc_nulls_first("c3")
        .sort_desc_nulls_first("c4");

    assert_eq!(
        q.sort,
        vec![
            SortKey {
                column: "c1".into(),
                order: Order::Asc,
                nulls: Nulls::Last,
            },
            SortKey {
                column: "c2".into(),
                order: Order::Desc,
                nulls: Nulls::Last,
            },
            SortKey {
                column: "c3".into(),
                order: Order::Asc,
                nulls: Nulls::First,
            },
            SortKey {
                column: "c4".into(),
                order: Order::Desc,
                nulls: Nulls::First,
            },
        ]
    );
}

#[test]
#[should_panic(
    expected = "Query::with_total called without any group level; call .group(col) first"
)]
fn test_with_total_without_group_panics() {
    let _ = Query::new().with_total(Total::count("col"));
}

#[test]
fn test_per_level_totals() {
    let mut store = Store::new();
    store
        .add(
            "dept",
            Column::dict_from(vec![
                Some("Engineering".into()),
                Some("Engineering".into()),
                Some("Sales".into()),
                Some("Sales".into()),
            ]),
        )
        .unwrap();
    store
        .add(
            "team",
            Column::dict_from(vec![
                Some("Frontend".into()),
                Some("Backend".into()),
                Some("Direct".into()),
                Some("Channel".into()),
            ]),
        )
        .unwrap();
    store
        .add(
            "salary",
            Column::ints(vec![Some(100), Some(200), Some(300), Some(400)]),
        )
        .unwrap();

    let query = Query::new()
        .group("dept")
        .with_total(Total::sum("salary"))
        .group("team")
        .with_total(Total::count("salary"));

    let view = View::new(&store, query).unwrap();
    let groups = view.groups();
    assert_eq!(groups.len(), 2);

    // Level 0: Engineering has sum=300.0, Sales has sum=700.0
    assert_eq!(groups[0].totals, vec![Some(300.0)]);
    assert_eq!(groups[1].totals, vec![Some(700.0)]);

    // Level 1: Frontend, Backend under Engineering; Direct, Channel under Sales
    assert_eq!(groups[0].children.len(), 2);
    assert_eq!(groups[0].children[0].totals, vec![Some(1.0)]);
    assert_eq!(groups[0].children[1].totals, vec![Some(1.0)]);

    assert_eq!(groups[1].children.len(), 2);
    assert_eq!(groups[1].children[0].totals, vec![Some(1.0)]);
    assert_eq!(groups[1].children[1].totals, vec![Some(1.0)]);
}

#[cfg(feature = "serde")]
#[test]
fn test_serde_round_trip_handbuilt() {
    let queries = vec![
        Query::default(),
        Query::new().filter(Filter::gt("val", 10)).sort_asc("val"),
        Query::new()
            .filter(Filter::gt("score", 30.0).and(!Filter::is_null("tag")))
            .sort_asc("score")
            .sort_desc_nulls_first("name")
            .group("region")
            .with_total(Total::sum("amount"))
            .group("city"),
        Query::new().filter(Filter::in_list("cat", vec!["A", "B", "C"])),
    ];

    for q in queries {
        let json = serde_json::to_string(&q).expect("serialize query to json");
        let deserialized: Query = serde_json::from_str(&json).expect("deserialize query from json");
        assert_eq!(deserialized, q);
        assert_eq!(hash_val(&deserialized), hash_val(&q));
    }

    // Check JSON representation of Owned
    assert_eq!(
        serde_json::to_string(&Owned::Int(5)).unwrap(),
        "{\"Int\":5}"
    );
    assert_eq!(
        serde_json::to_string(&Owned::Text("foo".into())).unwrap(),
        "{\"Text\":\"foo\"}"
    );
    assert_eq!(
        serde_json::to_string(&Owned::Bool(true)).unwrap(),
        "{\"Bool\":true}"
    );
    assert_eq!(
        serde_json::to_string(&Owned::Float(12.5)).unwrap(),
        "{\"Float\":12.5}"
    );
}

fn generate_random_store(rng: &mut XorShift64, row_count: usize) -> Store {
    let mut store = Store::new();

    let c_int: Vec<Option<i64>> = (0..row_count)
        .map(|_| {
            if rng.next_bool() {
                Some(rng.next_range(0, 100) as i64)
            } else {
                None
            }
        })
        .collect();

    let c_float: Vec<Option<f64>> = (0..row_count)
        .map(|_| {
            if rng.next_bool() {
                Some(rng.next_f64())
            } else {
                None
            }
        })
        .collect();

    let c_bool: Vec<Option<bool>> = (0..row_count)
        .map(|_| {
            if rng.next_bool() {
                Some(rng.next_bool())
            } else {
                None
            }
        })
        .collect();

    let c_dict: Vec<Option<String>> = (0..row_count)
        .map(|_| {
            if rng.next_bool() {
                Some(format!("cat_{}", rng.next_range(0, 4)))
            } else {
                None
            }
        })
        .collect();

    let c_text: Vec<Option<String>> = (0..row_count)
        .map(|_| {
            if rng.next_bool() {
                Some(format!("text_{}", rng.next_range(0, 4)))
            } else {
                None
            }
        })
        .collect();

    store.add("c_int", Column::ints(c_int)).unwrap();
    store.add("c_float", Column::floats(c_float)).unwrap();
    store.add("c_bool", Column::bools(c_bool)).unwrap();
    store.add("c_dict", Column::dict_from(c_dict)).unwrap();
    store.add("c_text", Column::texts(c_text)).unwrap();

    store
}

fn generate_random_filter(rng: &mut XorShift64, depth: usize) -> Filter {
    if depth >= 4 {
        generate_random_leaf_filter(rng)
    } else {
        match rng.next_range(0, 4) {
            0 => {
                let num_children = rng.next_range(1, 3) as usize;
                let mut children = Vec::with_capacity(num_children + 1);
                for _ in 0..num_children {
                    children.push(generate_random_filter(rng, depth + 1));
                }
                // Intentionally inject a duplicate child or nested And
                if rng.next_bool() && !children.is_empty() {
                    children.push(children[0].clone());
                }
                if rng.next_bool() && !children.is_empty() {
                    let sub = generate_random_filter(rng, depth + 1);
                    children.push(Filter::And(vec![sub]));
                }
                Filter::And(children)
            }
            1 => {
                let num_children = rng.next_range(1, 3) as usize;
                let mut children = Vec::with_capacity(num_children + 1);
                for _ in 0..num_children {
                    children.push(generate_random_filter(rng, depth + 1));
                }
                if rng.next_bool() && !children.is_empty() {
                    children.push(children[0].clone());
                }
                if rng.next_bool() && !children.is_empty() {
                    let sub = generate_random_filter(rng, depth + 1);
                    children.push(Filter::Or(vec![sub]));
                }
                Filter::Or(children)
            }
            2 => {
                let inner = generate_random_filter(rng, depth + 1);
                if rng.next_bool() {
                    // Double not: Not(Not(inner))
                    Filter::Not(Box::new(Filter::Not(Box::new(inner))))
                } else {
                    Filter::Not(Box::new(inner))
                }
            }
            _ => generate_random_leaf_filter(rng),
        }
    }
}

fn generate_random_leaf_filter(rng: &mut XorShift64) -> Filter {
    match rng.next_range(0, 9) {
        0 => Filter::IsNull(match rng.next_range(0, 3) {
            0 => "c_int".into(),
            1 => "c_float".into(),
            2 => "c_bool".into(),
            _ => "c_dict".into(),
        }),
        1 => Filter::Eq("c_int".into(), Owned::Int(rng.next_range(0, 100) as i64)),
        2 => Filter::Gt("c_int".into(), Owned::Int(rng.next_range(0, 50) as i64)),
        3 => Filter::Between(
            "c_int".into(),
            Owned::Int(rng.next_range(0, 30) as i64),
            Owned::Int(rng.next_range(30, 80) as i64),
        ),
        4 => {
            // In list with duplicates, including floats with 0.0 and -0.0 and NaN
            if rng.next_bool() {
                let mut values = vec![
                    Owned::Float(0.0),
                    Owned::Float(-0.0),
                    Owned::Float(rng.next_f64()),
                    Owned::Float(0.0),
                ];
                if rng.next_bool() {
                    values.push(Owned::Float(f64::NAN));
                    values.push(Owned::Float(-f64::NAN));
                }
                Filter::In("c_float".into(), values)
            } else {
                let v = rng.next_range(0, 10) as i64;
                Filter::In(
                    "c_int".into(),
                    vec![Owned::Int(v), Owned::Int(v + 1), Owned::Int(v)],
                )
            }
        }
        5 => Filter::Lt("c_float".into(), Owned::Float(rng.next_f64())),
        6 => Filter::Eq("c_bool".into(), Owned::Bool(rng.next_bool())),
        7 => Filter::Contains {
            column: "c_dict".into(),
            text: "cat".into(),
            ignore_case: rng.next_bool(),
        },
        _ => Filter::StartsWith {
            column: "c_dict".into(),
            text: "cat_1".into(),
            ignore_case: rng.next_bool(),
        },
    }
}

fn generate_random_query(rng: &mut XorShift64) -> Query {
    let filter = if rng.next_range(0, 5) > 0 {
        Some(generate_random_filter(rng, 0))
    } else {
        None
    };

    // Sort keys with repeated columns
    let candidate_cols = ["c_int", "c_float", "c_dict", "c_bool"];
    let num_sort = rng.next_range(1, 4) as usize;
    let mut sort = Vec::with_capacity(num_sort);
    for _ in 0..num_sort {
        let col = candidate_cols[rng.next_range(0, 3) as usize];
        let order = if rng.next_bool() {
            Order::Asc
        } else {
            Order::Desc
        };
        let nulls = if rng.next_bool() {
            Nulls::First
        } else {
            Nulls::Last
        };
        sort.push(SortKey {
            column: col.into(),
            order,
            nulls,
        });
    }

    // Group levels (from groupable columns) with totals
    let groupable = ["c_dict", "c_bool", "c_int", "c_float"];
    let num_levels = rng.next_range(0, 2) as usize;
    let mut group_by = Vec::with_capacity(num_levels);
    for &col in groupable.iter().take(num_levels) {
        let mut totals = Vec::new();
        if rng.next_bool() {
            totals.push(Total {
                column: "c_int".into(),
                aggregate: Aggregate::Count,
            });
        }
        if rng.next_bool() {
            totals.push(Total {
                column: "c_int".into(),
                aggregate: Aggregate::Sum,
            });
        }
        group_by.push(GroupLevel {
            column: col.into(),
            totals,
        });
    }

    Query {
        filter,
        sort,
        group_by,
        summary: Vec::new(),
    }
}

#[cfg(feature = "serde")]
fn filter_has_nan(f: &Filter) -> bool {
    match f {
        Filter::And(list) | Filter::Or(list) => list.iter().any(filter_has_nan),
        Filter::Not(inner) => filter_has_nan(inner),
        Filter::Eq(_, v)
        | Filter::Lt(_, v)
        | Filter::Le(_, v)
        | Filter::Gt(_, v)
        | Filter::Ge(_, v) => matches!(v, Owned::Float(f) if f.is_nan()),
        Filter::Between(_, lo, hi) => {
            matches!(lo, Owned::Float(f) if f.is_nan())
                || matches!(hi, Owned::Float(f) if f.is_nan())
        }
        Filter::In(_, list) => list
            .iter()
            .any(|v| matches!(v, Owned::Float(f) if f.is_nan())),
        _ => false,
    }
}

#[cfg(feature = "serde")]
fn query_has_nan(q: &Query) -> bool {
    q.filter.as_ref().is_some_and(filter_has_nan)
}

#[cfg(feature = "serde")]
fn generate_random_finite_query(rng: &mut XorShift64) -> Query {
    let filter = if rng.next_bool() {
        Some(Filter::And(vec![
            Filter::Gt("c_float".into(), Owned::Float(rng.next_finite_f64())),
            Filter::In(
                "c_float".into(),
                vec![
                    Owned::Float(0.0),
                    Owned::Float(-0.0),
                    Owned::Float(rng.next_finite_f64()),
                ],
            ),
        ]))
    } else {
        Some(Filter::Eq(
            "c_int".into(),
            Owned::Int(rng.next_range(0, 50) as i64),
        ))
    };

    let sort = vec![
        SortKey {
            column: "c_int".into(),
            order: Order::Asc,
            nulls: Nulls::Last,
        },
        SortKey {
            column: "c_float".into(),
            order: Order::Desc,
            nulls: Nulls::First,
        },
    ];

    let group_by = vec![GroupLevel {
        column: "c_dict".into(),
        totals: vec![Total::count("c_int"), Total::sum("c_int")],
    }];

    Query {
        filter,
        sort,
        group_by,
        summary: Vec::new(),
    }
}

#[test]
fn test_property_normalization_hashing_and_view_equivalence_200_seeds() {
    for seed in 0..200 {
        let mut rng = XorShift64::new(seed + 1);
        let store = generate_random_store(&mut rng, 40);
        let q = generate_random_query(&mut rng);

        // 1. Idempotence: q.normalized().normalized() == q.normalized()
        let q_norm = q.normalized();
        let q_norm_twice = q_norm.normalized();
        assert_eq!(
            q_norm, q_norm_twice,
            "q.normalized() must be idempotent for seed {}",
            seed
        );

        // 2. Equal queries hash equal (clone + normalize both)
        let q_clone_norm = q.clone().normalized();
        assert_eq!(q_norm, q_clone_norm);
        assert_eq!(
            hash_val(&q_norm),
            hash_val(&q_clone_norm),
            "equal normalized queries must hash equal for seed {}",
            seed
        );

        // 3. View on store produces identical rows and group tree
        let view_orig = View::new(&store, q.clone()).expect("view_orig must succeed");
        let view_norm = View::new(&store, q_norm).expect("view_norm must succeed");

        assert_eq!(
            view_orig.rows(),
            view_norm.rows(),
            "filtered and sorted rows must match for seed {}",
            seed
        );
        assert_eq!(
            view_orig.row_count(),
            view_norm.row_count(),
            "row_count must match for seed {}",
            seed
        );
        assert_group_trees_equal(view_orig.groups(), view_norm.groups());

        // 4. Serde round-trip on queries
        #[cfg(feature = "serde")]
        {
            if !query_has_nan(&q) {
                let json = serde_json::to_string(&q).expect("serialize query to json");
                let deserialized: Query =
                    serde_json::from_str(&json).expect("deserialize query from json");
                assert_eq!(
                    deserialized, q,
                    "serde round-trip mismatch for seed {}",
                    seed
                );
                assert_eq!(hash_val(&deserialized), hash_val(&q));
            }

            // Also test on guaranteed finite query
            let finite_q = generate_random_finite_query(&mut rng);
            let json = serde_json::to_string(&finite_q).expect("serialize finite query to json");
            let deserialized: Query =
                serde_json::from_str(&json).expect("deserialize finite query from json");
            assert_eq!(deserialized, finite_q);
            assert_eq!(hash_val(&deserialized), hash_val(&finite_q));
        }
    }
}
