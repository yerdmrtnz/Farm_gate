#![cfg(test)]

use soroban_sdk::{
    testutils::Address as _,
    Address,
    Env,
};

use crate::{FarmGatePay, FarmGatePayClient};

mod tests {

    use super::*;

    // Test 1: Happy path
    #[test]
    fn test_happy_path() {

        let env = Env::default();

        let contract_id = env.register(FarmGatePay, ());
        let client = FarmGatePayClient::new(&env, &contract_id);

        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);

        let escrow_id = client.create_escrow(
            &buyer,
            &farmer,
            &5000,
        );

        client.confirm_delivery(
            &escrow_id,
            &buyer,
        );

        client.release_payment(&escrow_id);

        let escrow = client.get_escrow(&escrow_id);

        assert_eq!(escrow.released, true);
    }

    // Test 2: Edge case
    #[test]
    #[should_panic(expected = "Delivery not confirmed")]
    fn test_release_without_delivery() {

        let env = Env::default();

        let contract_id = env.register(FarmGatePay, ());
        let client = FarmGatePayClient::new(&env, &contract_id);

        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);

        let escrow_id = client.create_escrow(
            &buyer,
            &farmer,
            &3000,
        );

        client.release_payment(&escrow_id);
    }

    // Test 3: State verification
    #[test]
    fn test_state_verification() {

        let env = Env::default();

        let contract_id = env.register(FarmGatePay, ());
        let client = FarmGatePayClient::new(&env, &contract_id);

        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);

        let escrow_id = client.create_escrow(
            &buyer,
            &farmer,
            &7000,
        );

        client.confirm_delivery(
            &escrow_id,
            &buyer,
        );

        let escrow = client.get_escrow(&escrow_id);

        assert_eq!(escrow.delivered, true);
    }

    // Test 4
    #[test]
    fn test_multiple_escrows() {

        let env = Env::default();

        let contract_id = env.register(FarmGatePay, ());
        let client = FarmGatePayClient::new(&env, &contract_id);

        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);

        let id1 = client.create_escrow(
            &buyer,
            &farmer,
            &1000,
        );

        let id2 = client.create_escrow(
            &buyer,
            &farmer,
            &2000,
        );

        assert_ne!(id1, id2);
    }

    // Test 5
    #[test]
    #[should_panic(expected = "Payment already released")]
    fn test_double_release() {

        let env = Env::default();

        let contract_id = env.register(FarmGatePay, ());
        let client = FarmGatePayClient::new(&env, &contract_id);

        let buyer = Address::generate(&env);
        let farmer = Address::generate(&env);

        let escrow_id = client.create_escrow(
            &buyer,
            &farmer,
            &5000,
        );

        client.confirm_delivery(
            &escrow_id,
            &buyer,
        );

        client.release_payment(&escrow_id);

        // Second release should fail
        client.release_payment(&escrow_id);
    }
}