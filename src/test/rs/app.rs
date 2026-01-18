#[test]
fn test_core_library_access() {
    let response = app::greet("User");
    assert!(response.contains("Hello, User"));
}

#[test]
fn test_version_return() {
    let v = app::version();
    assert!(!v.is_empty());
}
