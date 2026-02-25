use serde_json::Value;

/// Strict protocol output:
/// - {"message":{"content":"..."}}
/// - {"tool_call":{"tool":"...","args":{...}}}
#[derive(Debug, Clone)]
pub enum Output {
    Message { content: String },
    ToolCall { tool: String, args: Value },
}

pub fn parse_strict(s: &str) -> anyhow::Result<Output> {
    let v: Value = serde_json::from_str(s)
        .map_err(|e| anyhow::anyhow!("invalid json: {e}"))?;

    let has_msg = v.get("message").is_some();
    let has_tool = v.get("tool_call").is_some();
    if has_msg == has_tool {
        return Err(anyhow::anyhow!("must contain exactly one of message or tool_call"));
    }

    if let Some(m) = v.get("message") {
        let content = m.get("content").and_then(|x| x.as_str()).ok_or_else(|| anyhow::anyhow!("message.content missing"))?;
        return Ok(Output::Message { content: content.to_string() });
    }

    let tc = v.get("tool_call").ok_or_else(|| anyhow::anyhow!("tool_call missing"))?;
    let tool = tc.get("tool").and_then(|x| x.as_str()).ok_or_else(|| anyhow::anyhow!("tool_call.tool missing"))?;
    let args = tc.get("args").cloned().unwrap_or(Value::Object(Default::default()));
    if tool.trim().is_empty() || tool.len() > 64 {
        return Err(anyhow::anyhow!("invalid tool name"));
    }
    Ok(Output::ToolCall { tool: tool.to_string(), args })
}
