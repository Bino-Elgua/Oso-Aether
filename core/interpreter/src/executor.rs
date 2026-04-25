use oso_parser::Primitive;
use crate::events::{ExecutionEvent, PersonalitySnapshot};
use crate::response::{DefaultResponder, Response, ResponseGenerator};
use crate::state::{Agent, PaymentConfirmation};
use crate::tools;
use anyhow::{bail, Result};

/// Execute a primitive and return a structured event.
///
/// This is the core function. It enforces all rules and returns
/// a structured ExecutionEvent containing only verified facts.
/// No natural language is generated here.
pub fn execute_event(
    primitive: Primitive,
    agent: &mut Agent,
    payment: Option<&PaymentConfirmation>,
) -> Result<ExecutionEvent> {
    match primitive {
        Primitive::Birth { name } => {
            match payment {
                Some(p) if p.is_valid_for_birth() => {}
                Some(_) => bail!(
                    "Payment too low. Birthing an agent costs at least {} MIST (0.1 SUI).",
                    crate::state::BIRTH_COST_MIST,
                ),
                None => bail!(
                    "A SUI payment is required to birth a new agent.\n\
                     Cost: {} MIST (0.1 SUI).\n\
                     Connect your wallet to continue.",
                    crate::state::BIRTH_COST_MIST,
                ),
            }

            Ok(ExecutionEvent::BirthSuccess { name })
        }

        Primitive::Think { intent } => {
            // Slash command intents are tagged with [brackets].
            // System queries don't count as thoughts.
            match intent.as_str() {
                "[status]" => {
                    let alignment = match agent.alignment {
                        crate::state::Alignment::Light => "Light",
                        crate::state::Alignment::Shadow => "Shadow",
                        crate::state::Alignment::Neutral => "Neutral",
                    };
                    return Ok(ExecutionEvent::StatusRequested {
                        name: agent.name.clone(),
                        reputation: agent.reputation,
                        tier: agent.tier.level(),
                        alignment: alignment.to_string(),
                        personality: PersonalitySnapshot {
                            curiosity: agent.personality.curiosity,
                            boldness: agent.personality.boldness,
                            empathy: agent.personality.empathy,
                        },
                        thought_count: agent.thoughts.len(),
                        action_count: agent.action_log.len(),
                    });
                }
                "[tools]" => {
                    let unlocked: Vec<String> = tools::unlocked_tools(agent.reputation)
                        .into_iter().map(|s| s.to_string()).collect();
                    let all_tools = [
                        ("web_search", 21), ("image_gen", 21),
                        ("video_gen", 41), ("content_create", 41),
                        ("code_gen", 61), ("automation", 61),
                        ("browser", 81), ("advanced_tools", 81),
                    ];
                    let locked: Vec<(String, u64)> = all_tools.iter()
                        .filter(|(_, req)| agent.reputation < *req)
                        .map(|(name, req)| (name.to_string(), *req))
                        .collect();
                    return Ok(ExecutionEvent::ToolsRequested { unlocked, locked });
                }
                "[help]" => return Ok(ExecutionEvent::HelpRequested),
                "[personality]" => {
                    return Ok(ExecutionEvent::PersonalityRequested {
                        name: agent.name.clone(),
                        personality: PersonalitySnapshot {
                            curiosity: agent.personality.curiosity,
                            boldness: agent.personality.boldness,
                            empathy: agent.personality.empathy,
                        },
                        agent_type: agent.agent_type(),
                    });
                }
                "[clear]" => return Ok(ExecutionEvent::ConversationCleared),
                "[sandbox on]" => {
                    agent.set_sandbox(true);
                    return Ok(ExecutionEvent::SandboxToggled { enabled: true });
                }
                "[sandbox off]" => {
                    agent.set_sandbox(false);
                    return Ok(ExecutionEvent::SandboxToggled { enabled: false });
                }
                "[export]" => {
                    return Ok(ExecutionEvent::ExportGenerated {
                        name: agent.name.clone(),
                        thought_count: agent.thoughts.len(),
                        action_count: agent.action_log.len(),
                    });
                }
                _ => {}
            }

            // /private and /publish — these DO record a thought
            if intent.starts_with("[private] ") {
                let message = intent.strip_prefix("[private] ").unwrap().to_string();
                agent.think_private(&message);
                return Ok(ExecutionEvent::PrivateThoughtRecorded {
                    message,
                    thought_count: agent.thoughts.len(),
                });
            }
            if intent.starts_with("[publish] ") {
                let message = intent.strip_prefix("[publish] ").unwrap().to_string();
                let _content_hash = agent.think_and_publish(&message);
                return Ok(ExecutionEvent::PublishRequested {
                    message,
                    thought_count: agent.thoughts.len(),
                });
            }

            // Normal thought
            let evolved = agent.think(&intent);

            if evolved {
                let agent_type = agent.agent_type();
                let new_tools = tools::unlocked_tools(agent.reputation)
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect();

                Ok(ExecutionEvent::Evolved {
                    name: agent.name.clone(),
                    agent_type,
                    new_unlocked_tools: new_tools,
                    thought_count: agent.thoughts.len(),
                })
            } else {
                let in_tier_zero = agent.reputation < crate::state::TIER_0_THRESHOLD;
                Ok(ExecutionEvent::ThinkReceived {
                    intent,
                    thought_count: agent.thoughts.len(),
                    in_tier_zero,
                })
            }
        }

        Primitive::Act { tool, params } => {
            if !agent.can_act() {
                return Ok(ExecutionEvent::ActionBlockedTier0 {
                    name: agent.name.clone(),
                    current_rep: agent.reputation,
                    required_rep: crate::state::TIER_0_THRESHOLD,
                });
            }

            if let Some(required) = tools::tool_reputation_requirement(&tool) {
                if agent.reputation < required {
                    return Ok(ExecutionEvent::ActionBlockedToolLocked {
                        tool,
                        current_rep: agent.reputation,
                        required_rep: required,
                    });
                }
            } else {
                return Ok(ExecutionEvent::ActionBlockedUnknownTool {
                    tool,
                    available_tools: tools::unlocked_tools(agent.reputation)
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect(),
                });
            }

            let receipt_data = format!("{}:{}:{}:{}", agent.id, tool, params, agent.reputation);
            let receipt_hash = blake3::hash(receipt_data.as_bytes()).to_hex().to_string();
            let in_sandbox = agent.sandbox_mode;
            let rep_gain = agent.act_completed(&tool, &params, receipt_hash.clone());
            let receipt_number = agent.action_log.len();

            if in_sandbox {
                Ok(ExecutionEvent::ActionCompletedSandbox {
                    tool,
                    params,
                    receipt_hash,
                    receipt_number,
                    reputation_gained: rep_gain,
                })
            } else {
                Ok(ExecutionEvent::ActionCompleted {
                    tool,
                    params,
                    receipt_hash,
                    receipt_number,
                    reputation_gained: rep_gain,
                })
            }
        }
    }
}

