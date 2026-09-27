#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct HelloWorldContract;

#[contractimpl]
impl HelloWorldContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hola"), to]
    }

    pub fn mensaje_bienvenida(env: Env) -> Symbol {
        symbol_short!("HolaMundo")
    }
}

#[cfg(test)]
mod test;
