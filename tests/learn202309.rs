#[test]
fn learn_compare_option() {
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
fn learn_and_then() {
    let a: Option<i32> = Some(3);
    let b: Option<i32> = None;

    // and_then() return Option<T>.
    // If and_then() receives None, it returns None.
    assert_eq!(a.and_then(|n| Some(n * 3)), Some(9));
    assert_eq!(b.and_then(|n| Some(n * 3)), None);
}

#[test]
fn learn_cloned() {
    let a: Option<&i32> = Some(&3);
    let b: Option<&i32> = None;

    // cloned() make change the type of Option<&T> to Option<T>.
    // If cloned() receives None, it returns None.
    assert_eq!(a.cloned(), Some(3));
    assert_eq!(b.cloned(), None);
    assert_eq!(b, None);
}

#[test]
fn learn_flatten_used_for_option_iter() {
    let a = [Some(1), None, Some(2), None, Some(3)].iter();

    // flatten() make change the type of Option<Option<T>> to Option<T>.
    // flatten() removes None.
    assert_eq!(a.flatten().collect::<Vec<_>>(), vec![&1, &2, &3]);
}
