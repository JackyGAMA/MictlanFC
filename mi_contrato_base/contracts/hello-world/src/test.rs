#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_token_and_faucet() {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register(TokenContract, ());
    let client = TokenContractClient::new(&env, &contract_id);

    let admin = Address::generate(&env);
    let user1 = Address::generate(&env);
    let user2 = Address::generate(&env);

    client.initialize(&admin, &100);
    assert_eq!(client.get_faucet_amount(), 100);

    // Test faucet
    client.faucet(&user1);
    assert_eq!(client.balance(&user1), 100);

    // Test transfer
    client.transfer(&user1, &user2, &30);
    assert_eq!(client.balance(&user1), 70);
    assert_eq!(client.balance(&user2), 30);
}
