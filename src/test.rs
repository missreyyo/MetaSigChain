#![cfg(test)]
extern crate std;

use crate::{contract::Token, TokenClient};
use crate::storage_types::DataKey;
use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation, MockAuth, MockAuthInvoke, Ledger, LedgerInfo},
    Address, Env, IntoVal, Symbol, String, vec, Vec,
};

fn create_token<'a>(e: &Env, admin: &Address) -> TokenClient<'a> {
    let token = TokenClient::new(e, &e.register(Token {}, ()));
    token.initialize(admin, &7, &"name".into_val(e), &"symbol".into_val(e));
    token
}

#[test]
fn test() {
    let e = Env::default();
    e.mock_all_auths();

    let admin1 = Address::generate(&e);
    let admin2 = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let user3 = Address::generate(&e);
    let token = create_token(&e, &admin1);

    token.mint(&user1, &1000);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("mint"),
                    (&user1, 1000_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user2, &user3, &500, &200);
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 500_i128, 200_u32).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.allowance(&user2, &user3), 500);

    token.transfer(&user1, &user2, &600);
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("transfer"),
                    (&user1, &user2, 600_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 400);
    assert_eq!(token.balance(&user2), 600);

    token.transfer_from(&user3, &user2, &user1, &400);
    assert_eq!(
        e.auths(),
        std::vec![(
            user3.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    Symbol::new(&e, "transfer_from"),
                    (&user3, &user2, &user1, 400_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.balance(&user1), 800);
    assert_eq!(token.balance(&user2), 200);

    token.transfer(&user1, &user3, &300);
    assert_eq!(token.balance(&user1), 500);
    assert_eq!(token.balance(&user3), 300);

    token.set_admin(&admin2);
    assert_eq!(
        e.auths(),
        std::vec![(
            admin1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("set_admin"),
                    (&admin2,).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    token.approve(&user2, &user3, &500, &200);
    assert_eq!(token.allowance(&user2, &user3), 500);
    token.approve(&user2, &user3, &0, &200);
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("approve"),
                    (&user2, &user3, 0_i128, 200_u32).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );
    assert_eq!(token.allowance(&user2, &user3), 0);
}

#[test]
fn test_burn() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user1, &user2, &500, &200);
    assert_eq!(token.allowance(&user1, &user2), 500);

    token.burn_from(&user2, &user1, &500);
    assert_eq!(
        e.auths(),
        std::vec![(
            user2.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn_from"),
                    (&user2, &user1, 500_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.allowance(&user1, &user2), 0);
    assert_eq!(token.balance(&user1), 500);
    assert_eq!(token.balance(&user2), 0);

    token.burn(&user1, &500);
    assert_eq!(
        e.auths(),
        std::vec![(
            user1.clone(),
            AuthorizedInvocation {
                function: AuthorizedFunction::Contract((
                    token.address.clone(),
                    symbol_short!("burn"),
                    (&user1, 500_i128).into_val(&e),
                )),
                sub_invocations: std::vec![]
            }
        )]
    );

    assert_eq!(token.balance(&user1), 0);
    assert_eq!(token.balance(&user2), 0);
}

#[test]
#[should_panic(expected = "insufficient balance")]
fn transfer_insufficient_balance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.transfer(&user1, &user2, &1001);
}

#[test]
#[should_panic(expected = "insufficient allowance")]
fn transfer_from_insufficient_allowance() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let user3 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    assert_eq!(token.balance(&user1), 1000);

    token.approve(&user1, &user3, &100, &200);
    assert_eq!(token.allowance(&user1, &user3), 100);

    token.transfer_from(&user3, &user1, &user2, &101);
}

#[test]
#[should_panic(expected = "already initialized")]
fn initialize_already_initialized() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.initialize(&admin, &10, &"name".into_val(&e), &"symbol".into_val(&e));
}

#[test]
#[should_panic(expected = "Decimal must fit in a u8")]
fn decimal_is_over_max() {
    let e = Env::default();
    let admin = Address::generate(&e);
    let token = TokenClient::new(&e, &e.register(Token {}, ()));
    token.initialize(
        &admin,
        &(u32::from(u8::MAX) + 1),
        &"name".into_val(&e),
        &"symbol".into_val(&e),
    );
}
#[test]
#[should_panic(expected = "account is frozen")]
fn transfer_from_frozen_account_should_fail() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    token.freeze_account(&user1); // dondur

    token.transfer(&user1, &user2, &100); // başarısız olmalı
}
#[test]
fn unfreeze_account_then_transfer() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user1 = Address::generate(&e);
    let user2 = Address::generate(&e);
    let token = create_token(&e, &admin);

    token.mint(&user1, &1000);
    token.freeze_account(&user1);
    token.unfreeze_account(&user1);
    token.transfer(&user1, &user2, &100);

    assert_eq!(token.balance(&user2), 100);
}
#[test]
#[should_panic(expected = "account is frozen")]
fn burn_from_frozen_account_should_fail() {
    let e = Env::default();
    e.mock_all_auths();

    let admin = Address::generate(&e);
    let user = Address::generate(&e);
    let token = create_token(&e, &admin);

    // İlk olarak user1'e token mint ediyoruz
    token.mint(&user, &1000);
    assert_eq!(token.balance(&user), 1000);

    // user1'in hesabını donduruyoruz
    token.freeze_account(&user); // dondurma işlemi

    // Dondurulmuş hesaptan token yakma işlemi deniyoruz
    token.burn(&user, &500); // bu işlem panik yapmalı çünkü hesap dondurulmuş
}

