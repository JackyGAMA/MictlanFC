#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Symbol, symbol_short};

#[contract]
pub struct PayrollContract;

#[contractimpl]
impl PayrollContract {
    pub fn registrar_empleado(env: Env, admin: Address, empleado: Address, salario: i128) -> Symbol {
        admin.require_auth();
        symbol_short!("REGOK")
    }

    pub fn pagar_nomina(env: Env, admin: Address, empleado: Address) -> Symbol {
        admin.require_auth();
        symbol_short!("PAGO_OK")
    }

    pub fn consultar_empleado(env: Env, empleado: Address) -> Symbol {
        symbol_short!("ACTIVO")
    }
}

#[cfg(test)]
mod test;
