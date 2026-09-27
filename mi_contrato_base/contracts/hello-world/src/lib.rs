#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, symbol_short};

#[contract]
pub struct ContratoDePruebaDina;

#[contractimpl]
impl ContratoDePruebaDina {
    pub fn initialize(env: Env, owner: Address) -> Symbol {
        owner.require_auth();
        symbol_short!("INIT_OK")
    }

    pub fn ejecutar_accion(env: Env, caller: Address) -> Symbol {
        caller.require_auth();
        symbol_short!("SUCCESS")
    }

    pub fn consultar_estado(env: Env) -> Symbol {
        symbol_short!("ACTIVO")
    }
}

#[cfg(test)]
mod test;
