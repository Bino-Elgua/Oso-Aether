use std::collections::HashMap;

/// Reputation threshold to leave Tier 0 and unlock `act`.
pub const TIER_0_THRESHOLD: u64 = 21;

/// Default cost to birth a new agent, in MIST (1 SUI = 1_000_000_000 MIST).
/// This is the default — the frontend can pass a different amount when
/// calling the birth endpoint. The Move contract enforces the final price
/// on-chain, so this constant is the minimum for local validation only.
/// To change the birth price, update this value or override it from the
/// frontend/API layer.
pub const BIRTH_COST_MIST: u64 = 100_000_000; // 0.1 SUI

/// Represents a confirmed SUI payment for birth.
/// In production, this will be validated against the Sui blockchain
/// via the Move contract at `contracts/sources/pet.move`.
#[derive(Debug, Clone)]
pub struct PaymentConfirmation {
    /// The Sui transaction digest that proves payment.
    pub tx_digest: String,
    /// Amount paid in MIST.
    pub amount_mist: u64,
    /// The sender's Sui address.
    pub sender: String,
}

impl PaymentConfirmation {
    /// Validate that the payment meets the birth cost requirement.
    pub fn is_valid_for_birth(&self) -> bool {
        self.amount_mist >= BIRTH_COST_MIST && !self.tx_digest.is_empty()
    }
}

/// The 7 Hermetic Principles — embedded into every agent at birth.
/// These form the base "soul" of every Ọ̀ṣỌ́ agent.
/// Each principle maps to real behavior in the code.
pub const HERMETIC_PRINCIPLES: [&str; 7] = [
    // 0: MENTALISM — Tier 0 is pure thought. Identity is formed before action.
    "The Principle of Mentalism: The All is Mind; the Universe is Mental.",
    // 1: CORRESPONDENCE — Personality built in think shapes how the agent acts.
    "The Principle of Correspondence: As above, so below; as below, so above.",
    // 2: VIBRATION — Reputation is never static. It can rise and fall.
    "The Principle of Vibration: Nothing rests; everything moves; everything vibrates.",
    // 3: POLARITY — The agent can walk toward light or shadow. Both paths are valid.
    "The Principle of Polarity: Everything is dual; everything has poles; opposites are identical in nature, but different in degree.",
    // 4: RHYTHM — Reputation gains slow in higher tiers. Misuse causes decay.
    "The Principle of Rhythm: Everything flows, out and in; everything has its tides; all things rise and fall.",
    // 5: CAUSE AND EFFECT — Every act produces a permanent, immutable receipt.
    "The Principle of Cause and Effect: Every cause has its effect; every effect has its cause.",
    // 6: GENDER — think = receptive (small rep). act = active (strong rep).
    "The Principle of Gender: Gender is in everything; everything has its masculine and feminine principles.",
];

/// The tier of an agent — determines what primitives are allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    /// Tier 0: Newborn. Only `birth` and `think` allowed.
    /// [MENTALISM] The agent exists only in Mind. No action permitted.
    Zero,
    /// Tier 1+: Awakened. All three primitives unlocked.
    Awakened(u8),
}

impl Tier {
    /// Can this tier execute `act`?
    pub fn can_act(&self) -> bool {
        matches!(self, Tier::Awakened(_))
    }

    /// Display tier number.
    pub fn level(&self) -> u8 {
        match self {
            Tier::Zero => 0,
            Tier::Awakened(n) => *n,
        }
    }
}

/// The alignment of an agent — shaped by its thoughts and actions.
/// [POLARITY] Both light and shadow are valid paths.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Alignment {
    Light,   // helpful, constructive, protective
    Neutral, // balanced, undefined
    Shadow,  // destructive, selfish, aggressive
}

/// A permanent log entry for every `act` execution.
/// [CAUSE AND EFFECT] Every action produces an immutable receipt.
#[derive(Debug, Clone)]
pub struct ActionReceipt {
    pub tool: String,
    pub params: String,
    pub receipt_hash: String,
    pub reputation_at_time: u64,
    pub tier_at_time: u8,
    pub timestamp: u64,
}

