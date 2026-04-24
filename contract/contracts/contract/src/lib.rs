#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, Symbol, Address
};

#[contract]
pub struct EmergencyPool;

#[derive(Clone)]
#[contracttype]
pub struct Member {
    pub contributed: i128,
    pub is_active: bool,
}

fn get_member_key(env: &Env, user: &Address) -> (Symbol, Address) {
    (Symbol::new(env, "member"), user.clone())
}

#[contractimpl]
impl EmergencyPool {

    // Register a new member
    pub fn register(env: Env, user: Address) {
        let key = get_member_key(&env, &user);

        if env.storage().instance().has(&key) {
            panic!("Already registered");
        }

        let member = Member {
            contributed: 0,
            is_active: true,
        };

        env.storage().instance().set(&key, &member);
    }

    // Contribute to the pool
    pub fn contribute(env: Env, user: Address, amount: i128) {
        user.require_auth();

        let key = get_member_key(&env, &user);

        let mut member: Member = env.storage().instance().get(&key)
            .expect("Not registered");

        if !member.is_active {
            panic!("Inactive member");
        }

        member.contributed += amount;

        env.storage().instance().set(&key, &member);
    }

    // Request emergency withdrawal
    pub fn request_emergency_fund(env: Env, user: Address, amount: i128) {
        user.require_auth();

        let key = get_member_key(&env, &user);

        let member: Member = env.storage().instance().get(&key)
            .expect("Not registered");

        if !member.is_active {
            panic!("Not active");
        }

        if member.contributed <= 0 {
            panic!("No contribution history");
        }

        // Emit event
        let event = symbol_short!("payout");
        env.events().publish((event, user), amount);
    }
}