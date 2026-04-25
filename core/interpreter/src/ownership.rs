/// Agent ownership and private memory protection.
///
/// Core principle: private memory belongs to the AGENT, not the owner.
/// When an agent is sold/transferred, the previous owner loses access
/// to all private thoughts. The agent carries its memory forward.
///
/// The encryption key system is a placeholder — the actual Living Odu
/// Memory / entropy engine will be implemented in a later phase.

/// Tracks who owns an agent and controls memory access.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct OwnershipRecord {
    /// The agent's unique ID.
    pub agent_id: String,
    /// Current owner's Sui address.
    pub current_owner: String,
    /// History of all past owners (Sui addresses).
    pub previous_owners: Vec<String>,
    /// Placeholder for the encryption key that protects private memory.
    /// In production, this will be derived from the Living Odu Memory
    /// entropy engine. For now, it's a simple key rotation marker.
    pub memory_key_version: u64,
}

/// Result of an ownership transfer.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TransferResult {
    pub agent_id: String,
    pub from_owner: String,
    pub to_owner: String,
    /// The new memory key version after transfer.
    pub new_key_version: u64,
}

impl OwnershipRecord {
    pub fn new(agent_id: String, owner: String) -> Self {
        Self {
            agent_id,
            current_owner: owner,
            previous_owners: Vec::new(),
            memory_key_version: 1,
        }
    }

    /// Transfer ownership to a new address.
    ///
    /// This rotates the memory key version AND the agent's Odu key,
    /// which means the previous owner can no longer decrypt private
    /// thoughts. The agent's memory is NOT deleted — it's re-encrypted
    /// under the new key.
    ///
    /// Call `agent.odu_key.rotate_for_transfer(new_owner)` alongside this
    /// to actually rotate the Living Odu Memory encryption key.
    pub fn transfer(&mut self, new_owner: String) -> TransferResult {
        let from = self.current_owner.clone();
        self.previous_owners.push(from.clone());
        self.current_owner = new_owner.clone();
        self.memory_key_version += 1;

        TransferResult {
            agent_id: self.agent_id.clone(),
            from_owner: from,
            to_owner: new_owner,
            new_key_version: self.memory_key_version,
        }
    }

    /// Check if an address is the current owner.
    pub fn is_owner(&self, address: &str) -> bool {
        self.current_owner == address
    }

    /// Check if an address was ever an owner (for audit purposes).
    pub fn was_owner(&self, address: &str) -> bool {
        self.previous_owners.iter().any(|o| o == address)
    }

    /// Check if a given key version is current (can decrypt private memory).
    /// Old key versions cannot access private thoughts.
    pub fn is_key_valid(&self, version: u64) -> bool {
        version == self.memory_key_version
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_owner() {
        let rec = OwnershipRecord::new("agent1".into(), "0xAlice".into());
        assert!(rec.is_owner("0xAlice"));
        assert!(!rec.is_owner("0xBob"));
        assert_eq!(rec.memory_key_version, 1);
        assert!(rec.previous_owners.is_empty());
    }

    #[test]
    fn transfer_revokes_old_owner() {
        let mut rec = OwnershipRecord::new("agent1".into(), "0xAlice".into());
        let result = rec.transfer("0xBob".into());

        assert!(!rec.is_owner("0xAlice"));
        assert!(rec.is_owner("0xBob"));
        assert!(rec.was_owner("0xAlice"));
        assert_eq!(result.from_owner, "0xAlice");
        assert_eq!(result.to_owner, "0xBob");
    }

    #[test]
    fn transfer_rotates_key() {
        let mut rec = OwnershipRecord::new("agent1".into(), "0xAlice".into());
        let old_version = rec.memory_key_version;
        rec.transfer("0xBob".into());

        assert_ne!(rec.memory_key_version, old_version);
        assert!(!rec.is_key_valid(old_version));
        assert!(rec.is_key_valid(rec.memory_key_version));
    }

    #[test]
    fn multiple_transfers_track_history() {
        let mut rec = OwnershipRecord::new("agent1".into(), "0xAlice".into());
        rec.transfer("0xBob".into());
        rec.transfer("0xCarol".into());
        rec.transfer("0xDave".into());

        assert!(rec.is_owner("0xDave"));
        assert!(rec.was_owner("0xAlice"));
        assert!(rec.was_owner("0xBob"));
        assert!(rec.was_owner("0xCarol"));
        assert_eq!(rec.previous_owners.len(), 3);
        assert_eq!(rec.memory_key_version, 4);
    }

    #[test]
    fn old_key_cannot_decrypt() {
        let mut rec = OwnershipRecord::new("agent1".into(), "0xAlice".into());
        let alice_key = rec.memory_key_version;

        rec.transfer("0xBob".into());
        let bob_key = rec.memory_key_version;

        // Alice's key version is no longer valid
        assert!(!rec.is_key_valid(alice_key));
        // Bob's key version is current
        assert!(rec.is_key_valid(bob_key));
    }
}
