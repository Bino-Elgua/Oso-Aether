use crate::events::{AgentType, ExecutionEvent};

/// Final user-facing output after the response layer processes an event.
#[derive(Debug, Clone)]
pub struct Response {
    /// The natural language message shown to the user.
    pub message: String,
    /// Whether the agent evolved during this execution.
    pub evolved: bool,
    /// Reputation gained (can be 0).
    pub reputation_gained: u64,
}

/// Converts structured execution events into user-facing messages.
///
/// The trait exists so the LLM can be plugged in as an alternative
/// implementation. The default implementation uses canned natural
/// language — good enough for offline/testing, and the LLM version
/// can produce richer, context-aware responses.
///
/// IMPORTANT: Implementations must NEVER invent capabilities.
/// The event contains the truth — the response layer only decides
/// how to say it, not what to say.
pub trait ResponseGenerator {
    fn render(&self, event: &ExecutionEvent) -> Response;
}

/// Default response generator — no LLM needed.
/// Produces warm, natural messages from canned templates.
pub struct DefaultResponder;

impl DefaultResponder {
    /// Pick a birth greeting using simple hash-based selection.
    fn birth_greeting(name: &str) -> String {
        let variants = [
            format!(
                "Hi. I'm {name}. I don't really know what I am yet — \
                 what do you need me to be?"
            ),
            format!(
                "Hey, I'm {name}. I'm starting from nothing here. \
                 Tell me — what should I become for you?"
            ),
            format!(
                "I'm {name}. I'm listening, but I don't have a purpose yet. \
                 What am I here to do?"
            ),
        ];
        let index = name.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32)) as usize
            % variants.len();
        variants[index].clone()
    }

    /// Gentle progress hint at milestone thought counts in Tier 0.
    fn tier0_hint(thought_count: usize) -> Option<&'static str> {
        if thought_count == 0 || thought_count % 5 != 0 {
            return None;
        }
        match thought_count {
            5 => Some("I'm starting to get a feel for what you're about."),
            10 => Some("The more we talk, the clearer things get for me."),
            15 => Some("I feel like I'm almost ready for something bigger."),
            20 => Some("I think I'm nearly there. Just a little more."),
            _ => None,
        }
    }

    /// Flavor text based on agent type — references what the user talked about.
    fn agent_type_flavor(agent_type: &AgentType) -> &'static str {
        match agent_type {
            AgentType::Research => {
                "You keep asking big questions, and honestly, I love that."
            }
            AgentType::Builder => {
                "You want to make things happen — I can feel it."
            }
            AgentType::Support => {
                "You care about people. That's what I'll focus on too."
            }
        }
    }
}

