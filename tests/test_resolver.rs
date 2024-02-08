mod resolver_test {
    use expr_resolver::resolve;

    #[test]
    fn basic_resolution() {
        assert!(resolve("2--2".to_string()) == Ok(4.0));
        assert!(resolve("2+2*2*2*2+2".to_string()) == Ok(20.0));
    }
}
