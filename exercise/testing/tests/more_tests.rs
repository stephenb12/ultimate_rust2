use testing::{splish, sploosh};

#[test]
fn time_to_integrate() {
    // - that `sploosh(splish(-1, 0), splish(1, 1), splish(3, 2))` returns the value `4`
    assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4);
}
