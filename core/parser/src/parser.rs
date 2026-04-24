use crate::primitives::Primitive;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParserError {
    #[error("Invalid primitive: expected 'birth', 'think', or 'act'")]
    InvalidPrimitive,
    #[error("Invalid syntax: birth expects one quoted string")]
    InvalidBirthSyntax,
    #[error("Invalid syntax: think expects one quoted string")]
    InvalidThinkSyntax,
    #[error("Invalid syntax: act expects two quoted strings")]
    InvalidActSyntax,
    #[error("Trailing input after valid primitive: {0}")]
    TrailingInput(String),
}

/// Parse a single line of Ọ̀ṣỌ́ code into a Primitive.
/// Rejects anything that is not one of the three commands.
pub fn parse(line: &str) -> Result<Primitive, ParserError> {
    let line = line.trim();

    if let Some(rest) = line.strip_prefix("birth ") {
        let (name, remainder) = extract_quoted(rest).map_err(|_| ParserError::InvalidBirthSyntax)?;
        if !remainder.is_empty() {
            return Err(ParserError::TrailingInput(remainder.to_string()));
        }
        return Ok(Primitive::Birth { name });
    }

    if let Some(rest) = line.strip_prefix("think ") {
        let (intent, remainder) =
            extract_quoted(rest).map_err(|_| ParserError::InvalidThinkSyntax)?;
        if !remainder.is_empty() {
            return Err(ParserError::TrailingInput(remainder.to_string()));
        }
        return Ok(Primitive::Think { intent });
    }

    if let Some(rest) = line.strip_prefix("act ") {
        let rest = rest.trim_start();
        let (tool, remainder) = extract_quoted(rest).map_err(|_| ParserError::InvalidActSyntax)?;
        let remainder = remainder.trim_start();
        let (params, final_remainder) =
            extract_quoted(remainder).map_err(|_| ParserError::InvalidActSyntax)?;
        if !final_remainder.is_empty() {
            return Err(ParserError::TrailingInput(final_remainder.to_string()));
        }
        return Ok(Primitive::Act { tool, params });
    }

    Err(ParserError::InvalidPrimitive)
}

/// Extract a double-quoted string and return (content, remaining_input).
fn extract_quoted(input: &str) -> Result<(String, &str), ()> {
    let input = input.trim_start();
    if !input.starts_with('"') {
        return Err(());
    }
    let end = input[1..].find('"').ok_or(())?;
    let content = input[1..1 + end].to_string();
    let remainder = &input[2 + end..];
    Ok((content, remainder.trim_start()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_birth() {
        let result = parse(r#"birth "ember""#).unwrap();
        assert_eq!(result, Primitive::Birth { name: "ember".into() });
    }

    #[test]
    fn test_think() {
        let result = parse(r#"think "I want to learn about stars""#).unwrap();
        assert_eq!(
            result,
            Primitive::Think {
                intent: "I want to learn about stars".into()
            }
        );
    }

    #[test]
    fn test_act() {
        let result = parse(r#"act "web_search" "constellations 2026""#).unwrap();
        assert_eq!(
            result,
            Primitive::Act {
                tool: "web_search".into(),
                params: "constellations 2026".into()
            }
        );
    }

    #[test]
    fn test_rejects_trailing_input() {
        assert!(parse(r#"birth "x" tier:3"#).is_err());
        assert!(parse(r#"think "do it" { verify: true }"#).is_err());
    }

    #[test]
    fn test_rejects_invalid_primitives() {
        assert!(parse(r#"library.memory.recall("x")"#).is_err());
        assert!(parse(r#"spawn "something""#).is_err());
        assert!(parse(r#"delete "pet""#).is_err());
    }

    #[test]
    fn test_whitespace_handling() {
        let result = parse(r#"  birth   "ember"  "#).unwrap();
        assert_eq!(result, Primitive::Birth { name: "ember".into() });
    }
}