#[test]
fn test_multisig_setup() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let owner1 = Address::generate(&env);
    let owner2 = Address::generate(&env);
    let owner3 = Address::generate(&env);
    let contract_id = env.register(Token {}, ());
    let token = TokenClient::new(&env, &contract_id);

    // Initialize token
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "initialize",
            args: (admin.clone(), 10u32, String::from_str(&env, "Test Token"), String::from_str(&env, "TEST")).into_val(&env),
            sub_invokes: &[],
        },
    }]);
    token.initialize(&admin, &10, &String::from_str(&env, "Test Token"), &String::from_str(&env, "TEST"));

    // Setup multisig
    let owners = Vec::from_array(&env, [owner1.clone(), owner2.clone(), owner3.clone()]);
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "setup_multisig",
            args: (owners.clone(), 2u32).into_val(&env),
            sub_invokes: &[],
        },
    }]);
    token.setup_multisig(&owners, &2);

    // Verify setup
    let stored_owners: Vec<Address> = env.as_contract(&contract_id, || {
        env.storage().instance().get(&DataKey::MultiSigOwners).unwrap()
    });
    let stored_threshold: u32 = env.as_contract(&contract_id, || {
        env.storage().instance().get(&DataKey::MultiSigThreshold).unwrap()
    });
    assert_eq!(stored_owners.len(), 3);
    assert_eq!(stored_threshold, 2);
}



#[test]
#[should_panic(expected = "sender is not a multisig owner")]
fn test_multisig_transaction_unauthorized() {
    let env = Env::default();
    let admin = Address::generate(&env);
    let owner1 = Address::generate(&env);
    let owner2 = Address::generate(&env);
    let owner3 = Address::generate(&env);
    let unauthorized = Address::generate(&env);
    let target = Address::generate(&env);
    let contract_id = env.register(Token {}, ());
    let token = TokenClient::new(&env, &contract_id);

    // Initialize token
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "initialize",
            args: (admin.clone(), 10u32, String::from_str(&env, "Test Token"), String::from_str(&env, "TEST")).into_val(&env),
            sub_invokes: &[],
        },
    }]);
    token.initialize(&admin, &10, &String::from_str(&env, "Test Token"), &String::from_str(&env, "TEST"));

    // Setup multisig
    let owners = Vec::from_array(&env, [owner1.clone(), owner2.clone(), owner3.clone()]);
    env.mock_auths(&[MockAuth {
        address: &admin,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "setup_multisig",
            args: (owners.clone(), 2u32).into_val(&env),
            sub_invokes: &[],
        },
    }]);
    token.setup_multisig(&owners, &2);

    // Try to propose transaction with unauthorized user
    let operation = Symbol::new(&env, "transfer");
    env.mock_auths(&[MockAuth {
        address: &unauthorized,
        invoke: &MockAuthInvoke {
            contract: &contract_id,
            fn_name: "propose_multisig_transaction",
            args: (operation.clone(), target.clone(), 100i128, 1000u64).into_val(&env),
            sub_invokes: &[],
        },
    }]);
    token.propose_multisig_transaction(&operation, &target, &100, &1000);
}

