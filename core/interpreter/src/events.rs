/// Structured execution events.
///
/// These are the source of truth from the Rust executor. They contain
/// only verified system state — no natural language, no flavor text.
/// The ResponseGenerator layer converts these into user-facing messages.
///
/// This separation ensures the LLM can generate warm, natural responses
/// without ever deciding what tools are unlocked or what the agent can do.

/// The agent type determined by thought analysis.
#[derive(Debug, Clone, PartialEq)]
pub enum AgentType {
    Research,
    Builder,
    Support,
}

impl AgentType {
    pub fn label(&self) -> &'static str {
        match self {
            AgentType::Research => "Research Agent",
            AgentType::Builder => "Builder Agent",
            AgentType::Support => "Support Agent",
        }
    }
}

/// A structured event produced by the executor.
///
/// Contains only facts — no prose. The response layer decides how
/// to present these facts to the user.
#[derive(Debug, Clone)]
pub enum ExecutionEvent {
    /// Agent was successfully born.
    BirthSuccess {
        name: String,
    },

    /// A thought was recorded (no evolution).
    ThinkReceived {
        intent: String,
        /// How many thoughts the agent has now (after this one).
        thought_count: usize,
        /// Whether the agent is still in Tier 0.
        in_tier_zero: bool,
    },

    /// The agent evolved out of Tier 0.
    Evolved {
        name: String,
        agent_type: AgentType,
        /// Tools that just became available.
        new_unlocked_tools: Vec<String>,
        thought_count: usize,
    },

    /// An action was successfully executed.
    ActionCompleted {
        tool: String,
        params: String,
        receipt_hash: String,
        receipt_number: usize,
        reputation_gained: u64,
    },

    /// An action was blocked because the agent hasn't evolved yet.
    ActionBlockedTier0 {
        name: String,
        current_rep: u64,
        required_rep: u64,
    },

    /// An action was blocked because the tool is locked at this reputation.
    ActionBlockedToolLocked {
        tool: String,
        current_rep: u64,
        required_rep: u64,
    },

    /// An action was blocked because the tool doesn't exist.
    ActionBlockedUnknownTool {
        tool: String,
        available_tools: Vec<String>,
    },
}
