#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_payroll() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register(PayrollContract, ());
    let client = PayrollContractClient::new(&env, &contract_id);
    let emp = Address::generate(&env);
    assert_eq!(client.consultar_empleado(&emp), symbol_short!("ACTIVO"));
}
