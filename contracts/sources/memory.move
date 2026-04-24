/// Memory module for Ọ̀ṣỌ́ — links pet memory to Walrus storage.
module oso::memory {
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use sui::event;

    /// On-chain record linking a pet to its Walrus memory blob.
    struct MemoryRecord has key, store {
        id: UID,
        pet_address: address,
        walrus_cid: vector<u8>,       // Walrus content identifier
        content_hash: vector<u8>,     // Blake3 hash of memory content
        timestamp: u64,
        sequence: u64,                // Monotonic memory sequence number
    }

    /// Emitted when memory is stored.
    struct MemoryStored has copy, drop {
        pet_address: address,
        walrus_cid: vector<u8>,
        sequence: u64,
    }

    /// Store a new memory record for a pet.
    public entry fun store(
        pet_address: address,
        walrus_cid: vector<u8>,
        content_hash: vector<u8>,
        timestamp: u64,
        sequence: u64,
        ctx: &mut TxContext,
    ) {
        let record = MemoryRecord {
            id: object::new(ctx),
            pet_address,
            walrus_cid,
            content_hash,
            timestamp,
            sequence,
        };

        event::emit(MemoryStored {
            pet_address,
            walrus_cid: record.walrus_cid,
            sequence,
        });

        transfer::transfer(record, tx_context::sender(ctx));
    }

    // === View functions ===

    public fun walrus_cid(record: &MemoryRecord): &vector<u8> { &record.walrus_cid }
    public fun content_hash(record: &MemoryRecord): &vector<u8> { &record.content_hash }
    public fun sequence(record: &MemoryRecord): u64 { record.sequence }
}
