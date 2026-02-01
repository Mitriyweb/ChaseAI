use app::instruction::context::InstructionContext;

#[test]
fn test_valid_context_creation() {
    let context = InstructionContext::new(
        "WinSF".to_string(),
        "execution-agent".to_string(),
        "Do work".to_string(),
        vec!["analyze".to_string(), "execute-safe".to_string()],
        true,
    );
    assert!(context.is_ok());
}

#[test]
fn test_empty_system_fails() {
    let context = InstructionContext::new(
        "".to_string(),
        "role".to_string(),
        "inst".to_string(),
        vec!["action".to_string()],
        false,
    );
    assert!(context.is_err());
}

#[test]
fn test_empty_role_fails() {
    let context = InstructionContext::new(
        "sys".to_string(),
        "  ".to_string(),
        "inst".to_string(),
        vec!["action".to_string()],
        false,
    );
    assert!(context.is_err());
}

#[test]
fn test_empty_instruction_fails() {
    let context = InstructionContext::new(
        "sys".to_string(),
        "role".to_string(),
        "".to_string(),
        vec!["action".to_string()],
        false,
    );
    assert!(context.is_err());
}

#[test]
fn test_empty_actions_fails() {
    let context = InstructionContext::new(
        "sys".to_string(),
        "role".to_string(),
        "inst".to_string(),
        vec![],
        false,
    );
    assert!(context.is_err());
}

#[test]
fn test_invalid_action_format_fails() {
    let context = InstructionContext::new(
        "sys".to_string(),
        "role".to_string(),
        "inst".to_string(),
        vec!["InvalidAction".to_string()], // Uppercase not allowed
        false,
    );
    assert!(context.is_err());

    let context = InstructionContext::new(
        "sys".to_string(),
        "role".to_string(),
        "inst".to_string(),
        vec!["1action".to_string()], // Cannot start with number
        false,
    );
    assert!(context.is_err());
}
