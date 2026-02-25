#[test]
fn protocol_rejects_ambiguous() {
    let s = r#"{"message":{"content":"x"},"tool_call":{"tool":"shell","args":{}}}"#;
    assert!(femtoclaw_bench::protocol::parse_strict(s).is_err());
}