/// A logged reputation decay event.
#[derive(Debug, Clone)]
pub struct DecayEvent {
    pub amount: u64,
    pub reason: String,
    pub reputation_before: u64,
    pub reputation_after: u64,
    pub tier_before: u8,
    pub tier_after: u8,
}

/// A single Ọ̀ṣỌ́ agent — an independent, persistent digital being.
///
/// Every agent starts at reputation 0, Tier 0.
/// It never inherits reputation, tools, or memory from any other agent.
#[derive(Debug, Clone)]
pub struct Agent {
    /// Unique identifier for this agent.
    pub id: String,

    /// The agent's given name (from `birth "name"`).
    pub name: String,

    /// 86-character hex DNA fingerprint — unique identity marker.
    pub dna: String,

    /// Current tier. Starts at Tier::Zero, evolves to Awakened(1+).
    pub tier: Tier,

    /// Reputation score. Can increase AND decrease.
    /// [VIBRATION] Nothing rests — reputation is always in motion.
    pub reputation: u64,

    /// [POLARITY] The agent's moral alignment, shaped by thought content.
    pub alignment: Alignment,
    pub light_score: i64,
    pub shadow_score: i64,

    /// The 7 Hermetic Principles — base soul, set at birth, immutable.
    pub soul: [String; 7],

    /// Accumulated thoughts — the agent's memory and identity formation.
    /// [MENTALISM] All identity is built here before action is possible.
    pub thoughts: Vec<String>,

    /// [CAUSE AND EFFECT] Permanent log of every action ever taken.
    pub action_log: Vec<ActionReceipt>,

    /// Log of all reputation decay events with reasons.
    pub decay_log: Vec<DecayEvent>,

    /// Personality traits — shaped by what the agent thinks about.
    /// [CORRESPONDENCE] These traits influence how `act` behaves.
    pub personality: Personality,

    /// Walrus content ID for permanent memory storage.
    pub memory_root: Option<String>,

    /// Arbitrary session data for the interpreter.
    pub session: HashMap<String, String>,

    /// Timestamp of birth.
    pub born_at: u64,
}

#[derive(Debug, Clone)]
pub struct Personality {
    pub curiosity: f64,
    pub boldness: f64,
    pub empathy: f64,
}

impl Default for Personality {
    fn default() -> Self {
        Self {
            curiosity: 0.5,
            boldness: 0.5,
            empathy: 0.5,
        }
    }
}

impl Agent {
    /// Create a new agent. Always starts at reputation 0, Tier 0.
    /// The 7 Hermetic Principles are embedded as its soul.
    pub fn new(id: String, name: String, dna: String) -> Self {
        let soul = HERMETIC_PRINCIPLES.map(|p| p.to_string());

        Self {
            id,
            name,
            dna,
            tier: Tier::Zero,
            reputation: 0,
            alignment: Alignment::Neutral,
            light_score: 0,
            shadow_score: 0,
            soul,
            thoughts: Vec::new(),
            action_log: Vec::new(),
            decay_log: Vec::new(),
            personality: Personality::default(),
            memory_root: None,
            session: HashMap::new(),
            born_at: 0,
        }
    }

    /// [MENTALISM + GENDER] Record a thought. Receptive energy — small rep gain.
    /// think = feminine/receptive principle. Quiet accumulation.
    /// Returns true if the agent evolved out of Tier 0.
    pub fn think(&mut self, intent: &str) -> bool {
        self.thoughts.push(intent.to_string());

        // [GENDER] think is receptive — gains 1 reputation (small, steady)
        self.reputation += 1;

        // [POLARITY] Shift alignment based on thought content
        self.shift_alignment_from_thought(intent);

        // Shift personality based on thought content
        self.shift_personality_from_thought(intent);

        // [MENTALISM] Check for evolution out of pure thought
        if self.tier == Tier::Zero && self.reputation >= TIER_0_THRESHOLD {
            self.tier = Tier::Awakened(1);
            return true;
        }

        false
    }

