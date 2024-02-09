use expr_solver::resolve;

#[test]
fn basic_resolution() {
    assert_eq!(resolve("2".to_string()), Ok(2.0));
    assert_eq!(resolve("-2".to_string()), Ok(-2.0));
    assert_eq!(resolve("2--2".to_string()), Ok(4.0));
    assert_eq!(resolve("2+2*2*2*2+2".to_string()), Ok(20.0));
    assert_eq!(resolve("-2/2+2*2".to_string()), Ok(3.0));
}
