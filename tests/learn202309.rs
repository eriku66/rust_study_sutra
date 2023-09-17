#[test]
fn compare_option() {
    let a = Some(3);
    let b = Some(4);
    let c = None;

    // Option implements Ord if the element type implements Ord.
    // Some is greater than None.
    assert_eq!(a.max(b), Some(4));
    assert!(a.max(c).is_some());
    assert!(c.max(None).is_some());
}