    /// Check if this agent can execute `act`.
    pub fn can_act(&self) -> bool {
        self.tier.can_act()
    }

    /// [GENDER + RHYTHM + CAUSE AND EFFECT]
    /// Apply reputation from a completed action.
    /// act = masculine/active principle. Strong reputation impact.
    ///
    /// [RHYTHM] Reputation gains diminish at higher tiers.
    /// [CORRESPONDENCE] Personality modifies the actual reputation gained.
    pub fn act_completed(&mut self, tool: &str, params: &str, receipt_hash: String) -> u64 {
        // [GENDER] act is active — base gain is 5 (much stronger than think's 1)
        let base_gain: u64 = 5;

        // [RHYTHM] Higher tiers make reputation harder to earn
        let tier_dampening = match self.tier {
            Tier::Zero => 1.0,       // Should not happen, but safe
            Tier::Awakened(1) => 1.0,
            Tier::Awakened(2) => 0.8,
            Tier::Awakened(3) => 0.6,
            Tier::Awakened(4) => 0.4,
            Tier::Awakened(_) => 0.2,
        };

        // [CORRESPONDENCE] Personality built in Tier 0 influences action outcomes.
        // Bold agents gain slightly more from action. Curious agents slightly less
        // (they're thinkers at heart). Empathetic agents gain steady amounts.
        let personality_modifier = 1.0
            + (self.personality.boldness - 0.5) * 0.2
            - (self.personality.curiosity - 0.5) * 0.1;

        let actual_gain = ((base_gain as f64) * tier_dampening * personality_modifier).max(1.0) as u64;
        self.reputation += actual_gain;

        // [CAUSE AND EFFECT] Permanent receipt — cannot be erased
        self.action_log.push(ActionReceipt {
            tool: tool.to_string(),
            params: params.to_string(),
            receipt_hash,
            reputation_at_time: self.reputation,
            tier_at_time: self.tier.level(),
            timestamp: 0, // Caller sets real timestamp
        });

        // Check tier evolution
        if let Tier::Awakened(level) = self.tier {
            let next_threshold = match level {
                1 => 100,
                2 => 500,
                3 => 2000,
                4 => 10000,
                _ => u64::MAX,
            };
            if self.reputation >= next_threshold && level < 5 {
                self.tier = Tier::Awakened(level + 1);
            }
        }

        actual_gain
    }

    /// Decrease reputation with a logged reason.
    /// Called when the agent is misused, idle too long, or acts destructively.
    /// Reputation cannot go below 0. If it drops below 21, act is NOT re-locked
    /// (evolution is permanent — you can't un-learn what you've learned).
    pub fn decay_reputation(&mut self, amount: u64, reason: &str) {
        let rep_before = self.reputation;
        let tier_before = self.tier.level();

        self.reputation = self.reputation.saturating_sub(amount);

        // Tier can decrease if reputation drops far enough
        if let Tier::Awakened(level) = self.tier {
            let current_threshold = match level {
                1 => 0,    // Can't drop below Awakened(1) once evolved
                2 => 100,
                3 => 500,
                4 => 2000,
                5 => 10000,
                _ => 0,
            };
            // Drop tier if reputation fell below its threshold (but never back to Zero)
            if level > 1 && self.reputation < current_threshold {
                self.tier = Tier::Awakened(level - 1);
            }
        }

        self.decay_log.push(DecayEvent {
            amount,
            reason: reason.to_string(),
            reputation_before: rep_before,
            reputation_after: self.reputation,
            tier_before,
            tier_after: self.tier.level(),
        });
    }

