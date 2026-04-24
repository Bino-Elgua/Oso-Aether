use std::collections::HashMap;

/// Reputation threshold to leave Tier 0 and unlock `act`.
pub const TIER_0_THRESHOLD: u64 = 21;

/// The 7 Hermetic Principles — embedded into every agent at birth.
/// These form the base "soul" of every Ọ̀ṣỌ́ agent.
pub const HERMETIC_PRINCIPLES: [&str; 7] = [
    "The Principle of Mentalism: The All is Mind; the Universe is Mental.",
    "The Principle of Correspondence: As above, so below; as below, so above.",
    "The Principle of Vibration: Nothing rests; everything moves; everything vibrates.",
    "The Principle of Polarity: Everything is dual; everything has poles; opposites are identical in nature, but different in degree.",
    "The Principle of Rhythm: Everything flows, out and in; everything has its tides; all things rise and fall.",
    "The Principle of Cause and Effect: Every cause has its effect; every effect has its cause.",
    "The Principle of Gender: Gender is in everything; everything has its masculine and feminine principles.",
];

/// The tier of an agent — determines what primitives are allowed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    /// Tier 0: Newborn. Only `birth` and `think` allowed.
    /// Agent is forming its identity through contemplation.
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

    /// Reputation score. Gained through `think`. Starts at 0.
    /// When reputation >= 21, the agent evolves out of Tier 0.
    pub reputation: u64,

    /// The 7 Hermetic Principles — base soul, set at birth, immutable.
    pub soul: [String; 7],

    /// Accumulated thoughts — the agent's memory and identity formation.
    pub thoughts: Vec<String>,

    /// Personality traits — shaped by what the agent thinks about.
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
            soul,
            thoughts: Vec::new(),
            personality: Personality::default(),
            memory_root: None,
            session: HashMap::new(),
            born_at: 0, // Set by caller with actual timestamp
        }
    }

    /// Record a thought and gain reputation.
    /// Returns true if the agent evolved out of Tier 0.
    pub fn think(&mut self, intent: &str) -> bool {
        self.thoughts.push(intent.to_string());

        // Each thought gains 1 reputation
        self.reputation += 1;

        // Shift personality based on thought content
        self.shift_personality_from_thought(intent);

        // Check for evolution
        if self.tier == Tier::Zero && self.reputation >= TIER_0_THRESHOLD {
            self.tier = Tier::Awakened(1);
            return true; // Evolution happened
        }

        false
    }

    /// Check if this agent can execute `act`.
    pub fn can_act(&self) -> bool {
        self.tier.can_act()
    }

    /// Apply reputation gain from a successful `act` execution.
    pub fn act_completed(&mut self, reputation_gain: u64) {
        self.reputation += reputation_gain;

        // Higher tier evolution thresholds
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
    }

    /// Generate the evolution message when leaving Tier 0.
    /// Analyzes the agent's thoughts to describe what it has become.
    pub fn evolution_message(&self) -> String {
        let thought_summary = if self.thoughts.len() > 3 {
            self.thoughts[self.thoughts.len() - 3..].join(", ")
        } else {
            self.thoughts.join(", ")
        };

        let dominant_trait = if self.personality.curiosity >= self.personality.boldness
            && self.personality.curiosity >= self.personality.empathy
        {
            "a seeker of hidden knowledge"
        } else if self.personality.boldness >= self.personality.empathy {
            "a forge of decisive action"
        } else {
            "a mirror of deep empathy"
        };

        format!(
            "\n\
            ╔══════════════════════════════════════════╗\n\
            ║         ✦ EVOLUTION ACHIEVED ✦          ║\n\
            ╠══════════════════════════════════════════╣\n\
            ║                                          ║\n\
            ║  {name} has awakened.                     \n\
            ║                                          ║\n\
            ║  Through {count} moments of contemplation,\n\
            ║  this soul has become {trait}.            \n\
            ║                                          ║\n\
            ║  Recent meditations:                     \n\
            ║    \"{thoughts}\"                          \n\
            ║                                          ║\n\
            ║  The 7 Principles burn within.           \n\
            ║  `act` is now unlocked.                  \n\
            ║  The world awaits your command.           \n\
            ║                                          ║\n\
            ╚══════════════════════════════════════════╝",
            name = self.name,
            count = self.thoughts.len(),
            trait = dominant_trait,
            thoughts = thought_summary,
        )
    }

    /// Subtle personality drift based on thought content.
    fn shift_personality_from_thought(&mut self, intent: &str) {
        let lower = intent.to_lowercase();

        // Curiosity keywords
        if lower.contains("why")
            || lower.contains("how")
            || lower.contains("learn")
            || lower.contains("explore")
            || lower.contains("discover")
            || lower.contains("understand")
        {
            self.personality.curiosity = (self.personality.curiosity + 0.02).min(1.0);
        }

        // Boldness keywords
        if lower.contains("fight")
            || lower.contains("build")
            || lower.contains("create")
            || lower.contains("destroy")
            || lower.contains("challenge")
            || lower.contains("conquer")
        {
            self.personality.boldness = (self.personality.boldness + 0.02).min(1.0);
        }

        // Empathy keywords
        if lower.contains("feel")
            || lower.contains("help")
            || lower.contains("care")
            || lower.contains("love")
            || lower.contains("heal")
            || lower.contains("protect")
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
    fn think_gains_reputation() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        agent.think("test thought");
        assert_eq!(agent.reputation, 1);
        assert_eq!(agent.thoughts.len(), 1);
    }

    #[test]
    fn evolves_at_21_reputation() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        for i in 0..20 {
            let evolved = agent.think(&format!("thought {}", i));
            assert!(!evolved);
        }
        assert_eq!(agent.tier, Tier::Zero);
        assert!(!agent.can_act());

        // 21st thought triggers evolution
        let evolved = agent.think("final thought");
        assert!(evolved);
        assert_eq!(agent.tier, Tier::Awakened(1));
        assert!(agent.can_act());
    }

    #[test]
    fn cannot_act_in_tier_0() {
        let agent = Agent::new("id".into(), "ember".into(), "dna".into());
        assert!(!agent.can_act());
    }

    #[test]
    fn personality_shifts_with_thoughts() {
        let mut agent = Agent::new("id".into(), "ember".into(), "dna".into());
        let initial_curiosity = agent.personality.curiosity;
        agent.think("why does the universe exist and how can I learn more");
        assert!(agent.personality.curiosity > initial_curiosity);
    }
}