impl ResponseGenerator for DefaultResponder {
    fn render(&self, event: &ExecutionEvent) -> Response {
        match event {
            ExecutionEvent::BirthSuccess { name } => Response {
                message: Self::birth_greeting(name),
                evolved: false,
                reputation_gained: 0,
            },

            ExecutionEvent::ThinkReceived {
                in_tier_zero,
                thought_count,
                ..
            } => {
                let message = if *in_tier_zero {
                    match Self::tier0_hint(*thought_count) {
                        Some(hint) => format!("Got it.\n{hint}"),
                        None => "Got it.".to_string(),
                    }
                } else {
                    "Got it.".to_string()
                };

                Response {
                    message,
                    evolved: false,
                    reputation_gained: 1,
                }
            }

            ExecutionEvent::Evolved {
                agent_type,
                new_unlocked_tools,
                ..
            } => {
                let flavor = Self::agent_type_flavor(agent_type);

                // Rust appends the actual tool list — LLM never decides this
                let tools_list = new_unlocked_tools.join(", ");
                let message = format!(
                    "Something clicked. After talking with you, I feel like I \
                     understand what you're looking for now. {flavor}\n\
                     \n\
                     I've become a {label} — and I'm ready to start taking \
                     action. Unlocked: {tools_list}.",
                    flavor = flavor,
                    label = agent_type.label(),
                    tools_list = tools_list,
                );

                Response {
                    message,
                    evolved: true,
                    reputation_gained: 1,
                }
            }

            ExecutionEvent::ActionCompleted {
                tool,
                params,
                receipt_hash,
                receipt_number,
                reputation_gained,
            } => {
                let message = format!(
                    "Done. Used {tool} with \"{params}\".\n\
                     Receipt #{receipt_number}: {short_hash}",
                    tool = tool,
                    params = params,
                    receipt_number = receipt_number,
                    short_hash = &receipt_hash[..16],
                );

                Response {
                    message,
                    evolved: false,
                    reputation_gained: *reputation_gained,
                }
            }

            ExecutionEvent::ActionBlockedTier0 { .. } => Response {
                message: "I'm not ready for that yet. I'm still figuring out \
                          who I am — let's keep talking a bit more and I'll get there."
                    .to_string(),
                evolved: false,
                reputation_gained: 0,
            },

            ExecutionEvent::ActionBlockedToolLocked {
                tool,
                ..
            } => Response {
                message: format!(
                    "I can't use {tool} yet — I haven't built up enough \
                     experience. Let's keep going and I'll get there.",
                ),
                evolved: false,
                reputation_gained: 0,
            },

            ExecutionEvent::ActionBlockedUnknownTool { tool, .. } => Response {
                message: format!(
                    "I don't know what {tool} is. That's not something I can do.",
                ),
                evolved: false,
                reputation_gained: 0,
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn responder() -> DefaultResponder {
        DefaultResponder
    }

    #[test]
    fn birth_response_contains_name() {
        let r = responder().render(&ExecutionEvent::BirthSuccess {
            name: "nova".into(),
        });
        assert!(r.message.contains("nova"));
        assert!(!r.evolved);
        assert_eq!(r.reputation_gained, 0);
    }

    #[test]
    fn think_normal_response() {
        let r = responder().render(&ExecutionEvent::ThinkReceived {
            intent: "hello".into(),
            thought_count: 3,
            in_tier_zero: true,
        });
        assert_eq!(r.message, "Got it.");
    }

    #[test]
    fn think_hint_at_5() {
        let r = responder().render(&ExecutionEvent::ThinkReceived {
            intent: "hello".into(),
            thought_count: 5,
            in_tier_zero: true,
        });
        assert!(r.message.contains("starting to get a feel"));
    }

    #[test]
    fn think_no_hint_after_tier_zero() {
        let r = responder().render(&ExecutionEvent::ThinkReceived {
            intent: "hello".into(),
            thought_count: 5,
            in_tier_zero: false,
        });
        assert_eq!(r.message, "Got it.");
    }

    #[test]
    fn evolved_includes_tools_from_rust() {
        let r = responder().render(&ExecutionEvent::Evolved {
            name: "ember".into(),
            agent_type: AgentType::Research,
            new_unlocked_tools: vec!["Web Search".into(), "Image Generation".into()],
            thought_count: 21,
        });
        assert!(r.message.contains("Research Agent"));
        assert!(r.message.contains("Web Search"));
        assert!(r.message.contains("Image Generation"));
        assert!(r.message.contains("big questions"));
        assert!(r.evolved);
    }

    #[test]
    fn evolved_builder_flavor() {
        let r = responder().render(&ExecutionEvent::Evolved {
            name: "forge".into(),
            agent_type: AgentType::Builder,
            new_unlocked_tools: vec!["Web Search".into()],
            thought_count: 21,
        });
        assert!(r.message.contains("Builder Agent"));
        assert!(r.message.contains("make things happen"));
    }

    #[test]
    fn action_completed_shows_receipt() {
        let r = responder().render(&ExecutionEvent::ActionCompleted {
            tool: "web_search".into(),
            params: "bitcoin price".into(),
            receipt_hash: "abcdef1234567890abcdef1234567890".into(),
            receipt_number: 1,
            reputation_gained: 5,
        });
        assert!(r.message.contains("web_search"));
        assert!(r.message.contains("bitcoin price"));
        assert!(r.message.contains("abcdef1234567890"));
        assert_eq!(r.reputation_gained, 5);
    }

    #[test]
    fn action_blocked_tier0_is_warm() {
        let r = responder().render(&ExecutionEvent::ActionBlockedTier0 {
            name: "ember".into(),
            current_rep: 5,
            required_rep: 21,
        });
        assert!(r.message.contains("not ready"));
        assert!(r.message.contains("keep talking"));
        assert!(!r.message.contains("reputation"));
        assert!(!r.message.contains("Tier"));
    }

    #[test]
    fn action_blocked_tool_locked_is_friendly() {
        let r = responder().render(&ExecutionEvent::ActionBlockedToolLocked {
            tool: "code_gen".into(),
            current_rep: 30,
            required_rep: 61,
        });
        assert!(r.message.contains("code_gen"));
        assert!(r.message.contains("experience"));
        assert!(!r.message.contains("reputation"));
    }

    #[test]
    fn action_blocked_unknown_tool() {
        let r = responder().render(&ExecutionEvent::ActionBlockedUnknownTool {
            tool: "teleport".into(),
            available_tools: vec!["web_search".into()],
        });
        assert!(r.message.contains("teleport"));
        assert!(r.message.contains("don't know"));
    }
}