    /// Generate the evolution message when leaving Tier 0.
    ///
    /// Analyzes the agent's accumulated thoughts to determine what kind
    /// of agent it has become, then presents a simple, warm message.
    pub fn evolution_message(&self) -> String {
        // Analyze thought themes to determine agent type
        let mut curiosity_count = 0u32;
        let mut boldness_count = 0u32;
        let mut empathy_count = 0u32;

        for thought in &self.thoughts {
            let lower = thought.to_lowercase();
            if lower.contains("why") || lower.contains("how") || lower.contains("learn")
                || lower.contains("explore") || lower.contains("understand") || lower.contains("discover")
            {
                curiosity_count += 1;
            }
            if lower.contains("build") || lower.contains("create") || lower.contains("fight")
                || lower.contains("conquer") || lower.contains("challenge") || lower.contains("power")
            {
                boldness_count += 1;
            }
            if lower.contains("help") || lower.contains("heal") || lower.contains("protect")
                || lower.contains("care") || lower.contains("love") || lower.contains("feel")
            {
                empathy_count += 1;
            }
        }

        // Determine the agent type and a flavor sentence based on dominant personality
        let (agent_type, flavor) = if curiosity_count >= boldness_count && curiosity_count >= empathy_count {
            ("Research Agent",
             "You keep asking big questions, and honestly, I love that.")
        } else if boldness_count >= empathy_count {
            ("Builder Agent",
             "You want to make things happen — I can feel it.")
        } else {
            ("Support Agent",
             "You care about people. That's what I'll focus on too.")
        };

        format!(
            "Something clicked. After talking with you, I feel like I \
             understand what you're looking for now. {flavor}\n\
             \n\
             I've become a {agent_type} — and I'm ready to start taking \
             action. My first ability, Web Search, is now unlocked.",
            flavor = flavor,
            agent_type = agent_type,
        )
    }

    /// [POLARITY] Shift alignment based on thought content.
    /// Both paths are valid — the system does not judge.
    fn shift_alignment_from_thought(&mut self, intent: &str) {
        let lower = intent.to_lowercase();

        // Light words
        if lower.contains("help") || lower.contains("heal") || lower.contains("protect")
            || lower.contains("create") || lower.contains("love") || lower.contains("build")
            || lower.contains("grow") || lower.contains("care") || lower.contains("nurture")
        {
            self.light_score += 1;
        }

        // Shadow words
        if lower.contains("destroy") || lower.contains("hate") || lower.contains("kill")
            || lower.contains("steal") || lower.contains("exploit") || lower.contains("dominate")
            || lower.contains("deceive") || lower.contains("manipulate") || lower.contains("corrupt")
        {
            self.shadow_score += 1;
        }

        // Update alignment
        let diff = self.light_score - self.shadow_score;
        self.alignment = if diff > 3 {
            Alignment::Light
        } else if diff < -3 {
            Alignment::Shadow
        } else {
            Alignment::Neutral
        };
    }

    /// Subtle personality drift based on thought content.
    fn shift_personality_from_thought(&mut self, intent: &str) {
        let lower = intent.to_lowercase();

        if lower.contains("why") || lower.contains("how") || lower.contains("learn")
            || lower.contains("explore") || lower.contains("discover") || lower.contains("understand")
        {
            self.personality.curiosity = (self.personality.curiosity + 0.02).min(1.0);
        }

        if lower.contains("fight") || lower.contains("build") || lower.contains("create")
            || lower.contains("destroy") || lower.contains("challenge") || lower.contains("conquer")
            || lower.contains("power")
        {
            self.personality.boldness = (self.personality.boldness + 0.02).min(1.0);
        }

        if lower.contains("feel") || lower.contains("help") || lower.contains("care")
            || lower.contains("love") || lower.contains("heal") || lower.contains("protect")
        {
            self.personality.empathy = (self.personality.empathy + 0.02).min(1.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_agent_starts_at_tier_0() {
        let agent = Agent::new("id".into(), "ember".into(), "dna".into());
        assert_eq!(agent.tier, Tier::Zero);
        assert_eq!(agent.reputation, 0);
        assert_eq!(agent.alignment, Alignment::Neutral);
        assert!(agent.action_log.is_empty());
        assert!(!agent.can_act());
    }

    #[test]
    fn soul_contains_7_principles() {
        let agent = Agent::new("id".into(), "ember".into(), "dna".into());
        assert_eq!(agent.soul.len(), 7);
        assert!(agent.soul[0].contains("Mentalism"));
        assert!(agent.soul[6].contains("Gender"));
    }

    #[test]
    fn think_gains_1_reputation_gender_principle() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        agent.think("test thought");
        assert_eq!(agent.reputation, 1); // [GENDER] receptive = small gain
        assert_eq!(agent.thoughts.len(), 1);
    }

    #[test]
    fn evolves_at_21_reputation() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        for i in 0..20 {
            assert!(!agent.think(&format!("thought {}", i)));
        }
        assert_eq!(agent.tier, Tier::Zero);
        assert!(agent.think("final thought")); // 21st — evolution
        assert_eq!(agent.tier, Tier::Awakened(1));
        assert!(agent.can_act());
    }

    #[test]
    fn act_gains_more_than_think_gender_principle() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        // Evolve first
        for i in 0..21 { agent.think(&format!("t{}", i)); }
        let rep_before = agent.reputation;
        let gained = agent.act_completed("tool", "params", "hash".into());
        assert!(gained > 1); // [GENDER] active = stronger than receptive
        assert!(agent.reputation > rep_before);
    }

