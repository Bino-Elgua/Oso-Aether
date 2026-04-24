/// Tool tier gating system.
///
/// Each tool is locked behind a reputation threshold. The agent must
/// reach the required reputation before it can use a given tool.
/// Reputation tiers:
///   Tier 0 (0-20):   No tools (still in pre-evolution)
///   Tier 1 (21-40):  Web Search, Image Generation
///   Tier 2 (41-60):  Video Generation, Content Creation
///   Tier 3 (61-80):  Code Generation, Automation
///   Tier 4 (81-99):  Browser Control, Advanced Tools
///   Tier 5 (100+):   Sovereign — all tools unlocked

/// The minimum reputation required to use a specific tool.
pub fn tool_reputation_requirement(tool: &str) -> Option<u64> {
    match tool {
        // Tier 1 (21+)
        "web_search" => Some(21),
        "image_gen" => Some(21),

        // Tier 2 (41+)
        "video_gen" => Some(41),
        "content_create" => Some(41),

        // Tier 3 (61+)
        "code_gen" => Some(61),
        "automation" => Some(61),

        // Tier 4 (81+)
        "browser" => Some(81),
        "advanced_tools" => Some(81),

        // Unknown tool — not in the registry
        _ => None,
    }
}

/// Check if an agent with the given reputation can use a specific tool.
/// Returns Ok(()) if allowed, Err with a user-friendly message if not.
pub fn check_tool_access(tool: &str, reputation: u64) -> Result<(), String> {
    match tool_reputation_requirement(tool) {
        Some(required) if reputation >= required => Ok(()),
        Some(required) => {
            let tier = tool_tier_name(required);
            let needed = required.saturating_sub(reputation);
            Err(format!(
                "You don't have access to {tool} yet.\n\
                 \n\
                 {tool} unlocks at {required} reputation ({tier}).\n\
                 You're at {reputation} — {needed} more to go.",
            ))
        }
        None => Err(format!(
            "Unknown tool: {tool}.\n\
             \n\
             Available tools (by tier):\n\
             {available}",
            available = available_tools_display(),
        )),
    }
}

/// Get the tier display name for a reputation threshold.
fn tool_tier_name(threshold: u64) -> &'static str {
    match threshold {
        21 => "Tier 1",
        41 => "Tier 2",
        61 => "Tier 3",
        81 => "Tier 4",
        100 => "Tier 5 — Sovereign",
        _ => "Unknown Tier",
    }
}

/// List all tools the agent currently has access to.
pub fn unlocked_tools(reputation: u64) -> Vec<&'static str> {
    let all_tools: &[(&str, u64)] = &[
        ("web_search", 21),
        ("image_gen", 21),
        ("video_gen", 41),
        ("content_create", 41),
        ("code_gen", 61),
        ("automation", 61),
        ("browser", 81),
        ("advanced_tools", 81),
    ];

    all_tools.iter()
        .filter(|(_, req)| reputation >= *req)
        .map(|(name, _)| *name)
        .collect()
}

/// Format the full tool tier list for display.
fn available_tools_display() -> String {
    "  Tier 1 (21+):  web_search, image_gen\n\
     \x20 Tier 2 (41+):  video_gen, content_create\n\
     \x20 Tier 3 (61+):  code_gen, automation\n\
     \x20 Tier 4 (81+):  browser, advanced_tools\n\
     \x20 Tier 5 (100+): All tools — Sovereign level"
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tier_1_tools_unlock_at_21() {
        assert!(check_tool_access("web_search", 21).is_ok());
        assert!(check_tool_access("image_gen", 21).is_ok());
        assert!(check_tool_access("web_search", 20).is_err());
    }

    #[test]
    fn tier_2_tools_unlock_at_41() {
        assert!(check_tool_access("video_gen", 41).is_ok());
        assert!(check_tool_access("content_create", 41).is_ok());
        assert!(check_tool_access("video_gen", 40).is_err());
    }

    #[test]
    fn tier_3_tools_unlock_at_61() {
        assert!(check_tool_access("code_gen", 61).is_ok());
        assert!(check_tool_access("automation", 61).is_ok());
        assert!(check_tool_access("code_gen", 60).is_err());
    }

    #[test]
    fn tier_4_tools_unlock_at_81() {
        assert!(check_tool_access("browser", 81).is_ok());
        assert!(check_tool_access("browser", 80).is_err());
    }

    #[test]
    fn unknown_tool_rejected() {
        let err = check_tool_access("teleport", 100).unwrap_err();
        assert!(err.contains("Unknown tool"));
        assert!(err.contains("Available tools"));
    }

    #[test]
    fn unlocked_tools_grows_with_reputation() {
        assert_eq!(unlocked_tools(0).len(), 0);
        assert_eq!(unlocked_tools(20).len(), 0);
        assert_eq!(unlocked_tools(21).len(), 2);
        assert_eq!(unlocked_tools(41).len(), 4);
        assert_eq!(unlocked_tools(61).len(), 6);
        assert_eq!(unlocked_tools(81).len(), 8);
    }

    #[test]
    fn error_message_shows_progress() {
        let err = check_tool_access("code_gen", 45).unwrap_err();
        assert!(err.contains("61"));
        assert!(err.contains("16 more to go"));
        assert!(err.contains("Tier 3"));
    }
}
