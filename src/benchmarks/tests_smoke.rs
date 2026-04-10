use crate::protocol;

#[test]
fn protocol_rejects_ambiguous() {
    let s = r#"{"message":{"content":"x"},"tool_call":{"tool":"shell","args":{}}}"#;
    assert!(protocol::parse_strict(s).is_err());
}
