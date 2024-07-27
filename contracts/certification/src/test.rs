#![cfg(test)]

use super::*;
use soroban_sdk::{symbol_short, vec, Env};

#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, CertificationContract);
    let client = CertificationContractClient::new(&env, &contract_id);

    let words = client.echo(&symbol_short!("Rust"));
    assert_eq!(words, vec![&env, symbol_short!("Rust")]);
}