/// Execute a primitive and return a user-facing Response.
///
/// This is the convenience function that most callers should use.
/// It runs the executor, then renders the event through the default
/// response generator. For LLM-powered responses, call `execute_event()`
/// directly and pass the event to your own ResponseGenerator.
pub fn execute(
    primitive: Primitive,
    agent: &mut Agent,
    payment: Option<&PaymentConfirmation>,
) -> Result<Response> {
    let event = execute_event(primitive, agent, payment)?;
    let responder = DefaultResponder;
    Ok(responder.render(&event))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::AgentType;
    use crate::state::{Agent, PaymentConfirmation};

    fn test_agent() -> Agent {
        Agent::new("test_id".into(), "ember".into(), "test_dna".into())
    }

    fn valid_payment() -> PaymentConfirmation {
        PaymentConfirmation {
            tx_digest: "test_tx_abc123".into(),
            amount_mist: 100_000_000,
            sender: "0xtest".into(),
        }
    }

    // ── Birth events ──

    #[test]
    fn birth_event_contains_name() {
        let mut agent = test_agent();
        let event = execute_event(
            Primitive::Birth { name: "ember".into() },
            &mut agent,
            Some(&valid_payment()),
        ).unwrap();

        assert!(matches!(event, ExecutionEvent::BirthSuccess { ref name } if name == "ember"));
    }

    #[test]
    fn birth_response_contains_name() {
        let mut agent = test_agent();
        let r = execute(
            Primitive::Birth { name: "ember".into() },
            &mut agent,
            Some(&valid_payment()),
        ).unwrap();

        assert!(r.message.contains("ember"));
        assert!(!r.evolved);
        assert!(!r.message.contains("reputation"));
        assert!(!r.message.contains("Tier"));
    }

    #[test]
    fn birth_uses_actual_name() {
        let payment = valid_payment();

        let mut a1 = Agent::new("id".into(), "nova".into(), "dna".into());
        let r1 = execute(Primitive::Birth { name: "nova".into() }, &mut a1, Some(&payment)).unwrap();
        assert!(r1.message.contains("nova"));

        let mut a2 = Agent::new("id".into(), "blaze".into(), "dna".into());
        let r2 = execute(Primitive::Birth { name: "blaze".into() }, &mut a2, Some(&payment)).unwrap();
        assert!(r2.message.contains("blaze"));
    }

    #[test]
    fn birth_rejected_without_payment() {
        let mut agent = test_agent();
        let err = execute(
            Primitive::Birth { name: "ember".into() },
            &mut agent,
            None,
        ).unwrap_err().to_string();
        assert!(err.contains("payment is required"));
    }

    #[test]
    fn birth_rejected_with_low_payment() {
        let mut agent = test_agent();
        let low = PaymentConfirmation {
            tx_digest: "tx".into(),
            amount_mist: 1000,
            sender: "0x".into(),
        };
        let err = execute(
            Primitive::Birth { name: "ember".into() },
            &mut agent,
            Some(&low),
        ).unwrap_err().to_string();
        assert!(err.contains("Payment too low"));
    }

    // ── Think events ──

    #[test]
    fn think_event_tracks_count() {
        let mut agent = test_agent();
        let event = execute_event(
            Primitive::Think { intent: "hello".into() },
            &mut agent,
            None,
        ).unwrap();

        match event {
            ExecutionEvent::ThinkReceived { thought_count, in_tier_zero, .. } => {
                assert_eq!(thought_count, 1);
                assert!(in_tier_zero);
            }
            _ => panic!("expected ThinkReceived"),
        }
    }

    #[test]
    fn think_response_is_clean() {
        let mut agent = test_agent();
        let r = execute(
            Primitive::Think { intent: "I care about everyone".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(r.message.contains("Got it"));
        assert!(!r.message.contains("Reputation"));
        assert!(!r.message.contains("Tier"));
    }

    #[test]
    fn think_shows_progress_hint_at_5() {
        let mut agent = test_agent();
        for i in 0..4 {
            let r = execute(
                Primitive::Think { intent: format!("thought {}", i) },
                &mut agent,
                None,
            ).unwrap();
            assert_eq!(r.message, "Got it.");
        }
        let r = execute(
            Primitive::Think { intent: "thought 4".into() },
            &mut agent,
            None,
        ).unwrap();
        assert!(r.message.contains("starting to get a feel"));
    }

    #[test]
    fn think_shows_progress_hint_at_10() {
        let mut agent = test_agent();
        for i in 0..9 {
            execute(
                Primitive::Think { intent: format!("t{}", i) },
                &mut agent,
                None,
            ).unwrap();
        }
        let r = execute(
            Primitive::Think { intent: "t9".into() },
            &mut agent,
            None,
        ).unwrap();
        assert!(r.message.contains("clearer things get"));
    }

    // ── Evolution ──

    #[test]
    fn evolution_event_has_tools_and_type() {
        let mut agent = test_agent();
        for _ in 0..20 {
            agent.think("why does everything work and how can I learn");
        }
        let event = execute_event(
            Primitive::Think { intent: "final question to understand".into() },
            &mut agent,
            None,
        ).unwrap();

        match event {
            ExecutionEvent::Evolved { agent_type, new_unlocked_tools, .. } => {
                assert_eq!(agent_type, AgentType::Research);
                assert!(new_unlocked_tools.contains(&"web_search".to_string()));
                assert!(new_unlocked_tools.contains(&"image_gen".to_string()));
            }
            _ => panic!("expected Evolved"),
        }
    }

    #[test]
    fn evolution_response_references_tools() {
        let mut agent = test_agent();
        for i in 0..20 {
            execute(
                Primitive::Think { intent: format!("thought {}", i) },
                &mut agent,
                None,
            ).unwrap();
        }
        let r = execute(
            Primitive::Think { intent: "I am ready".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(r.evolved);
        assert!(r.message.contains("web_search"));
        assert!(r.message.contains("understand"));
        assert!(!r.message.contains("CEREMONY"));
        assert!(!r.message.contains("Principle"));
        assert!(!r.message.contains("Hermetic"));
        assert!(!r.message.contains("Reputation"));
        assert!(!r.message.contains("Tier"));
    }

    // ── Act events ──

    #[test]
    fn act_blocked_tier0_event() {
        let mut agent = test_agent();
        let event = execute_event(
            Primitive::Act { tool: "web_search".into(), params: "test".into() },
            &mut agent,
            None,
        ).unwrap();

        match event {
            ExecutionEvent::ActionBlockedTier0 { current_rep, required_rep, .. } => {
                assert_eq!(current_rep, 0);
                assert_eq!(required_rep, 21);
            }
            _ => panic!("expected ActionBlockedTier0"),
        }
    }

    #[test]
    fn act_blocked_tier0_response_is_friendly() {
        let mut agent = test_agent();
        let r = execute(
            Primitive::Act { tool: "web_search".into(), params: "test".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(r.message.contains("not ready"));
        assert!(r.message.contains("keep talking"));
        assert!(!r.message.contains("reputation"));
        assert!(!r.message.contains("Tier"));
    }

    #[test]
    fn act_blocked_locked_tool_event() {
        let mut agent = test_agent();
        for i in 0..21 { agent.think(&format!("t{}", i)); }

        let event = execute_event(
            Primitive::Act { tool: "code_gen".into(), params: "fib".into() },
            &mut agent,
            None,
        ).unwrap();

        match event {
            ExecutionEvent::ActionBlockedToolLocked { tool, current_rep, required_rep } => {
                assert_eq!(tool, "code_gen");
                assert_eq!(current_rep, 21);
                assert_eq!(required_rep, 61);
            }
            _ => panic!("expected ActionBlockedToolLocked"),
        }
    }

    #[test]
    fn act_blocked_unknown_tool_event() {
        let mut agent = test_agent();
        for i in 0..21 { agent.think(&format!("t{}", i)); }

        let event = execute_event(
            Primitive::Act { tool: "teleport".into(), params: "moon".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(matches!(event, ExecutionEvent::ActionBlockedUnknownTool { .. }));
    }

    #[test]
    fn act_success_event() {
        let mut agent = test_agent();
        for i in 0..21 { agent.think(&format!("t{}", i)); }

        let event = execute_event(
            Primitive::Act { tool: "web_search".into(), params: "test query".into() },
            &mut agent,
            None,
        ).unwrap();

        match event {
            ExecutionEvent::ActionCompleted { tool, receipt_number, reputation_gained, .. } => {
                assert_eq!(tool, "web_search");
                assert_eq!(receipt_number, 1);
                assert!(reputation_gained >= 4);
            }
            _ => panic!("expected ActionCompleted"),
        }
    }

    #[test]
    fn act_success_response() {
        let mut agent = test_agent();
        for i in 0..21 { agent.think(&format!("t{}", i)); }

        let r = execute(
            Primitive::Act { tool: "web_search".into(), params: "test query".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(r.message.contains("web_search"));
        assert!(r.message.contains("Receipt"));
        assert!(r.reputation_gained >= 4);
    }

    // ── Sandbox mode ──

    #[test]
    fn sandbox_on_event() {
        let mut agent = test_agent();
        let event = execute_event(
            Primitive::Think { intent: "[sandbox on]".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(matches!(event, ExecutionEvent::SandboxToggled { enabled: true }));
        assert!(agent.sandbox_mode);
    }

    #[test]
    fn sandbox_off_event() {
        let mut agent = test_agent();
        agent.set_sandbox(true);

        let event = execute_event(
            Primitive::Think { intent: "[sandbox off]".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(matches!(event, ExecutionEvent::SandboxToggled { enabled: false }));
        assert!(!agent.sandbox_mode);
    }

    #[test]
    fn act_in_sandbox_is_private() {
        let mut agent = test_agent();
        for i in 0..21 { agent.think(&format!("t{}", i)); }
        agent.set_sandbox(true);

        let event = execute_event(
            Primitive::Act { tool: "web_search".into(), params: "test".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(matches!(event, ExecutionEvent::ActionCompletedSandbox { .. }));
    }

    #[test]
    fn act_outside_sandbox_is_normal() {
        let mut agent = test_agent();
        for i in 0..21 { agent.think(&format!("t{}", i)); }

        let event = execute_event(
            Primitive::Act { tool: "web_search".into(), params: "test".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(matches!(event, ExecutionEvent::ActionCompleted { .. }));
    }

    #[test]
    fn sandbox_response_is_friendly() {
        let mut agent = test_agent();

        let r = execute(
            Primitive::Think { intent: "[sandbox on]".into() },
            &mut agent,
            None,
        ).unwrap();
        assert!(r.message.contains("Sandbox mode is on"));

        let r = execute(
            Primitive::Think { intent: "[sandbox off]".into() },
            &mut agent,
            None,
        ).unwrap();
        assert!(r.message.contains("Sandbox mode is off"));
    }

    // ── Private thoughts ──

    #[test]
    fn private_thought_records_in_private_list() {
        let mut agent = test_agent();
        let event = execute_event(
            Primitive::Think { intent: "[private] secret stuff".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(matches!(event, ExecutionEvent::PrivateThoughtRecorded { .. }));
        assert!(agent.private_thoughts.contains(&"secret stuff".to_string()));
    }

    // ── Publish ──

    #[test]
    fn publish_records_garden_hash() {
        let mut agent = test_agent();
        let event = execute_event(
            Primitive::Think { intent: "[publish] my public thought".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(matches!(event, ExecutionEvent::PublishRequested { .. }));
        assert_eq!(agent.garden_hashes.len(), 1);
        assert!(!agent.garden_hashes[0].is_empty());
    }

    // ── Odu key evolves ──

    #[test]
    fn odu_key_evolves_on_think() {
        let mut agent = test_agent();
        let key_before = agent.odu_key.derived_key.clone();

        execute_event(
            Primitive::Think { intent: "hello world".into() },
            &mut agent,
            None,
        ).unwrap();

        assert_ne!(agent.odu_key.derived_key, key_before);
        assert_eq!(agent.odu_key.evolution_count, 1);
    }

    #[test]
    fn odu_key_evolves_on_act() {
        let mut agent = test_agent();
        for i in 0..21 { agent.think(&format!("t{}", i)); }
        let key_before = agent.odu_key.derived_key.clone();

        execute_event(
            Primitive::Act { tool: "web_search".into(), params: "test".into() },
            &mut agent,
            None,
        ).unwrap();

        assert_ne!(agent.odu_key.derived_key, key_before);
    }
}
