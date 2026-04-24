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
///
/// IMPORTANT: `think` is the default. Most input becomes a thought.
/// `act` is only produced when the user gives an explicit, unambiguous
/// command to execute a tool — like "go search the web for X" or
/// "generate an image of Y". Researching, wondering, asking questions,
/// and casual conversation are ALL `think`.
pub fn translate(input: &str) -> Option<TranslationResult> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return None;
    }

    let lower = trimmed.to_lowercase();

    // ── Birth detection ────────────────────────────────────────────
    if let Some(result) = try_birth(&lower, trimmed) {
        return Some(result);
    }

    // ── Act detection ──────────────────────────────────────────────
    // ONLY matches explicit tool commands. Anything that could be
    // interpreted as thinking, researching, or asking is kept as think.
    if let Some(result) = try_act(&lower, trimmed) {
        return Some(result);
    }

    // ── Think (default) ────────────────────────────────────────────
    // Everything else is a thought — chatting, asking, wondering,
    // researching, learning. This is the vast majority of input.
    Some(TranslationResult {
        primitive: Primitive::Think { intent: trimmed.to_string() },
        original: trimmed.to_string(),
        confidence: 0.8,
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
///
/// This is intentionally conservative. Only explicit, unambiguous
/// commands to use a tool match here. Research-style language like
/// "search for blockchain" or "look up crypto" is NOT an act — it's
/// a thought. The user is telling the agent what they're interested in,
/// not commanding it to execute a web search.
///
/// Act only matches when the user clearly wants the agent to DO something:
///   "go search the web for X"  — explicit tool command
///   "generate an image of Y"   — explicit generation command
///   "execute a web search"     — explicit execution language
fn try_act(lower: &str, original: &str) -> Option<TranslationResult> {
    // Explicit tool execution patterns only.
    // These require command-style language, not casual/research language.
    let tool_patterns: &[(&[&str], &str)] = &[
        // Web search — only explicit "use the tool" language
        (&["go search the web", "execute a web search", "use web search",
           "run a web search", "do a web search"],
         "web_search"),
        // Image generation — explicit generation commands
        (&["generate an image", "create an image", "make an image",
           "generate a picture", "make a picture"],
         "image_gen"),
        // Video generation
        (&["generate a video", "create a video", "make a video"],
         "video_gen"),
        // Code generation
        (&["write code for", "generate code for", "code me a",
           "program a", "build a script for"],
         "code_gen"),
        // Content creation
        (&["write a post about", "create content about",
           "write content about", "draft a post"],
         "content_create"),
        // Automation
        (&["run a script", "execute a script", "automate this",
           "schedule a task"],
         "automation"),
        // Browser
        (&["browse to", "open the page", "go to the website",
           "navigate to"],
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
                    confidence: 0.9,
                });
            }
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

    // ── Act (only explicit tool commands) ──

    #[test]
    fn detects_explicit_web_search() {
        let r = translate("go search the web for bitcoin prices").unwrap();
        assert!(matches!(r.primitive, Primitive::Act { ref tool, .. } if tool == "web_search"));
        assert!(r.confidence >= 0.9);
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

    // ── Think (default for almost everything) ──

    #[test]
    fn unknown_input_becomes_think() {
        let r = translate("what is the meaning of life").unwrap();
        assert!(matches!(r.primitive, Primitive::Think { ref intent } if intent == "what is the meaning of life"));
    }

    #[test]
    fn casual_chat_becomes_think() {
        let r = translate("I'm curious about blockchain").unwrap();
        assert!(matches!(r.primitive, Primitive::Think { .. }));
    }

    #[test]
    fn search_for_is_think_not_act() {
        // "search for X" is research language, not a tool command
        let r = translate("search for blockchain").unwrap();
        assert!(matches!(r.primitive, Primitive::Think { .. }));
    }

    #[test]
    fn research_is_think() {
        let r = translate("research crypto trends").unwrap();
        assert!(matches!(r.primitive, Primitive::Think { .. }));
    }

    #[test]
    fn tell_me_about_is_think() {
        let r = translate("tell me about Bitcoin").unwrap();
        assert!(matches!(r.primitive, Primitive::Think { .. }));
    }

    #[test]
    fn look_up_is_think() {
        let r = translate("look up the latest news").unwrap();
        assert!(matches!(r.primitive, Primitive::Think { .. }));
    }

    // ── Edge cases ──

    #[test]
    fn create_image_is_act_not_birth() {
        // "create an image" has explicit generation language
        let r = translate("create an image of a cat").unwrap();
        assert!(matches!(r.primitive, Primitive::Act { ref tool, .. } if tool == "image_gen"));
    }

    #[test]
    fn create_agent_is_birth_not_act() {
        let r = translate("create an agent called Blaze").unwrap();
        assert!(matches!(r.primitive, Primitive::Birth { ref name } if name == "Blaze"));
    }
}
