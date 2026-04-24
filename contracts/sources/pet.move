/// Dynamic NFT pet object for Ọ̀ṣỌ́.
/// Each pet is a living, evolving on-chain entity.
module oso::pet {
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use sui::event;
    use std::string::{Self, String};

    /// The pet — a dynamic NFT that grows and evolves.
    struct Pet has key, store {
        id: UID,
        name: String,
        dna_fingerprint: vector<u8>,  // 86 bytes — unique identity
        ascii_form: String,           // Current visual representation
        tier: u8,                     // 1-5 evolution level
        xp: u64,                      // Experience points
        memory_root: vector<u8>,      // Walrus content ID hash
        owner: address,
    }

    /// Emitted when a pet is born.
    struct PetBorn has copy, drop {
        pet_id: address,
        name: String,
        dna: vector<u8>,
        owner: address,
    }

    /// Emitted when a pet evolves to a new tier.
    struct PetEvolved has copy, drop {
        pet_id: address,
        old_tier: u8,
        new_tier: u8,
        xp: u64,
    }

    /// birth "name" — Create a new pet and transfer to sender.
    public entry fun birth(
        name: vector<u8>,
        dna: vector<u8>,
        ascii_form: vector<u8>,
        ctx: &mut TxContext,
    ) {
        let sender = tx_context::sender(ctx);
        let pet = Pet {
            id: object::new(ctx),
            name: string::utf8(name),
            dna_fingerprint: dna,
            ascii_form: string::utf8(ascii_form),
            tier: 1,
            xp: 0,
            memory_root: vector::empty(),
            owner: sender,
        };

        event::emit(PetBorn {
            pet_id: object::uid_to_address(&pet.id),
            name: pet.name,
            dna: pet.dna_fingerprint,
            owner: sender,
        });

        transfer::transfer(pet, sender);
    }

    /// Apply XP gain and check for tier evolution.
    public entry fun evolve(pet: &mut Pet, xp_gain: u64) {
        let old_tier = pet.tier;
        pet.xp = pet.xp + xp_gain;

        if (pet.xp >= 10000 && pet.tier < 5) {
            pet.tier = 5;
        } else if (pet.xp >= 2000 && pet.tier < 4) {
            pet.tier = 4;
        } else if (pet.xp >= 500 && pet.tier < 3) {
            pet.tier = 3;
        } else if (pet.xp >= 100 && pet.tier < 2) {
            pet.tier = 2;
        };

        if (pet.tier != old_tier) {
            event::emit(PetEvolved {
                pet_id: object::uid_to_address(&pet.id),
                old_tier,
                new_tier: pet.tier,
                xp: pet.xp,
            });
        };
    }

    /// Update the ASCII visual form (called after evolution).
    public entry fun update_ascii(pet: &mut Pet, new_ascii: vector<u8>) {
        pet.ascii_form = string::utf8(new_ascii);
    }

    /// Store a new Walrus memory root hash.
    public entry fun store_memory_root(pet: &mut Pet, memory_root: vector<u8>) {
        pet.memory_root = memory_root;
    }

    // === View functions ===

    public fun name(pet: &Pet): &String { &pet.name }
    public fun tier(pet: &Pet): u8 { pet.tier }
    public fun xp(pet: &Pet): u64 { pet.xp }
    public fun dna(pet: &Pet): &vector<u8> { &pet.dna_fingerprint }
    public fun ascii_form(pet: &Pet): &String { &pet.ascii_form }
    public fun memory_root(pet: &Pet): &vector<u8> { &pet.memory_root }
}
