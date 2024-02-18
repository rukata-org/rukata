use p00001_basic_addition::add;

#[test]
fn basic_test() {
    let result = add(2, 2);
    assert_eq!(result, 4);
}
