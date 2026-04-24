/// Agent registry for Ọ̀ṣỌ́ — tracks all pets globally.
module oso::registry {
    use sui::object::{Self, UID};
    use sui::transfer;
    use sui::tx_context::{Self, TxContext};
    use sui::table::{Self, Table};
    use std::string::String;

    /// Shared registry object — one per deployment.
    struct Registry has key {
        id: UID,
        total_pets: u64,
        pet_names: Table<String, address>,  // name → pet object address
    }

    /// Create the global registry (called once at deploy).
    fun init(ctx: &mut TxContext) {
        let registry = Registry {
            id: object::new(ctx),
            total_pets: 0,
            pet_names: table::new(ctx),
        };
        transfer::share_object(registry);
    }

    /// Register a newly born pet.
    public entry fun register_pet(
        registry: &mut Registry,
        name: String,
        pet_address: address,
    ) {
        table::add(&mut registry.pet_names, name, pet_address);
        registry.total_pets = registry.total_pets + 1;
    }

    /// Look up a pet by name.
    public fun lookup(registry: &Registry, name: &String): &address {
        table::borrow(&registry.pet_names, *name)
    }

    /// Total number of pets ever born.
    public fun total_pets(registry: &Registry): u64 {
        registry.total_pets
    }
}
