#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_custom() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(ContratoDePruebaDina, ());
    let client = ContratoDePruebaDinaClient::new(&env, &contract_id);
    let owner = Address::generate(&env);
    assert_eq!(client.initialize(&owner), symbol_short!("INIT_OK"));
}
