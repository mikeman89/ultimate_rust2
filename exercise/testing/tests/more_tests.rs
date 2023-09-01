use testing::splish;
use testing::sploosh;

#[test]
fn test_sploosh_splish_return_4() {
    assert_eq!(sploosh(splish(-1, 0), splish(1, 1), splish(3, 2)), 4)
}
