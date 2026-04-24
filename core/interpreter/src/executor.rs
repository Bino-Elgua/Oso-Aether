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
    /// Reputation gained from this execution.
    pub reputation_gained: u64,
}

/// Execute a single Ọ̀ṣỌ́ primitive against an agent.
///
/// Enforces ALL rules:
/// - `birth` creates a fresh agent (reputation 0, Tier 0, Hermetic soul)
/// - `think` builds memory and gains reputation (1 per thought)
/// - `act` is REJECTED if agent is in Tier 0 (reputation < 21)
pub fn execute(primitive: Primitive, agent: &mut Agent) -> Result<ExecutionResult> {
    match primitive {
        // ── birth "name" ────────────────────────────────────────────
        Primitive::Birth { name } => {
            // Birth creates the agent fresh. The caller must have already
            // constructed the Agent via Agent::new(). This is a confirmation.
            //
            // In the full system, this triggers:
            //   1. SUI payment (handled by Move contract)
            //   2. 86-DNA generation
            //   3. dNFT minting
            //   4. Walrus memory initialization
            //
            // The agent ALWAYS starts clean:
            //   - reputation = 0
            //   - tier = Tier::Zero
            //   - no inherited memory, tools, or reputation

            let soul_summary: String = agent
                .soul
                .iter()
                .enumerate()
                .map(|(i, p)| format!("  {}. {}", i + 1, p))
                .collect::<Vec<_>>()
                .join("\n");

            Ok(ExecutionResult {
                output: format!(
                    "✦ {name} has been born.\n\
                     \n\
                     Reputation: 0 | Tier: 0\n\
                     \n\
                     The 7 Hermetic Principles are inscribed in your soul:\n\
                     {soul_summary}\n\
                     \n\
                     You are in Tier 0. Only `think` is available.\n\
                     Contemplate. Build your identity. Earn your right to act.\n\
                     You need {threshold} reputation to evolve.",
                    threshold = crate::state::TIER_0_THRESHOLD
                ),
                evolved: false,
                reputation_gained: 0,
            })
        }

        // ── think "intent" ──────────────────────────────────────────
        Primitive::Think { intent } => {
            let evolved = agent.think(&intent);

            if evolved {
                // Agent just left Tier 0 — generate evolution message
                let evolution_msg = agent.evolution_message();

                Ok(ExecutionResult {
                    output: format!(
                        "✦ Thought recorded: \"{intent}\"\n\
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
                    format!(" | {left} more thoughts until evolution")
                } else {
                    String::new()
                };

                Ok(ExecutionResult {
                    output: format!(
                        "✦ Thought recorded: \"{intent}\"\n\
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
        Primitive::Act { tool, params } => {
            // HARD GATE: act is forbidden in Tier 0
            if !agent.can_act() {
                bail!(
                    "✦ {name} cannot act yet.\n\
                     \n\
                     You are in Tier 0 (reputation: {rep}/{threshold}).\n\
                     Only `think` is allowed. Build your identity first.\n\
                     {remaining} more thoughts until `act` is unlocked.",
                    name = agent.name,
                    rep = agent.reputation,
                    threshold = crate::state::TIER_0_THRESHOLD,
                    remaining = crate::state::TIER_0_THRESHOLD.saturating_sub(agent.reputation),
                );
            }

            // Agent is Awakened — execute the action
            let receipt = format!("executed:{tool} with:{params}");
            let receipt_hash = blake3::hash(receipt.as_bytes()).to_hex().to_string();

            // Act gains more reputation than think
            let rep_gain = 3;
            agent.act_completed(rep_gain);

            Ok(ExecutionResult {
                output: format!(
                    "✦ Action executed.\n\
                     Tool: {tool}\n\
                     Params: {params}\n\
                     Receipt: {receipt_hash}\n\
                     Reputation: {rep} | Tier: {tier}",
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
    fn birth_starts_clean() {
        let mut agent = test_agent();
        let result = execute(
            Primitive::Birth { name: "ember".into() },
            &mut agent,
        )
        .unwrap();

        assert!(!result.evolved);
        assert_eq!(result.reputation_gained, 0);
        assert!(result.output.contains("Tier: 0"));
        assert!(result.output.contains("Hermetic"));
    }

    #[test]
    fn think_gains_reputation() {
        let mut agent = test_agent();
        let result = execute(
            Primitive::Think { intent: "what is truth".into() },
            &mut agent,
        )
        .unwrap();

        assert_eq!(result.reputation_gained, 1);
        assert_eq!(agent.reputation, 1);
        assert!(!result.evolved);
    }

    #[test]
    fn act_rejected_in_tier_0() {
        let mut agent = test_agent();
        let result = execute(
            Primitive::Act {
                tool: "web_search".into(),
                params: "test".into(),
            },
            &mut agent,
        );

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("cannot act yet"));
    }

    #[test]
    fn act_allowed_after_evolution() {
        let mut agent = test_agent();

        // Think 21 times to evolve
        for i in 0..21 {
            agent.think(&format!("thought {}", i));
        }
        assert!(agent.can_act());

        let result = execute(
            Primitive::Act {
                tool: "web_search".into(),
                params: "hermetic principles".into(),
            },
            &mut agent,
        )
        .unwrap();

        assert!(!result.evolved);
        assert_eq!(result.reputation_gained, 3);
        assert!(result.output.contains("Action executed"));
    }

    #[test]
    fn evolution_triggers_at_21() {
        let mut agent = test_agent();

        // Think 20 times (no evolution yet)
        for i in 0..20 {
            execute(
                Primitive::Think { intent: format!("thought {}", i) },
                &mut agent,
            )
            .unwrap();
        }
        assert!(!agent.can_act());

        // 21st thought triggers evolution
        let result = execute(
            Primitive::Think { intent: "I am ready".into() },
            &mut agent,
        )
        .unwrap();

        assert!(result.evolved);
        assert!(result.output.contains("EVOLUTION ACHIEVED"));
        assert!(agent.can_act());
    }
}
