#![cfg(test)]
use super::*;
use soroban_sdk::{symbol_short, vec, Env};

#[test]
fn test_hello() {
    let env = Env::default();
    let contract_id = env.register(HelloWorldContract, ());
    let client = HelloWorldContractClient::new(&env, &contract_id);
    let result = client.hello(&symbol_short!("Mundo"));
    assert_eq!(result, vec![&env, symbol_short!("Hola"), symbol_short!("Mundo")]);
}
