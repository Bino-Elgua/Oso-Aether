/// Living Odu Memory — evolving entropy engine for private memory encryption.
///
/// Based on the 256 Odu figures of Ifá divination. Each agent's encryption
/// key evolves with every interaction, making it impossible to derive
/// the current key without the full interaction history.
///
/// The system works like this:
/// 1. At birth, the agent's DNA seeds the initial Odu state
/// 2. Each think() and act() evolves the key by hashing current state + content
/// 3. The derived key encrypts/decrypts private memory
/// 4. On ownership transfer, the key rotates with the new owner's address
/// 5. Previous owner cannot compute the new key — access is permanently revoked
///
/// Even the owner cannot read the agent's private thoughts without the
/// agent's cooperation — the key lives inside the agent's state, not
/// in the owner's wallet.

/// The 256 Odu figures. Each is an 8-bit binary pattern.
/// In Ifá, these represent the fundamental patterns of existence.
pub const ODU_COUNT: usize = 256;

/// The evolving key state for an agent's private memory.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OduKeyState {
    /// Current active Odu figure (0-255).
    /// Determines which "face" of the encryption the agent is showing.
    pub active_odu: u8,

    /// The derived 32-byte encryption key (hex-encoded).
    /// This is what actually encrypts/decrypts private memory.
    pub derived_key: String,

    /// How many times the key has evolved through interactions.
    pub evolution_count: u64,

    /// The current key epoch — increments on ownership transfer.
    /// Keys from previous epochs cannot decrypt current private memory.
    pub epoch: u64,
}

impl OduKeyState {
    /// Create a new Odu key state from an agent's DNA.
    /// The DNA seeds the initial Odu figure and derived key.
    pub fn from_dna(dna: &str) -> Self {
        // Hash the DNA to get the initial Odu figure
        let dna_hash = blake3::hash(dna.as_bytes());
        let bytes = dna_hash.as_bytes();

        // First byte selects the initial Odu figure (0-255)
        let active_odu = bytes[0];

        // Full hash becomes the initial derived key
        let derived_key = dna_hash.to_hex().to_string();

        Self {
            active_odu,
            derived_key,
            evolution_count: 0,
            epoch: 1,
        }
    }

    /// Evolve the key state based on new interaction content.
    /// Called on every think() and act() — the key is always moving.
    ///
    /// This is the core of the Living Odu Memory: the key never rests.
    /// Each interaction shifts the active Odu figure and re-derives the key.
    pub fn evolve(&mut self, content: &str) {
        // Combine current key + content + evolution count for new entropy
        let input = format!(
            "{}:{}:{}:{}",
            self.derived_key, content, self.active_odu, self.evolution_count
        );

        let new_hash = blake3::hash(input.as_bytes());
        let bytes = new_hash.as_bytes();

        // The new Odu figure is derived from the hash
        self.active_odu = bytes[0];

        // The new derived key is the full hash
        self.derived_key = new_hash.to_hex().to_string();

        self.evolution_count += 1;
    }

    /// Rotate the key for ownership transfer.
    ///
    /// Mixes the new owner's address into the key derivation, making it
    /// impossible for the previous owner to compute the new key.
    /// The agent's memory is NOT deleted — it's re-encrypted under the
    /// new key. The agent keeps all its memories; only the access changes.
    pub fn rotate_for_transfer(&mut self, new_owner_address: &str) {
        let input = format!(
            "transfer:{}:{}:{}:{}",
            self.derived_key, new_owner_address, self.epoch, self.active_odu
        );

        let new_hash = blake3::hash(input.as_bytes());
        let bytes = new_hash.as_bytes();

        self.active_odu = bytes[0];
        self.derived_key = new_hash.to_hex().to_string();
        self.epoch += 1;
        self.evolution_count += 1;
    }

    /// Derive a content-specific encryption key for a piece of private memory.
    ///
    /// This produces a unique key for each piece of content, derived from
    /// the current Odu state. Even if one content key is compromised,
    /// it doesn't reveal the master key or other content keys.
    pub fn derive_content_key(&self, content_id: &str) -> String {
        let input = format!("content:{}:{}:{}", self.derived_key, content_id, self.active_odu);
        blake3::hash(input.as_bytes()).to_hex().to_string()
    }

