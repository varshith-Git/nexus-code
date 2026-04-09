use serde_json::Value;

/// Attempts to repair malformed JSON strings often produced by local LLMs
/// before giving up and returning an error.
pub fn repair_json(raw: &str) -> Result<Value, serde_json::Error> {
    // 1. First try parsing it as-is
    if let Ok(val) = serde_json::from_str::<Value>(raw) {
        return Ok(val);
    }
    
    let mut repaired = raw.trim().to_string();
    
    // 2. Strip markdown code block markers
    if repaired.starts_with("```json") {
        repaired = repaired[7..].trim_start().to_string();
    } else if repaired.starts_with("```") {
        repaired = repaired[3..].trim_start().to_string();
    }
    
    if repaired.ends_with("```") {
        repaired = repaired[..repaired.len()-3].trim_end().to_string();
    }
    
    // Attempt parse after strip
    if let Ok(val) = serde_json::from_str::<Value>(&repaired) {
        return Ok(val);
    }
    
    // 3. Fix trailing commas (common error)
    // Replace ", }" with "}" and ", ]" with "]"
    // This is a naive regex-free approach
    let mut without_trailing = String::with_capacity(repaired.len());
    let chars: Vec<char> = repaired.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i] == ',' {
            // look ahead for closing brace or bracket ignoring whitespace
            let mut j = i + 1;
            while j < chars.len() && chars[j].is_whitespace() {
                j += 1;
            }
            if j < chars.len() && (chars[j] == '}' || chars[j] == ']') {
                // skip the comma
                i = j;
                continue;
            }
        }
        without_trailing.push(chars[i]);
        i += 1;
    }
    
    if let Ok(val) = serde_json::from_str::<Value>(&without_trailing) {
        return Ok(val);
    }
    
    // 4. Fallback to parsing the original causing the actual error
    serde_json::from_str::<Value>(raw)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repair_json_trailing_comma() {
        let raw = r#"{ "name": "test", }"#;
        let val = repair_json(raw).unwrap();
        assert_eq!(val["name"], "test");
    }

    #[test]
    fn test_repair_json_markdown() {
        let raw = "```json\n{ \"name\": \"test\" }\n```";
        let val = repair_json(raw).unwrap();
        assert_eq!(val["name"], "test");
    }

    #[test]
    fn test_repair_json_combined() {
        let raw = "```\n{ \"name\": \"test\", \n }\n```";
        let val = repair_json(raw).unwrap();
        assert_eq!(val["name"], "test");
    }
}
