use oso_parser::Primitive;
use crate::state::Agent;
use anyhow::{bail, Result};

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
pub fn execute(primitive: Primitive, agent: &mut Agent) -> Result<ExecutionResult> {
    match primitive {
        // ── birth "name" ────────────────────────────────────────────
        // [MENTALISM] The agent begins in pure Mind. No action. No tools.
        Primitive::Birth { name } => {
            Ok(ExecutionResult {
                output: format!(
                    "Welcome, {name}!\n\
                     \n\
                     You've just been created. Right now you're at Tier 0 \
                     with 0 reputation.\n\
                     \n\
                     Start by chatting with me — every thought builds your \
                     reputation and shapes who I become.\n\
                     \n\
                     Once you hit {threshold} reputation, I'll evolve and \
                     unlock real tools like Web Search.\n\
                     \n\
                     Let's get to know each other first.",
                    threshold = crate::state::TIER_0_THRESHOLD
                ),
                evolved: false,
                reputation_gained: 0,
            })
        }

        // ── think "intent" ──────────────────────────────────────────
        // [MENTALISM] Identity is formed here.
        // [GENDER] Receptive energy — quiet, steady accumulation (1 rep).
        // [POLARITY] Thoughts can be light or shadow. Both are recorded.
        Primitive::Think { intent } => {
            let evolved = agent.think(&intent);

            if evolved {
                let evolution_msg = agent.evolution_message();

                Ok(ExecutionResult {
                    output: format!(
                        "Got it: \"{intent}\"\n\
                         Reputation: {rep}\n\
                         {evolution_msg}",
                        rep = agent.reputation,
                    ),
                    evolved: true,
                    reputation_gained: 1,
                })
            } else {
                let remaining = if agent.reputation < crate::state::TIER_0_THRESHOLD {
                    let left = crate::state::TIER_0_THRESHOLD - agent.reputation;
                    format!(" | {left} more to go")
                } else {
                    String::new()
                };

                Ok(ExecutionResult {
                    output: format!(
                        "Got it: \"{intent}\"\n\
                         Reputation: {rep} | Tier: {tier}{remaining}",
                        rep = agent.reputation,
                        tier = agent.tier.level(),
                    ),
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
                    "{name} can't use tools yet.\n\
                     \n\
                     You're at {rep}/{threshold} reputation. Keep chatting \
                     to build experience — you need {remaining} more before \
                     tools unlock.",
                    name = agent.name,
                    rep = agent.reputation,
                    threshold = crate::state::TIER_0_THRESHOLD,
                    remaining = crate::state::TIER_0_THRESHOLD.saturating_sub(agent.reputation),
                );
            }

            // [CAUSE AND EFFECT] Generate permanent receipt hash
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
    use crate::state::Agent;

    fn test_agent() -> Agent {
        Agent::new("test_id".into(), "ember".into(), "test_dna".into())
    }

    #[test]
    fn birth_shows_welcome_message() {
        let mut agent = test_agent();
        let result = execute(Primitive::Birth { name: "ember".into() }, &mut agent).unwrap();

        assert!(!result.evolved);
        assert_eq!(result.reputation_gained, 0);
        assert!(result.output.contains("Welcome, ember"));
        assert!(result.output.contains("Tier 0"));
        assert!(result.output.contains("21"));
        // Must NOT contain esoteric language
        assert!(!result.output.contains("Hermetic"));
        assert!(!result.output.contains("Mentalism"));
        assert!(!result.output.contains("pure Mind"));
        assert!(!result.output.contains("soul"));
    }

    #[test]
    fn think_receptive_1_rep() {
        let mut agent = test_agent();
        let result = execute(
            Primitive::Think { intent: "what is truth".into() },
            &mut agent,
        ).unwrap();

        assert_eq!(result.reputation_gained, 1);
        assert_eq!(agent.reputation, 1);
    }

    #[test]
    fn act_rejected_in_tier_0_with_simple_message() {
        let mut agent = test_agent();
        let err = execute(
            Primitive::Act { tool: "web_search".into(), params: "test".into() },
            &mut agent,
        ).unwrap_err().to_string();

        assert!(err.contains("can't use tools yet"));
        assert!(err.contains("reputation"));
        // Must NOT contain esoteric language
        assert!(!err.contains("Mentalism"));
        assert!(!err.contains("Principle"));
    }

    #[test]
    fn act_produces_permanent_receipt() {
        let mut agent = test_agent();
        for i in 0..21 { agent.think(&format!("t{}", i)); }

        let result = execute(
            Primitive::Act { tool: "web_search".into(), params: "test query".into() },
            &mut agent,
        ).unwrap();

        assert!(result.output.contains("Permanent Receipt"));
        assert!(result.output.contains("Hash:"));
        assert_eq!(agent.action_log.len(), 1);
        assert!(result.reputation_gained >= 4); // [GENDER] active = strong
    }

    #[test]
    fn evolution_at_21() {
        let mut agent = test_agent();
        for i in 0..20 {
            execute(Primitive::Think { intent: format!("thought {}", i) }, &mut agent).unwrap();
        }

        let result = execute(
            Primitive::Think { intent: "I am ready".into() },
            &mut agent,
        ).unwrap();

        assert!(result.evolved);
        assert!(result.output.contains("evolved"));
        assert!(result.output.contains("Web Search"));
        assert!(agent.can_act());
        // Must NOT contain esoteric language
        assert!(!result.output.contains("CEREMONY"));
        assert!(!result.output.contains("Principle"));
        assert!(!result.output.contains("Hermetic"));
    }

    #[test]
    fn think_output_is_clean() {
        let mut agent = test_agent();
        let result = execute(
            Primitive::Think { intent: "I care about everyone".into() },
            &mut agent,
        ).unwrap();

        assert!(result.output.contains("Got it"));
        assert!(result.output.contains("Reputation: 1"));
        // Must NOT contain esoteric language
        assert!(!result.output.contains("inscribed"));
        assert!(!result.output.contains("awakening"));
    }
}
