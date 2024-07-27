#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, vec, Env, Symbol, Vec};

#[contract]
pub struct CertificationContract;

#[contractimpl]
impl CertificationContract {
    pub fn echo(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, to]
    }

    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![&env, symbol_short!("Hello"), to]
    }
}

mod test;
