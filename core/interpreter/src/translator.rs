use oso_parser::Primitive;

/// Result of translating natural language into a primitive.
#[derive(Debug, Clone, PartialEq)]
pub struct TranslationResult {
    pub primitive: Primitive,
    /// The original natural language input.
    pub original: String,
    /// Confidence score (0.0 to 1.0) — how certain the match was.
    pub confidence: f64,
}

/// Translate natural language into an Ọ̀ṣỌ́ primitive.
///
/// This is a lightweight keyword-based translator for local use.
/// The Claude-powered translator on the API side handles ambiguous input;
/// this handles the common, obvious cases without an API call.
pub fn translate(input: &str) -> Option<TranslationResult> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let lower = trimmed.to_lowercase();

    // ── Birth detection ────────────────────────────────────────────
    // User wants to create/make/start a new agent
    if let Some(result) = try_birth(&lower, trimmed) {
        return Some(result);
    }

    // ── Act detection ──────────────────────────────────────────────
    // User wants the agent to DO something concrete (search, generate, etc.)
    // Check act BEFORE think — "search for X" is an action, not a thought.
    if let Some(result) = try_act(&lower, trimmed) {
        return Some(result);
    }

    // ── Think detection (fallback) ─────────────────────────────────
    // Everything else is a thought. Chatting, asking, wondering, etc.
    Some(TranslationResult {
        primitive: Primitive::Think { intent: trimmed.to_string() },
        original: trimmed.to_string(),
        confidence: 0.6,
    })
}

/// Try to detect a birth intent.
fn try_birth(lower: &str, original: &str) -> Option<TranslationResult> {
    let birth_signals = [
        "create a new agent",
        "create an agent",
        "make a new agent",
        "make an agent",
        "start a new agent",
        "birth a new",
        "spawn a new",
        "i want to create",
        "i want to make",
        "create a pet",
        "make a pet",
        "new pet named",
        "new agent named",
        "create one called",
        "make one called",
    ];

    for signal in &birth_signals {
        if lower.contains(signal) {
            let name = extract_name(lower, original);
            return Some(TranslationResult {
                primitive: Primitive::Birth { name },
                original: original.to_string(),
                confidence: 0.9,
            });
        }
    }

    // Direct "call it X" / "named X" patterns
    if (lower.starts_with("create ") || lower.starts_with("make "))
        && !contains_action_verb(lower)
    {
        let name = extract_name(lower, original);
        if !name.is_empty() {
            return Some(TranslationResult {
                primitive: Primitive::Birth { name },
                original: original.to_string(),
                confidence: 0.7,
            });
        }
    }

    None
}

/// Try to detect an act intent.
fn try_act(lower: &str, original: &str) -> Option<TranslationResult> {
    // Tool-specific patterns: (signal, tool_name)
    let tool_patterns: &[(&[&str], &str)] = &[
        (&["search for", "look up", "google", "find out about", "search the web"],
         "web_search"),
        (&["generate an image", "create an image", "make an image", "draw", "make a picture"],
         "image_gen"),
        (&["generate a video", "create a video", "make a video"],
         "video_gen"),
        (&["write code", "generate code", "code a", "program a", "build a script"],
         "code_gen"),
        (&["write a post", "create content", "write content", "draft a"],
         "content_create"),
        (&["automate", "run a script", "execute a script", "schedule"],
         "automation"),
        (&["browse to", "open the page", "go to the website", "navigate to"],
         "browser"),
    ];

    for (signals, tool) in tool_patterns {
        for signal in *signals {
            if lower.contains(signal) {
                let params = extract_params(lower, signal, original);
                return Some(TranslationResult {
                    primitive: Primitive::Act {
                        tool: tool.to_string(),
                        params,
                    },
                    original: original.to_string(),
                    confidence: 0.85,
                });
            }
        }
    }

    // Generic action verbs that suggest act (lower confidence)
    let generic_action_verbs = [
        "can you do", "please do", "go ahead and", "execute",
        "run", "perform", "carry out",
    ];
    for verb in &generic_action_verbs {
        if lower.contains(verb) {
            return Some(TranslationResult {
                primitive: Primitive::Act {
                    tool: "web_search".to_string(),
                    params: original.to_string(),
                },
                original: original.to_string(),
                confidence: 0.5,
            });
        }
    }

    None
}

