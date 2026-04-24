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

    // ── Slash command events ──

    /// /status — show agent state.
    StatusRequested {
        name: String,
        reputation: u64,
        tier: u8,
        alignment: String,
        personality: PersonalitySnapshot,
        thought_count: usize,
        action_count: usize,
    },

    /// /tools — show available/locked tools.
    ToolsRequested {
        unlocked: Vec<String>,
        locked: Vec<(String, u64)>,
    },

    /// /help — list available slash commands.
    HelpRequested,

    /// /personality — detailed personality breakdown.
    PersonalityRequested {
        name: String,
        personality: PersonalitySnapshot,
        agent_type: AgentType,
    },

    /// /private — thought recorded as private.
    PrivateThoughtRecorded {
        message: String,
        thought_count: usize,
    },

    /// /publish — thought marked for public wiki (The Garden).
    PublishRequested {
        message: String,
        thought_count: usize,
    },

    /// /clear — conversation history cleared.
    ConversationCleared,

    /// /export — private markdown export generated.
    ExportGenerated {
        name: String,
        thought_count: usize,
        action_count: usize,
    },

    /// A public wiki entry was updated (Garden event).
    GardenEntryUpdated {
        agent_id: String,
        content_hash: String,
    },

    /// Agent ownership was transferred.
    OwnershipTransferred {
        agent_id: String,
        from_owner: String,
        to_owner: String,
    },
}

/// Snapshot of personality traits for events.
#[derive(Debug, Clone)]
pub struct PersonalitySnapshot {
    pub curiosity: f64,
    pub boldness: f64,
    pub empathy: f64,
}
