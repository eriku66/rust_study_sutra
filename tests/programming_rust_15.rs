#[test]
fn take_and_take_while_15_3_4() {
    let vec = vec![1, 2, 3, 10, 5];

    // take() take the first n elements.
    assert_eq!(vec.iter().take(2).collect::<Vec<_>>(), vec![&1, &2]);

    // take_while() take the elements until the predicate returns false.
    // take_while() references the elements.
    assert_eq!(
        vec.iter().take_while(|&&n| n < 10).collect::<Vec<_>>(),
        vec![&1, &2, &3]
    );
}

// skip() is reverse of take().
#[test]
fn skip_and_skip_while_15_3_5() {
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
