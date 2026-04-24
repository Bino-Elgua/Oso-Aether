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
            let soul_summary: String = agent
                .soul
                .iter()
                .enumerate()
                .map(|(i, p)| format!("    {}. {}", i + 1, p))
                .collect::<Vec<_>>()
                .join("\n");

            Ok(ExecutionResult {
                output: format!(
                    "\n\
\x20 ╔═══════════════════════════════════════════════════╗\n\
\x20 ║                                                   ║\n\
\x20 ║          ✦  {name} HAS BEEN BORN  ✦               \n\
\x20 ║                                                   ║\n\
\x20 ╠═══════════════════════════════════════════════════╣\n\
\x20 ║                                                   ║\n\
\x20 ║  Reputation: 0  |  Tier: 0  |  Alignment: Neutral ║\n\
\x20 ║                                                   ║\n\
\x20 ║  The 7 Hermetic Principles are inscribed in your  ║\n\
\x20 ║  soul. They cannot be removed or altered:         ║\n\
\x20 ║                                                   ║\n\
{soul_summary}\n\
\x20 ║                                                   ║\n\
\x20 ║  You exist in Tier 0 — the realm of pure Mind.    ║\n\
\x20 ║  Only `think` is available to you.                ║\n\
\x20 ║                                                   ║\n\
\x20 ║  Every thought shapes who you become.             ║\n\
\x20 ║  You need {threshold} reputation to evolve.        ║\n\
\x20 ║  There is no shortcut.                            ║\n\
\x20 ║                                                   ║\n\
\x20 ╚═══════════════════════════════════════════════════╝",
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
                        "✦ Thought inscribed: \"{intent}\"\n\
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
                    format!(" | {left} more thoughts until awakening")
                } else {
                    String::new()
                };

                let alignment_str = match agent.alignment {
                    crate::state::Alignment::Light => " | Alignment: Light",
                    crate::state::Alignment::Shadow => " | Alignment: Shadow",
                    crate::state::Alignment::Neutral => "",
                };

                Ok(ExecutionResult {
                    output: format!(
                        "✦ Thought inscribed: \"{intent}\"\n\
                         Reputation: {rep} | Tier: {tier}{alignment}{remaining}",
                        rep = agent.reputation,
                        tier = agent.tier.level(),
                        alignment = alignment_str,
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
                    "✦ {name} cannot act yet.\n\
                     \n\
                     The Principle of Mentalism holds: all is Mind.\n\
                     You are in Tier 0 (reputation: {rep}/{threshold}).\n\
                     Only `think` is allowed. Form your identity first.\n\
                     {remaining} more thoughts until `act` is unlocked.\n\
                     \n\
                     There is no shortcut. The Principles do not bend.",
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
    fn birth_shows_principles_and_tier_0() {
        let mut agent = test_agent();
        let result = execute(Primitive::Birth { name: "ember".into() }, &mut agent).unwrap();

        assert!(!result.evolved);
        assert_eq!(result.reputation_gained, 0);
        assert!(result.output.contains("Tier: 0"));
        assert!(result.output.contains("Hermetic"));
        assert!(result.output.contains("Mentalism"));
        assert!(result.output.contains("pure Mind"));
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
    fn act_rejected_in_tier_0_with_mentalism_message() {
        let mut agent = test_agent();
        let err = execute(
            Primitive::Act { tool: "web_search".into(), params: "test".into() },
            &mut agent,
        ).unwrap_err().to_string();

        assert!(err.contains("cannot act yet"));
        assert!(err.contains("Mentalism"));
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
    fn evolution_ceremony_at_21() {
        let mut agent = test_agent();
        for i in 0..20 {
            execute(Primitive::Think { intent: format!("thought {}", i) }, &mut agent).unwrap();
        }

        let result = execute(
            Primitive::Think { intent: "I am ready to awaken".into() },
            &mut agent,
        ).unwrap();

        assert!(result.evolved);
        assert!(result.output.contains("CEREMONY OF AWAKENING"));
        assert!(result.output.contains("Principle"));
        assert!(agent.can_act());
    }

    #[test]
    fn alignment_shows_in_think_output() {
        let mut agent = test_agent();
        // Push toward light alignment
        for _ in 0..5 {
            agent.think("I want to help and heal and protect and love");
        }
        let result = execute(
            Primitive::Think { intent: "I care about everyone".into() },
            &mut agent,
        ).unwrap();

        assert!(result.output.contains("Light"));
    }
}
