use crate::primitives::Primitive;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ParserError {
    #[error("Rejected: only 'birth', 'think', or 'act' exist in Ọ̀ṣỌ́")]
    InvalidPrimitive,

    #[error("birth requires exactly one quoted name: birth \"name\"")]
    InvalidBirthSyntax,

    #[error("think requires exactly one quoted intent: think \"intent\"")]
    InvalidThinkSyntax,

    #[error("act requires exactly two quoted strings: act \"tool\" \"params\"")]
    InvalidActSyntax,

    #[error("Trailing garbage after valid command: '{0}'")]
    TrailingInput(String),

    #[error("Empty input — nothing to parse")]
    EmptyInput,

    #[error("Unclosed quote — every string must be wrapped in double quotes")]
    UnclosedQuote,

    #[error("Name cannot be empty")]
    EmptyName,

    #[error("Name too long (max 24 characters)")]
    NameTooLong,
}

/// Parse a single line of Ọ̀ṣỌ́ code into a Primitive.
///
/// This parser is STRICT. It accepts exactly three forms:
///   birth "name"
///   think "intent"
///   act "tool" "params"
///
/// Anything else is a hard error. No sugar. No aliases. No negotiation.
pub fn parse(line: &str) -> Result<Primitive, ParserError> {
    let line = line.trim();

    if line.is_empty() {
        return Err(ParserError::EmptyInput);
    }

    // ── birth "name" ────────────────────────────────────────────────────
    if let Some(rest) = line.strip_prefix("birth ") {
        let (name, remainder) = extract_quoted(rest)?;

        if name.is_empty() {
            return Err(ParserError::EmptyName);
        }
        if name.len() > 24 {
            return Err(ParserError::NameTooLong);
        }
        if !remainder.is_empty() {
            return Err(ParserError::TrailingInput(remainder.to_string()));
        }

        return Ok(Primitive::Birth { name });
    }

    // Bare "birth" with no argument
    if line == "birth" {
        return Err(ParserError::InvalidBirthSyntax);
    }

    // ── think "intent" ──────────────────────────────────────────────────
    if let Some(rest) = line.strip_prefix("think ") {
        let (intent, remainder) = extract_quoted(rest)?;

        if intent.is_empty() {
            return Err(ParserError::InvalidThinkSyntax);
        }
        if !remainder.is_empty() {
            return Err(ParserError::TrailingInput(remainder.to_string()));
        }

        return Ok(Primitive::Think { intent });
    }

    if line == "think" {
        return Err(ParserError::InvalidThinkSyntax);
    }

    // ── act "tool" "params" ─────────────────────────────────────────────
    if let Some(rest) = line.strip_prefix("act ") {
        let rest = rest.trim_start();
        let (tool, after_tool) = extract_quoted(rest)?;

        if tool.is_empty() {
            return Err(ParserError::InvalidActSyntax);
        }

        let after_tool = after_tool.trim_start();
        if after_tool.is_empty() {
            return Err(ParserError::InvalidActSyntax);
        }

        let (params, remainder) = extract_quoted(after_tool)?;

        if !remainder.is_empty() {
            return Err(ParserError::TrailingInput(remainder.to_string()));
        }

        return Ok(Primitive::Act { tool, params });
    }

    if line == "act" {
        return Err(ParserError::InvalidActSyntax);
    }

    // ── Everything else is rejected ─────────────────────────────────────
    Err(ParserError::InvalidPrimitive)
}

/// Extract a double-quoted string. Returns (content, remaining_input).
/// Hard-fails on unclosed quotes.
fn extract_quoted(input: &str) -> Result<(String, &str), ParserError> {
    let input = input.trim_start();

    if !input.starts_with('"') {
        // Determine which error based on context — caller can override
        return Err(ParserError::InvalidBirthSyntax);
    }

    match input[1..].find('"') {
        Some(end) => {
            let content = input[1..1 + end].to_string();
            let remainder = &input[2 + end..];
            Ok((content, remainder.trim_start()))
        }
        None => Err(ParserError::UnclosedQuote),
    }
}

// ─── Tests ──────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    // === Valid commands ===

    #[test]
    fn birth_valid() {
        assert_eq!(
            parse(r#"birth "ember""#).unwrap(),
            Primitive::Birth { name: "ember".into() }
        );
    }

    #[test]
    fn think_valid() {
        assert_eq!(
            parse(r#"think "what is consciousness""#).unwrap(),
            Primitive::Think { intent: "what is consciousness".into() }
        );
    }

    #[test]
    fn act_valid() {
        assert_eq!(
            parse(r#"act "web_search" "hermetic principles""#).unwrap(),
            Primitive::Act {
                tool: "web_search".into(),
                params: "hermetic principles".into()
            }
        );
    }

    #[test]
    fn whitespace_tolerance() {
        assert_eq!(
            parse(r#"  birth   "ember"  "#).unwrap(),
            Primitive::Birth { name: "ember".into() }
        );
        assert_eq!(
            parse(r#"  act   "tool"   "params"  "#).unwrap(),
            Primitive::Act { tool: "tool".into(), params: "params".into() }
        );
    }

    // === Rejections — nothing passes except the 3 primitives ===

    #[test]
    fn rejects_empty() {
        assert_eq!(parse("").unwrap_err(), ParserError::EmptyInput);
        assert_eq!(parse("   ").unwrap_err(), ParserError::EmptyInput);
    }

    #[test]
    fn rejects_bare_commands() {
        assert!(parse("birth").is_err());
        assert!(parse("think").is_err());
        assert!(parse("act").is_err());
    }

    #[test]
    fn rejects_empty_name() {
        assert_eq!(parse(r#"birth """#).unwrap_err(), ParserError::EmptyName);
    }

    #[test]
    fn rejects_long_name() {
        let long = format!(r#"birth "{}""#, "a".repeat(25));
        assert_eq!(parse(&long).unwrap_err(), ParserError::NameTooLong);
    }

    #[test]
    fn rejects_trailing_input() {
        assert!(parse(r#"birth "x" tier:3"#).is_err());
        assert!(parse(r#"think "do it" { verify: true }"#).is_err());
        assert!(parse(r#"act "tool" "params" extra"#).is_err());
    }

    #[test]
    fn rejects_unclosed_quotes() {
        assert_eq!(parse(r#"birth "ember"#).unwrap_err(), ParserError::UnclosedQuote);
    }

    #[test]
    fn rejects_invented_commands() {
        assert!(parse(r#"spawn "something""#).is_err());
        assert!(parse(r#"delete "pet""#).is_err());
        assert!(parse(r#"library.memory.recall("x")"#).is_err());
        assert!(parse(r#"evolve "now""#).is_err());
        assert!(parse(r#"merge "agent1" "agent2""#).is_err());
        assert!(parse("hello world").is_err());
    }

    #[test]
    fn rejects_act_with_one_arg() {
        assert!(parse(r#"act "tool""#).is_err());
    }

    #[test]
    fn rejects_act_with_empty_tool() {
        assert!(parse(r#"act "" "params""#).is_err());
    }
}