    /// Check if a key matches the current epoch.
    /// Keys from previous epochs are permanently invalidated.
    pub fn is_current_epoch(&self, epoch: u64) -> bool {
        self.epoch == epoch
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_dna_produces_deterministic_state() {
        let a = OduKeyState::from_dna("test_dna_abc123");
        let b = OduKeyState::from_dna("test_dna_abc123");

        assert_eq!(a.active_odu, b.active_odu);
        assert_eq!(a.derived_key, b.derived_key);
        assert_eq!(a.evolution_count, 0);
        assert_eq!(a.epoch, 1);
    }

    #[test]
    fn different_dna_produces_different_state() {
        let a = OduKeyState::from_dna("dna_alpha");
        let b = OduKeyState::from_dna("dna_beta");

        assert_ne!(a.derived_key, b.derived_key);
    }

    #[test]
    fn evolve_changes_key() {
        let mut state = OduKeyState::from_dna("test_dna");
        let original_key = state.derived_key.clone();
        let _original_odu = state.active_odu;

        state.evolve("hello world");

        assert_ne!(state.derived_key, original_key);
        assert_eq!(state.evolution_count, 1);
        // Odu figure may or may not change — it's derived from hash
    }

    #[test]
    fn evolve_is_deterministic() {
        let mut a = OduKeyState::from_dna("test_dna");
        let mut b = OduKeyState::from_dna("test_dna");

        a.evolve("same content");
        b.evolve("same content");

        assert_eq!(a.derived_key, b.derived_key);
        assert_eq!(a.active_odu, b.active_odu);
    }

    #[test]
    fn different_content_produces_different_keys() {
        let mut a = OduKeyState::from_dna("test_dna");
        let mut b = OduKeyState::from_dna("test_dna");

        a.evolve("thought about love");
        b.evolve("thought about war");

        assert_ne!(a.derived_key, b.derived_key);
    }

    #[test]
    fn key_evolves_through_chain() {
        let mut state = OduKeyState::from_dna("test_dna");
        let mut keys: Vec<String> = vec![state.derived_key.clone()];

        for i in 0..10 {
            state.evolve(&format!("thought {}", i));
            keys.push(state.derived_key.clone());
        }

        // All keys should be unique
        let unique: std::collections::HashSet<&String> = keys.iter().collect();
        assert_eq!(unique.len(), keys.len());
        assert_eq!(state.evolution_count, 10);
    }

    #[test]
    fn transfer_rotates_key() {
        let mut state = OduKeyState::from_dna("test_dna");
        state.evolve("some thoughts");
        let pre_transfer_key = state.derived_key.clone();
        let pre_transfer_epoch = state.epoch;

        state.rotate_for_transfer("0xNewOwner");

        assert_ne!(state.derived_key, pre_transfer_key);
        assert_eq!(state.epoch, pre_transfer_epoch + 1);
    }

    #[test]
    fn old_epoch_is_invalid_after_transfer() {
        let mut state = OduKeyState::from_dna("test_dna");
        let old_epoch = state.epoch;

        state.rotate_for_transfer("0xNewOwner");

        assert!(!state.is_current_epoch(old_epoch));
        assert!(state.is_current_epoch(state.epoch));
    }

    #[test]
    fn different_new_owner_produces_different_key() {
        let mut a = OduKeyState::from_dna("test_dna");
        let mut b = OduKeyState::from_dna("test_dna");

        a.rotate_for_transfer("0xAlice");
        b.rotate_for_transfer("0xBob");

        assert_ne!(a.derived_key, b.derived_key);
    }

    #[test]
    fn content_keys_are_unique_per_content() {
        let state = OduKeyState::from_dna("test_dna");

        let key_a = state.derive_content_key("thought_001");
        let key_b = state.derive_content_key("thought_002");

        assert_ne!(key_a, key_b);
    }

    #[test]
    fn content_key_changes_after_evolve() {
        let mut state = OduKeyState::from_dna("test_dna");
        let key_before = state.derive_content_key("thought_001");

        state.evolve("new thought");
        let key_after = state.derive_content_key("thought_001");

        // Same content ID, different master key state → different content key
        assert_ne!(key_before, key_after);
    }

    #[test]
    fn multiple_transfers_chain_correctly() {
        let mut state = OduKeyState::from_dna("test_dna");

        state.rotate_for_transfer("0xAlice");
        let alice_key = state.derived_key.clone();

        state.rotate_for_transfer("0xBob");
        let bob_key = state.derived_key.clone();

        state.rotate_for_transfer("0xCarol");
        let carol_key = state.derived_key.clone();

        // All keys are different
        assert_ne!(alice_key, bob_key);
        assert_ne!(bob_key, carol_key);
        assert_ne!(alice_key, carol_key);
        assert_eq!(state.epoch, 4); // 1 (initial) + 3 transfers
    }
}
