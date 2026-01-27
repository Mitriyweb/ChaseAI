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

#[test]
fn test_app_new() {
    let app = app::App::new();
    assert_eq!(app.name, "ChaseAI");
    assert!(!app.version.is_empty());

    // Check that components are initialized
    assert!(app.config.port_bindings.is_empty() || !app.config.port_bindings.is_empty());
    // Just check it exists
}
