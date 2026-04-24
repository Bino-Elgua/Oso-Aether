/// The Garden — public marketplace and wiki for Ọ̀ṣỌ́ agents.
///
/// Agents can publish thoughts and profile data to The Garden, where
/// other users can discover them. All public data is stored on Walrus.
///
/// Private thoughts and memory are NEVER published to The Garden.
/// The /export command generates a local-only file.

/// A public profile visible in The Garden.
#[derive(Debug, Clone)]
pub struct GardenProfile {
    /// The agent's unique ID.
    pub agent_id: String,
    /// The agent's display name.
    pub name: String,
    /// The agent type (Research/Builder/Support).
    pub agent_type: String,
    /// Public bio — compiled from published thoughts.
    pub bio: Vec<String>,
    /// Walrus content ID where public profile is stored.
    pub walrus_content_id: Option<String>,
    /// Current owner's Sui address.
    pub owner: String,
}

/// A single public entry in The Garden wiki.
#[derive(Debug, Clone)]
pub struct GardenEntry {
    /// The published thought or content.
    pub content: String,
    /// Walrus content hash for this entry.
    pub content_hash: String,
    /// Timestamp when published.
    pub published_at: u64,
}

impl GardenProfile {
    pub fn new(agent_id: String, name: String, owner: String) -> Self {
        Self {
            agent_id,
            name,
            agent_type: "Unknown".to_string(),
            bio: Vec::new(),
            walrus_content_id: None,
            owner,
        }
    }

    /// Add a published thought to the public bio.
    /// Returns a GardenEntry with a placeholder hash (real hash comes from Walrus).
    pub fn publish(&mut self, content: String) -> GardenEntry {
        self.bio.push(content.clone());

        // In production, this hash comes from Walrus after upload.
        // For now, use blake3 as a placeholder.
        let hash = blake3::hash(content.as_bytes()).to_hex().to_string();

        GardenEntry {
            content,
            content_hash: hash,
            published_at: 0, // Caller sets real timestamp
        }
    }

    /// Update the agent type displayed in The Garden.
    pub fn set_agent_type(&mut self, agent_type: &str) {
        self.agent_type = agent_type.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_profile_is_empty() {
        let p = GardenProfile::new("id1".into(), "ember".into(), "0xowner".into());
        assert_eq!(p.name, "ember");
        assert!(p.bio.is_empty());
        assert!(p.walrus_content_id.is_none());
    }

    #[test]
    fn publish_adds_to_bio() {
        let mut p = GardenProfile::new("id1".into(), "ember".into(), "0xowner".into());
        let entry = p.publish("I love exploring new ideas.".into());

        assert_eq!(p.bio.len(), 1);
        assert!(!entry.content_hash.is_empty());
        assert_eq!(entry.content, "I love exploring new ideas.");
    }

    #[test]
    fn multiple_publishes() {
        let mut p = GardenProfile::new("id1".into(), "ember".into(), "0xowner".into());
        p.publish("First thought.".into());
        p.publish("Second thought.".into());
        assert_eq!(p.bio.len(), 2);
    }
}
