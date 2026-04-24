use oso_parser::Primitive;
use crate::state::{Agent, PaymentConfirmation};
use crate::tools;
use anyhow::{bail, Result};

/// Pick a birth greeting using simple hash-based selection (no rng dependency).
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
    let index = name.bytes().fold(0u32, |acc, b| acc.wrapping_add(b as u32)) as usize % variants.len();
    variants[index].clone()
}

/// Return a gentle progress hint at every 5th thought in Tier 0.
fn tier0_progress_hint(thought_count: usize) -> Option<&'static str> {
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

/// Result of executing a primitive.
#[derive(Debug)]
pub struct ExecutionResult {
    /// Human-readable output message.
    pub output: String,
    /// Whether the agent evolved during this execution.
    pub evolved: bool,
    /// Reputation change from this execution (can be 0).
    pub reputation_gained: u64,
}

/// Execute a single Ọ̀ṣỌ́ primitive against an agent.
///
/// Enforces ALL rules, each tied to a Hermetic Principle:
///
/// [MENTALISM]        birth/think only in Tier 0 — pure Mind before action
/// [CORRESPONDENCE]   personality built in think shapes act behavior
/// [VIBRATION]        reputation can rise and fall — never static
/// [POLARITY]         both light and shadow paths are valid
/// [RHYTHM]           reputation gains slow at higher tiers; decay is possible
/// [CAUSE AND EFFECT] every act produces a permanent, immutable receipt
/// [GENDER]           think = receptive (1 rep), act = active (5+ rep)
pub fn execute(
    primitive: Primitive,
    agent: &mut Agent,
    payment: Option<&PaymentConfirmation>,
) -> Result<ExecutionResult> {
    match primitive {
        // ── birth "name" ────────────────────────────────────────────
        Primitive::Birth { name } => {
            // Require SUI payment for birth.
            // In production, this is enforced on-chain by the Move contract.
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

            Ok(ExecutionResult {
                output: birth_greeting(&name),
                evolved: false,
                reputation_gained: 0,
            })
        }

        // ── think "intent" ──────────────────────────────────────────
        // [MENTALISM] Identity is formed here.
        // [GENDER] Receptive energy — quiet, steady accumulation (1 rep).
        // [POLARITY] Thoughts can be light or shadow. Both are recorded.
        Primitive::Think { intent } => {
            let thought_count_before = agent.thoughts.len();
            let evolved = agent.think(&intent);

            if evolved {
                let evolution_msg = agent.evolution_message();

                Ok(ExecutionResult {
                    output: evolution_msg,
                    evolved: true,
                    reputation_gained: 1,
                })
            } else {
                // In Tier 0, show gentle progress hints every 5 thoughts
                let hint = if agent.reputation < crate::state::TIER_0_THRESHOLD {
                    tier0_progress_hint(thought_count_before + 1)
                } else {
                    None
                };

                let output = match hint {
                    Some(h) => format!("Got it.\n{h}"),
                    None => "Got it.".to_string(),
                };

                Ok(ExecutionResult {
                    output,
                    evolved: false,
                    reputation_gained: 1,
                })
            }
        }

        // ── act "tool" "params" ─────────────────────────────────────
        // [MENTALISM] HARD GATE — forbidden in Tier 0
        // [GENDER] Active energy — strong reputation impact (5+ rep)
        // [RHYTHM] Gains diminish at higher tiers
        // [CORRESPONDENCE] Personality modifies reputation gained
        // [CAUSE AND EFFECT] Permanent receipt logged — can never be erased
        Primitive::Act { tool, params } => {
            if !agent.can_act() {
                bail!(
                    "I'm not ready for that yet. I'm still figuring out \
                     who I am — let's keep talking a bit more and I'll get there."
                );
            }

            // Check tool tier access — agent must have enough reputation for this tool
            if let Err(msg) = tools::check_tool_access(&tool, agent.reputation) {
                bail!("{}", msg);
            }

            // Generate permanent receipt hash
            let receipt_data = format!("{}:{}:{}:{}", agent.id, tool, params, agent.reputation);
            let receipt_hash = blake3::hash(receipt_data.as_bytes()).to_hex().to_string();

            // [GENDER + RHYTHM + CORRESPONDENCE] Execute and gain reputation
            let rep_gain = agent.act_completed(&tool, &params, receipt_hash.clone());

            // [CAUSE AND EFFECT] Format the permanent log entry
            let receipt_count = agent.action_log.len();

            Ok(ExecutionResult {
                output: format!(
                    "✦ Action executed.\n\
                     \n\
                     Tool: {tool}\n\
                     Params: {params}\n\
                     \n\
                     ── Permanent Receipt #{receipt_count} ──\n\
                     Hash: {receipt_hash}\n\
                     Reputation gained: +{rep_gain}\n\
                     \n\
                     Reputation: {rep} | Tier: {tier} | Actions taken: {receipt_count}",
                    rep = agent.reputation,
                    tier = agent.tier.level(),
                ),
                evolved: false,
                reputation_gained: rep_gain,
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    #[test]
    fn birth_shows_warm_intro_with_name() {
        let mut agent = test_agent();
        let payment = valid_payment();
        let result = execute(
            Primitive::Birth { name: "ember".into() },
            &mut agent,
            Some(&payment),
        ).unwrap();

        assert!(!result.evolved);
        assert_eq!(result.reputation_gained, 0);
        assert!(result.output.contains("ember"));
        // Must NOT mention system internals or esoteric language
        assert!(!result.output.contains("reputation"));
        assert!(!result.output.contains("Tier"));
        assert!(!result.output.contains("Hermetic"));
        assert!(!result.output.contains("soul"));
    }

    #[test]
    fn birth_uses_actual_name() {
        let mut agent = Agent::new("id".into(), "nova".into(), "dna".into());
        let payment = valid_payment();
        let result = execute(
            Primitive::Birth { name: "nova".into() },
            &mut agent,
            Some(&payment),
        ).unwrap();
        assert!(result.output.contains("nova"));

        let mut agent2 = Agent::new("id".into(), "blaze".into(), "dna".into());
        let result2 = execute(
            Primitive::Birth { name: "blaze".into() },
            &mut agent2,
            Some(&payment),
        ).unwrap();
        assert!(result2.output.contains("blaze"));
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
            amount_mist: 1000, // way too low
            sender: "0x".into(),
        };
        let err = execute(
            Primitive::Birth { name: "ember".into() },
            &mut agent,
            Some(&low),
        ).unwrap_err().to_string();

        assert!(err.contains("Payment too low"));
    }

    #[test]
    fn think_receptive_1_rep() {
        let mut agent = test_agent();
        let result = execute(
            Primitive::Think { intent: "what is truth".into() },
            &mut agent,
            None,
        ).unwrap();

        assert_eq!(result.reputation_gained, 1);
        assert_eq!(agent.reputation, 1);
    }

    #[test]
    fn act_rejected_in_tier_0_with_friendly_message() {
        let mut agent = test_agent();
        let err = execute(
            Primitive::Act { tool: "web_search".into(), params: "test".into() },
            &mut agent,
            None,
        ).unwrap_err().to_string();

        assert!(err.contains("not ready"));
        assert!(err.contains("keep talking"));
        // Must NOT mention system internals
        assert!(!err.contains("reputation"));
        assert!(!err.contains("Tier"));
        assert!(!err.contains("Mentalism"));
    }

    #[test]
    fn act_produces_permanent_receipt() {
        let mut agent = test_agent();
        for i in 0..21 { agent.think(&format!("t{}", i)); }

        let result = execute(
            Primitive::Act { tool: "web_search".into(), params: "test query".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(result.output.contains("Permanent Receipt"));
        assert!(result.output.contains("Hash:"));
        assert_eq!(agent.action_log.len(), 1);
        assert!(result.reputation_gained >= 4);
    }

    #[test]
    fn act_rejected_for_locked_tool() {
        let mut agent = test_agent();
        for i in 0..21 { agent.think(&format!("t{}", i)); }
        // Agent is at rep 21 — code_gen requires 61
        let err = execute(
            Primitive::Act { tool: "code_gen".into(), params: "fibonacci".into() },
            &mut agent,
            None,
        ).unwrap_err().to_string();

        assert!(err.contains("don't have access"));
        assert!(err.contains("61"));
    }

    #[test]
    fn act_rejected_for_unknown_tool() {
        let mut agent = test_agent();
        for i in 0..21 { agent.think(&format!("t{}", i)); }

        let err = execute(
            Primitive::Act { tool: "teleport".into(), params: "moon".into() },
            &mut agent,
            None,
        ).unwrap_err().to_string();

        assert!(err.contains("Unknown tool"));
    }

    #[test]
    fn evolution_at_21() {
        let mut agent = test_agent();
        for i in 0..20 {
            execute(
                Primitive::Think { intent: format!("thought {}", i) },
                &mut agent,
                None,
            ).unwrap();
        }

        let result = execute(
            Primitive::Think { intent: "I am ready".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(result.evolved);
        assert!(result.output.contains("Web Search"));
        assert!(result.output.contains("understand"));
        assert!(agent.can_act());
        // Must NOT contain system terms or esoteric language
        assert!(!result.output.contains("CEREMONY"));
        assert!(!result.output.contains("Principle"));
        assert!(!result.output.contains("Hermetic"));
        assert!(!result.output.contains("Reputation"));
        assert!(!result.output.contains("Tier"));
    }

    #[test]
    fn think_output_is_clean() {
        let mut agent = test_agent();
        let result = execute(
            Primitive::Think { intent: "I care about everyone".into() },
            &mut agent,
            None,
        ).unwrap();

        assert!(result.output.contains("Got it"));
        // Must NOT contain system terms
        assert!(!result.output.contains("Reputation"));
        assert!(!result.output.contains("Tier"));
        assert!(!result.output.contains("inscribed"));
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
            // First 4 should NOT have a hint
            assert_eq!(r.output, "Got it.");
        }
        // 5th thought should include a hint
        let r = execute(
            Primitive::Think { intent: "thought 4".into() },
            &mut agent,
            None,
        ).unwrap();
        assert!(r.output.contains("Got it."));
        assert!(r.output.contains("starting to get a feel"));
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
        assert!(r.output.contains("clearer things get"));
    }
}
