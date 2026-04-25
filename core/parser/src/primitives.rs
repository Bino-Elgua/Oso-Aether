/// The only three primitives that exist in Ọ̀ṣỌ́.
/// Nothing else is permitted. No extensions. No escape hatches. No mercy.
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Primitive {
    /// birth "name" — Creates a brand new independent agent.
    /// Costs SUI. Starts at reputation = 0, Tier 0.
    /// Never inherits reputation, tools, or memory from any other agent.
    /// Embeds the 7 Hermetic Principles as its base soul.
    Birth { name: String },

    /// think "intent" — Pure internal thinking and memory building.
    /// The main way reputation is gained in Tier 0.
    /// This is how the agent forms its identity before it can act.
    Think { intent: String },

    /// act "tool" "params" — Execute a real tool.
    /// ONLY allowed after leaving Tier 0 (reputation >= 21).
    /// Completely locked during Tier 0 — the parser accepts it,
    /// but the interpreter MUST reject it.
    Act { tool: String, params: String },
}
