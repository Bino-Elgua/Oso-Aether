use std::collections::HashMap;

/// Agent state maintained across the execution lifecycle.
#[derive(Debug, Clone)]
pub struct AgentState {
    pub agent_id: Option<String>,
    pub name: Option<String>,
    pub tier: u8,
    pub xp: u64,
    pub personality: Personality,
    pub memory_root: Option<String>,
    pub session_data: HashMap<String, String>,
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

impl AgentState {
    pub fn new() -> Self {
        Self {
            agent_id: None,
            name: None,
            tier: 1,
            xp: 0,
            personality: Personality::default(),
            memory_root: None,
            session_data: HashMap::new(),
        }
    }

    /// Apply XP gain and check for tier evolution.
    /// Returns true if tier changed.
    pub fn apply_growth(&mut self, xp_gain: u64) -> bool {
        self.xp += xp_gain;
        let old_tier = self.tier;

        self.tier = match self.xp {
            xp if xp >= 10000 => 5,
            xp if xp >= 2000 => 4,
            xp if xp >= 500 => 3,
            xp if xp >= 100 => 2,
            _ => 1,
        };

        self.tier != old_tier
    }

    /// Shift personality traits based on interaction type.
    pub fn shift_personality(&mut self, curiosity_delta: f64, boldness_delta: f64, empathy_delta: f64) {
        self.personality.curiosity = (self.personality.curiosity + curiosity_delta).clamp(0.0, 1.0);
        self.personality.boldness = (self.personality.boldness + boldness_delta).clamp(0.0, 1.0);
        self.personality.empathy = (self.personality.empathy + empathy_delta).clamp(0.0, 1.0);
    }
}
