#[test]
fn learn_skip_and_skip_while() {
    let vec = vec![1, 2, 3, 10, 5];

    // skip() skip the first n elements.
    assert_eq!(vec.iter().skip(2).collect::<Vec<_>>(), vec![&3, &10, &5]);

    // skip_while() skip the elements until the predicate returns true.
    // skip_while() references the elements.
    assert_eq!(
        vec.iter().skip_while(|&&n| n < 10).collect::<Vec<_>>(),
        vec![&10, &5]
    );
}