/// Check if the input contains action verbs (to disambiguate "create an image" from "create an agent").
fn contains_action_verb(lower: &str) -> bool {
    let action_objects = [
        "image", "video", "code", "script", "content", "post",
        "song", "music", "art", "drawing", "picture",
    ];
    action_objects.iter().any(|obj| lower.contains(obj))
}

/// Extract a name from natural language input for birth.
fn extract_name(lower: &str, original: &str) -> String {
    // Try "named X" or "called X"
    for keyword in &["named ", "called ", "name it ", "call it "] {
        if let Some(pos) = lower.find(keyword) {
            let after = &original[pos + keyword.len()..];
            let name = after.trim()
                .trim_matches(|c: char| c == '"' || c == '\'' || c == '.' || c == '!')
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string();
            if !name.is_empty() {
                return name;
            }
        }
    }

    // Try last quoted word
    if let Some(start) = original.find('"') {
        if let Some(end) = original[start + 1..].find('"') {
            let quoted = &original[start + 1..start + 1 + end];
            if !quoted.is_empty() {
                return quoted.to_string();
            }
        }
    }

    // Fallback: use the last meaningful word
    let skip_words = [
        "create", "make", "new", "a", "an", "the", "agent", "pet",
        "one", "please", "i", "want", "to", "start",
    ];
    let last_word = original.split_whitespace()
        .rev()
        .find(|w| !skip_words.contains(&w.to_lowercase().as_str()))
        .unwrap_or("agent")
        .trim_matches(|c: char| !c.is_alphanumeric())
        .to_string();

    if last_word.is_empty() { "agent".to_string() } else { last_word }
}

/// Extract params from the input after the matched signal.
fn extract_params(lower: &str, signal: &str, original: &str) -> String {
    if let Some(pos) = lower.find(signal) {
        let after = &original[pos + signal.len()..];
        let params = after.trim()
            .trim_start_matches(|c: char| c == ':' || c == '-' || c == ' ');
        if !params.is_empty() {
            return params.to_string();
        }
    }
    original.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_returns_none() {
        assert!(translate("").is_none());
        assert!(translate("   ").is_none());
    }

    // ── Birth ──

    #[test]
    fn detects_birth_create_agent() {
        let r = translate("create a new agent named Ember").unwrap();
        assert!(matches!(r.primitive, Primitive::Birth { ref name } if name == "Ember"));
        assert!(r.confidence >= 0.7);
    }

    #[test]
    fn detects_birth_make_pet() {
        let r = translate("make a pet called Nova").unwrap();
        assert!(matches!(r.primitive, Primitive::Birth { ref name } if name == "Nova"));
    }

    #[test]
    fn detects_birth_with_quotes() {
        let r = translate("create an agent named \"spark\"").unwrap();
        assert!(matches!(r.primitive, Primitive::Birth { ref name } if name == "spark"));
    }

    // ── Act ──

    #[test]
    fn detects_web_search() {
        let r = translate("search for hermetic principles").unwrap();
        assert!(matches!(r.primitive, Primitive::Act { ref tool, .. } if tool == "web_search"));
        assert!(r.confidence >= 0.8);
    }

    #[test]
    fn detects_image_gen() {
        let r = translate("generate an image of a sunset").unwrap();
        assert!(matches!(r.primitive, Primitive::Act { ref tool, .. } if tool == "image_gen"));
    }

    #[test]
    fn detects_code_gen() {
        let r = translate("write code for a fibonacci function").unwrap();
        assert!(matches!(r.primitive, Primitive::Act { ref tool, .. } if tool == "code_gen"));
    }

    // ── Think (fallback) ──

    #[test]
    fn unknown_input_becomes_think() {
        let r = translate("what is the meaning of life").unwrap();
        assert!(matches!(r.primitive, Primitive::Think { ref intent } if intent == "what is the meaning of life"));
        assert!(r.confidence < 0.8);
    }

    #[test]
    fn casual_chat_becomes_think() {
        let r = translate("I'm curious about blockchain").unwrap();
        assert!(matches!(r.primitive, Primitive::Think { .. }));
    }

    // ── Edge cases ──

    #[test]
    fn create_image_is_act_not_birth() {
        let r = translate("create an image of a cat").unwrap();
        assert!(matches!(r.primitive, Primitive::Act { ref tool, .. } if tool == "image_gen"));
    }

    #[test]
    fn create_agent_is_birth_not_act() {
        let r = translate("create an agent called Blaze").unwrap();
        assert!(matches!(r.primitive, Primitive::Birth { ref name } if name == "Blaze"));
    }
}
