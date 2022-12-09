
use macros::composed;

#[test]
fn composed_macro_generates_valid_code() {
    let c0 = composed!(a: u8 => 0, b: u32 => 1);
    let c0_ref: &u8 = c0.as_ref();

    assert_eq!(*c0_ref, 0);
    assert_eq!(c0.a, 0);
    assert_eq!(c0.b, 1);

    let c1 = composed!(a: u16 => 2, b: u64 => 3);
    let c1_ref: &u16 = c1.as_ref();

    assert_eq!(*c1_ref, 2);
    assert_eq!(c1.a, 2);
    assert_eq!(c1.b, 3);
}