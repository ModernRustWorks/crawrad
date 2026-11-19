use crate::Collection;

#[test]
fn collection_starts_empty() {
    let col: Collection<String> = Collection::new();
    assert_eq!(col.len(), 0);
    assert!(col.is_empty());
    assert_eq!(col.get(0), None);
}

#[test]
fn collection_push_and_indexing() {
    let mut col = Collection::new();
    col.push(10);
    col.push(20);
    col.push(30);

    assert_eq!(col.len(), 3);
    assert!(!col.is_empty());
    assert_eq!(col[0], 10);
    assert_eq!(col[1], 20);
    assert_eq!(col[2], 30);
    assert_eq!(col.get(1), Some(&20));
    assert_eq!(col.get(3), None);
}

#[test]
fn collection_mutation_and_deref() {
    let mut col = Collection::new();
    col.push("alpha".to_string());
    col.push("beta".to_string());

    col[0] = "alpha_modified".to_string();
    assert_eq!(col[0], "alpha_modified");

    if let Some(second) = col.get_mut(1) {
        second.push_str("_mutated");
    }
    assert_eq!(col[1], "beta_mutated");
}

#[test]
fn collection_iteration_and_conversions() {
    let initial = vec![1, 2, 3];
    let mut col = Collection::from(initial);

    // Iter
    let sum: i32 = col.iter().sum();
    assert_eq!(sum, 6);

    // Iter mut
    for item in &mut col {
        *item *= 2;
    }
    assert_eq!(col.as_slice(), &[2, 4, 6]);

    // IntoIterator
    let collected: Vec<i32> = col.into_iter().collect();
    assert_eq!(collected, vec![2, 4, 6]);
}

#[test]
fn collection_remove_and_clear() {
    let mut col = Collection::with_items(vec!["a", "b", "c"]);
    let removed = col.remove(1);
    assert_eq!(removed, "b");
    assert_eq!(col.len(), 2);
    assert_eq!(col[0], "a");
    assert_eq!(col[1], "c");

    col.clear();
    assert!(col.is_empty());
}
