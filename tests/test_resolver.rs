use expr_solver::resolve;

#[test]
fn basic_resolution() {
    assert_eq!(resolve("3!".to_string()), Ok(6.0));
    assert_eq!(resolve("-3!".to_string()), Ok(-6.0));
    assert_eq!(resolve("3.2!".to_string()), Ok(7.756592718904097));
    assert_eq!(resolve("3--3!".to_string()), Ok(9.0));
    assert_eq!(resolve("-2".to_string()), Ok(-2.0));
    assert_eq!(resolve("2--2".to_string()), Ok(4.0));
    assert_eq!(resolve("2+2*2*2*2+2".to_string()), Ok(20.0));
    assert_eq!(resolve("-2/2+2*2".to_string()), Ok(3.0));
}
