#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Symbol, String, symbol_short, log};

#[contracttype]
#[derive(Clone)]
pub struct WasteEntry {
    pub user_id: u64,
    pub category: String, // e.g., Plastic, Organic, E-waste
    pub quantity: u64,    // in kilograms
    pub timestamp: u64,
}

#[contracttype]
pub enum WasteBook {
    Entry(u64),
}

const ENTRY_COUNT: Symbol = symbol_short!("COUNT");

#[contract]
pub struct WasteRecycleContract;

#[contractimpl]
impl WasteRecycleContract {
    pub fn log_waste(env: Env, user_id: u64, category: String, quantity: u64) {
        let mut count: u64 = env.storage().instance().get(&ENTRY_COUNT).unwrap_or(0);
        count += 1;

        let entry = WasteEntry {
            user_id,
            category,
            quantity,
            timestamp: env.ledger().timestamp(),
        };

        env.storage().instance().set(&WasteBook::Entry(count), &entry);
        env.storage().instance().set(&ENTRY_COUNT, &count);

        log!(&env, "Waste logged: ID {}, Category {}, Quantity {}kg", user_id, entry.category, entry.quantity);
    }

    pub fn get_entry(env: Env, index: u64) -> WasteEntry {
        env.storage().instance().get(&WasteBook::Entry(index)).unwrap()
    }

    pub fn get_total_entries(env: Env) -> u64 {
        env.storage().instance().get(&ENTRY_COUNT).unwrap_or(0)
    }
}