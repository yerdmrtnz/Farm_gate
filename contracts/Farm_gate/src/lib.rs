#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    Address, Env, Symbol,
};

#[contract]
pub struct FarmGatePay;

#[contracttype]
#[derive(Clone)]
pub struct Escrow {
    pub buyer: Address,
    pub farmer: Address,
    pub amount: i128,
    pub delivered: bool,
    pub released: bool,
}

#[contracttype]
pub enum DataKey {
    Escrow(u64),
    Counter,
}

#[contractimpl]
impl FarmGatePay {

    // Create a new escrow payment
    pub fn create_escrow(
        env: Env,
        buyer: Address,
        farmer: Address,
        amount: i128,
    ) -> u64 {

        // Require buyer authorization
        buyer.require_auth();

        // Get current counter
        let mut counter: u64 = env
            .storage()
            .instance()
            .get(&DataKey::Counter)
            .unwrap_or(0);

        counter += 1;

        // Create escrow object
        let escrow = Escrow {
            buyer,
            farmer,
            amount,
            delivered: false,
            released: false,
        };

        // Store escrow
        env.storage()
            .instance()
            .set(&DataKey::Escrow(counter), &escrow);

        // Update counter
        env.storage()
            .instance()
            .set(&DataKey::Counter, &counter);

        counter
    }

    // Confirm produce delivery
    pub fn confirm_delivery(
        env: Env,
        escrow_id: u64,
        buyer: Address,
    ) {

        buyer.require_auth();

        let mut escrow: Escrow = env
            .storage()
            .instance()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap();

        // Ensure correct buyer confirms
        if escrow.buyer != buyer {
            panic!("Unauthorized buyer");
        }

        escrow.delivered = true;

        env.storage()
            .instance()
            .set(&DataKey::Escrow(escrow_id), &escrow);
    }

    // Release payment to farmer
    pub fn release_payment(
        env: Env,
        escrow_id: u64,
    ) {

        let mut escrow: Escrow = env
            .storage()
            .instance()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap();

        // Ensure delivery is confirmed
        if !escrow.delivered {
            panic!("Delivery not confirmed");
        }

        // Prevent double release
        if escrow.released {
            panic!("Payment already released");
        }

        escrow.released = true;

        env.storage()
            .instance()
            .set(&DataKey::Escrow(escrow_id), &escrow);
    }

    // Get escrow details
    pub fn get_escrow(
        env: Env,
        escrow_id: u64,
    ) -> Escrow {

        env.storage()
            .instance()
            .get(&DataKey::Escrow(escrow_id))
            .unwrap()
    }
}