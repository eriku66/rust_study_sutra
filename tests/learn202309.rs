#[test]
fn compare_option() {
    let a = Some(3);
    let b = Some(4);
    let c = None;

    // Option implements Ord if the element type implements Ord.
    // Some is greater than None.
    assert_eq!(a.max(b), Some(4));
    assert_eq!(a.max(c), Some(3));
    assert!(c.max(None).is_none());
}

#[test]
fn and_then() {
    let a = Some(3);
    let b = None;

    // and_then() return Option<T>.
    // If and_then() receives None, it returns None.
    assert_eq!(a.max(b).and_then(|n| Some(n * 3)), Some(9));
    assert_eq!(b.and_then(|n| Some(n * 3)), None);
}