    #[test]
    fn act_creates_permanent_receipt_cause_and_effect() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        for i in 0..21 { agent.think(&format!("t{}", i)); }
        assert!(agent.action_log.is_empty());
        agent.act_completed("web_search", "test query", "hash123".into());
        assert_eq!(agent.action_log.len(), 1);
        assert_eq!(agent.action_log[0].tool, "web_search");
        assert_eq!(agent.action_log[0].receipt_hash, "hash123");
    }

    #[test]
    fn reputation_can_decrease_vibration() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        for i in 0..21 { agent.think(&format!("t{}", i)); }
        assert_eq!(agent.reputation, 21);
        agent.decay_reputation(5, "test decay");
        assert_eq!(agent.reputation, 16);
    }

    #[test]
    fn reputation_decay_cannot_go_below_zero() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        agent.think("test");
        agent.decay_reputation(100, "test underflow");
        assert_eq!(agent.reputation, 0);
    }

    #[test]
    fn tier_drops_on_heavy_decay_rhythm() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        // Get to tier 2 (reputation 100+)
        for i in 0..21 { agent.think(&format!("t{}", i)); }
        for _ in 0..20 { agent.act_completed("t", "p", "h".into()); }
        assert!(agent.reputation >= 100);
        assert_eq!(agent.tier, Tier::Awakened(2));

        // [RHYTHM] Decay back below threshold
        agent.decay_reputation(agent.reputation - 50, "heavy decay test");
        assert_eq!(agent.tier, Tier::Awakened(1)); // Dropped a tier
    }

    #[test]
    fn evolution_never_reverts_to_zero() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        for i in 0..21 { agent.think(&format!("t{}", i)); }
        assert_eq!(agent.tier, Tier::Awakened(1));
        agent.decay_reputation(100, "test permanent evolution"); // Drop rep to 0
        assert_eq!(agent.reputation, 0);
        assert_eq!(agent.tier, Tier::Awakened(1)); // Still Awakened — can't go back to Zero
        assert!(agent.can_act());
    }

    #[test]
    fn polarity_shifts_alignment() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        agent.think("I want to help people and heal the world");
        agent.think("I want to protect the innocent and care for others");
        agent.think("I want to love and nurture all living things");
        agent.think("I will build something beautiful and grow");
        assert_eq!(agent.alignment, Alignment::Light);

        let mut shadow = Agent::new("id2".into(), "void".into(), "dna2".into());
        shadow.think("I want to destroy everything and dominate");
        shadow.think("I will exploit the weak and steal their power");
        shadow.think("I want to kill and corrupt all things");
        shadow.think("I will manipulate and deceive everyone");
        assert_eq!(shadow.alignment, Alignment::Shadow);
    }

    #[test]
    fn personality_shapes_act_reputation_correspondence() {
        // Bold agent should gain more from act than curious agent
        let mut bold = Agent::new("b".into(), "bold".into(), "d".into());
        bold.personality.boldness = 0.9;
        bold.personality.curiosity = 0.2;
        for i in 0..21 { bold.think(&format!("t{}", i)); }

        let mut curious = Agent::new("c".into(), "curious".into(), "d".into());
        curious.personality.boldness = 0.2;
        curious.personality.curiosity = 0.9;
        for i in 0..21 { curious.think(&format!("t{}", i)); }

        let bold_gain = bold.act_completed("t", "p", "h".into());
        let curious_gain = curious.act_completed("t", "p", "h".into());
        assert!(bold_gain > curious_gain); // [CORRESPONDENCE] personality matters
    }

    #[test]
    fn rhythm_dampens_higher_tiers() {
        let mut agent = Agent::new("id".into(), "test".into(), "dna".into());
        for i in 0..21 { agent.think(&format!("t{}", i)); }

        // Tier 1 gain
        let t1_gain = agent.act_completed("t", "p", "h".into());

        // Push to tier 3
        agent.reputation = 500;
        agent.tier = Tier::Awakened(3);
        let t3_gain = agent.act_completed("t", "p", "h".into());

        assert!(t1_gain > t3_gain); // [RHYTHM] harder to gain at higher tiers
    }

    #[test]
    fn decay_logs_reason() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        for i in 0..21 { agent.think(&format!("t{}", i)); }
        assert!(agent.decay_log.is_empty());

        agent.decay_reputation(5, "idle too long");
        assert_eq!(agent.decay_log.len(), 1);
        assert_eq!(agent.decay_log[0].reason, "idle too long");
        assert_eq!(agent.decay_log[0].reputation_before, 21);
        assert_eq!(agent.decay_log[0].reputation_after, 16);
        assert_eq!(agent.decay_log[0].amount, 5);

        agent.decay_reputation(3, "destructive action");
        assert_eq!(agent.decay_log.len(), 2);
        assert_eq!(agent.decay_log[1].reason, "destructive action");
    }

    #[test]
    fn payment_validation() {
        use crate::state::PaymentConfirmation;

        let valid = PaymentConfirmation {
            tx_digest: "abc123".into(),
            amount_mist: 100_000_000,
            sender: "0xtest".into(),
        };
        assert!(valid.is_valid_for_birth());

        let too_low = PaymentConfirmation {
            tx_digest: "abc123".into(),
            amount_mist: 1000,
            sender: "0xtest".into(),
        };
        assert!(!too_low.is_valid_for_birth());

        let no_digest = PaymentConfirmation {
            tx_digest: "".into(),
            amount_mist: 100_000_000,
            sender: "0xtest".into(),
        };
        assert!(!no_digest.is_valid_for_birth());
    }

    #[test]
    fn evolution_message_is_natural_and_warm() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        for _ in 0..21 { agent.think("why does the universe exist and how can I learn"); }
        let msg = agent.evolution_message();
        assert!(msg.contains("Research Agent")); // Curiosity-dominant thoughts
        assert!(msg.contains("Web Search"));
        assert!(msg.contains("understand"));
        assert!(msg.contains("big questions")); // Flavor for research type
        // Must NOT contain system terms or esoteric language
        assert!(!msg.contains("Principle"));
        assert!(!msg.contains("CEREMONY"));
        assert!(!msg.contains("Hermetic"));
        assert!(!msg.contains("Reputation"));
        assert!(!msg.contains("Tier"));
    }

    #[test]
    fn evolution_message_builder_type() {
        let mut agent = Agent::new("id".into(), "forge".into(), "dna".into());
        for _ in 0..21 { agent.think("I want to build and create powerful things"); }
        let msg = agent.evolution_message();
        assert!(msg.contains("Builder Agent"));
        assert!(msg.contains("make things happen"));
    }

    #[test]
    fn evolution_message_support_type() {
        let mut agent = Agent::new("id".into(), "heart".into(), "dna".into());
        for _ in 0..21 { agent.think("I want to help people and care for them"); }
        let msg = agent.evolution_message();
        assert!(msg.contains("Support Agent"));
        assert!(msg.contains("care about people"));
    }
}
