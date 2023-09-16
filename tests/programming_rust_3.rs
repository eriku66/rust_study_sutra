#[test]
fn checked_operation() {
    // Checked operations return None if the result would overflow.
    assert_eq!(100_u8.checked_add(155), Some(255));
    assert_eq!(100_u8.checked_add(156), None);

    // The signed number divide operations also return None if overflow
    assert_eq!((-100_i8).checked_div(5), Some(-20));
    assert_eq!((-127_i8).checked_div(-1), Some(127));
    assert_eq!((-128_i8).checked_div(-1), None);
}

#[test]
fn wrapping_operation() {
    // In release builds, wrapping operations are standard operations.
    // Wrapping operations return the value wrapped around at the boundary of the type.
    assert_eq!(100_u8.wrapping_add(155), 255);
    assert_eq!(100_u8.wrapping_add(156), 0);

    // If signed number overflow, the sign of the result may be reversed.
    assert_eq!(100_i8.wrapping_mul(5), -12);

    // shl is bit shift left
    // A bit shift left operation is equivalent to multiplying by 2.
    assert_eq!(5_i16.wrapping_shl(2), 20);
    assert_eq!(5_i16.wrapping_shl(17), 10);
}

#[test]
fn saturating_operation() {
    // Saturating operations saturate at the boundary of the type.
    assert_eq!(100_u8.saturating_add(155), 255);
    assert_eq!(100_u8.saturating_add(156), 255);

    // If signed number overflow, the result is the saturated value.
    assert_eq!(100_i8.saturating_mul(5), 127);
    assert_eq!((-100_i8).saturating_mul(5), -128);
}

#[test]
fn overflowing_operation() {
    // Overflowing operations return a tuple of the result and a bool.
    assert_eq!(100_u8.overflowing_add(155), (255, false));
    assert_eq!(100_u8.overflowing_add(156), (0, true));

    // A shift of 17 bits is too large for 'u16', so the result is wrapped.
    assert_eq!(6_u16.overflowing_shl(18), (24, true));
}
