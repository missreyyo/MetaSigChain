#![cfg_attr(feature = "no-std", no_std)]
extern crate alloc;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

use crate::admin::{has_administrator, read_administrator, write_administrator};
use crate::allowance::{read_allowance, spend_allowance, write_allowance};
use crate::balance::{read_balance, receive_balance, spend_balance};
use crate::metadata::{read_decimal, read_name, read_symbol, write_metadata};
use crate::storage_types::{INSTANCE_BUMP_AMOUNT, INSTANCE_LIFETIME_THRESHOLD};
use crate::storage_types::DataKey;
use soroban_sdk::token::{self, Interface as _};
use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol, Vec, Val, TryFromVal, IntoVal};
use soroban_token_sdk::metadata::TokenMetadata;
use soroban_token_sdk::TokenUtils;

// İşlem yapısı
#[derive(Clone)]
pub struct MultiSigTransaction {
    operation: Symbol,     // İşlem türü
    target: Address,      // Hedef adres
    amount: i128,         // İşlem miktarı
    expiration: u64,      // Son geçerlilik zamanı
    executed: bool,       // İşlem gerçekleştirildi mi
}

impl TryFromVal<Env, Val> for MultiSigTransaction {
    type Error = soroban_sdk::Error;

    fn try_from_val(env: &Env, val: &Val) -> Result<Self, Self::Error> {
        let vec = Vec::<Val>::try_from_val(env, val)?;
        Ok(MultiSigTransaction {
            operation: Symbol::try_from_val(env, &vec.get(0).unwrap())?,
            target: Address::try_from_val(env, &vec.get(1).unwrap())?,
            amount: i128::try_from_val(env, &vec.get(2).unwrap())?,
            expiration: u64::try_from_val(env, &vec.get(3).unwrap())?,
            executed: bool::try_from_val(env, &vec.get(4).unwrap())?,
        })
    }
}

impl IntoVal<Env, Val> for MultiSigTransaction {
    fn into_val(&self, env: &Env) -> Val {
        let mut vec: Vec<Val> = Vec::new(env);
        vec.push_back(self.operation.into_val(env));
        vec.push_back(self.target.into_val(env));
        vec.push_back(self.amount.into_val(env));
        vec.push_back(self.expiration.into_val(env));
        vec.push_back(self.executed.into_val(env));
        vec.into_val(env)
    }
}

fn check_nonnegative_amount(amount: i128) {
    if amount < 0 {
        panic!("Negatif miktar işlemlere izin verilmez: {}", amount)
    }
}

// Bir hesabın dondurulup dondurulmadığını kontrol eden yardımcı fonksiyon
fn is_account_frozen(e: &Env, account: &Address) -> bool {
    let key = DataKey::Frozen(account.clone());
    e.storage().instance().get::<_, bool>(&key).unwrap_or(false)
}

// Özel olayları yayınlamak için yardımcı fonksiyon
fn emit_custom_event(e: &Env, event_type: &str, admin: Address, account: Address) {
    e.events().publish((event_type, admin, account), ());
}

// Çoklu imza sahiplerini alan yardımcı fonksiyon
fn get_multisig_owners(e: &Env) -> Vec<Address> {
    e.storage().instance().get(&DataKey::MultiSigOwners).unwrap_or_else(|| {
        panic!("Çoklu imza yapılandırması bulunamadı");
    })
}



#[contract]
pub struct Token;

#[contractimpl]
impl Token {
    pub fn initialize(e: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        if has_administrator(&e) {
            panic!("already initialized")
        }
        write_administrator(&e, &admin);
        if decimal > u8::MAX.into() {
            panic!("Decimal must fit in a u8");
        }

        write_metadata(
            &e,
            TokenMetadata {
                decimal,
                name,
                symbol,
            },
        )
    }

    pub fn mint(e: Env, to: Address, amount: i128) {
        check_nonnegative_amount(amount);
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().mint(admin, to, amount);
    }

    pub fn set_admin(e: Env, new_admin: Address) {
        let admin = read_administrator(&e);
        admin.require_auth();

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_administrator(&e, &new_admin);
        TokenUtils::new(&e).events().set_admin(admin, new_admin);
    }

    // Bir hesabı dondur (sadece yönetici yapabilir)
    pub fn freeze_account(e: Env, account: Address) {
        // Sadece yönetici hesapları dondurabilir
        let admin = read_administrator(&e);
        admin.require_auth();

        // Kontrat örneğinin TTL süresini uzat
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Hesabı dondurulmuş olarak ayarla
        let key = DataKey::Frozen(account.clone());
        e.storage().instance().set(&key, &true);

       // Dondurma olayını yayınla
       emit_custom_event(&e, "freeze_account", admin, account);
    }

    // Bir hesabın dondurulmasını kaldır (sadece yönetici yapabilir)
    pub fn unfreeze_account(e: Env, account: Address) {
        // Sadece yönetici hesapların dondurulmasını kaldırabilir
        let admin = read_administrator(&e);
        admin.require_auth();

        // Kontrat örneğinin TTL süresini uzat
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Dondurulmuş durumu kaldır
        let key = DataKey::Frozen(account.clone());
        e.storage().instance().remove(&key);

        // Dondurma kaldırma olayını yayınla
        emit_custom_event(&e, "unfreeze_account", admin, account);
    }
     // Çoklu imza (multisig) yapılandırmasını ayarlama
     pub fn setup_multisig(e: Env, owners: Vec<Address>, threshold: u32) {
        let admin = read_administrator(&e);
        admin.require_auth();
        
        if owners.is_empty() {
            panic!("Sahip listesi boş olamaz");
        }
        
        if threshold == 0 || threshold > owners.len().try_into().unwrap() {
            panic!("Geçersiz eşik değeri");
        }
        
        // Çoklu imza ayarlarını kaydet
        e.storage().instance().set(&DataKey::MultiSigRequired, &true);
        e.storage().instance().set(&DataKey::MultiSigOwners, &owners);
        e.storage().instance().set(&DataKey::MultiSigThreshold, &threshold);
        
        // Olayı yayınla
        e.events().publish(
            ("setup_multisig", admin), 
            (owners.len(), threshold)
        );
    }

    // Çoklu imza işlemi önerisi
    pub fn propose_multisig_transaction(e: Env, function: Symbol, to: Address, amount: i128, expiration: u64) -> u64 {
        let sender = e.current_contract_address();
        let owners = get_multisig_owners(&e);
        if !owners.contains(&sender) {
            panic!("sender is not a multisig owner");
        }

        // İşlem ID'sini oluştur
        let transaction_id: u64 = e.storage().instance().get(&DataKey::TotalTransfers).unwrap_or(0);
        e.storage().instance().set(&DataKey::TotalTransfers, &(transaction_id + 1));
        
        // İşlemi kaydet
        let transaction = MultiSigTransaction {
            operation: function.clone(),
            target: to.clone(),
            amount,
            expiration,
            executed: false,
        };
        
        e.storage().instance().set(&DataKey::MultiSigTransaction(transaction_id), &transaction);
        
        // Gönderici tarafından otomatik onay
        let mut approvals: Vec<Address> = e.storage().instance().get(&DataKey::MultiSigApproval(transaction_id)).unwrap_or_else(|| Vec::new(&e));
        approvals.push_back(sender.clone());
        e.storage().instance().set(&DataKey::MultiSigApproval(transaction_id), &approvals);
        
        // Olayı yayınla
        e.events().publish(
            ("propose_multisig_transaction", sender), 
            (transaction_id, function, to, amount, expiration)
        );
        
        transaction_id
    }

    // Çoklu imza işlemine onay verme
    pub fn approve_multisig_transaction(e: Env, transaction_id: u64) {
        let sender = e.current_contract_address();
        let owners = get_multisig_owners(&e);
        if !owners.contains(&sender) {
            panic!("sender is not a multisig owner");
        }

        // İşlemi kontrol et
        let transaction: MultiSigTransaction = e.storage().instance().get(&DataKey::MultiSigTransaction(transaction_id)).unwrap_or_else(|| {
            panic!("transaction not found");
        });
        
        if transaction.executed {
            panic!("transaction already executed");
        }
        
        // Süre dolmuş mu kontrol et
        let ledger_info = e.ledger().sequence();
        if u64::from(ledger_info) >= transaction.expiration {
            panic!("transaction expired");
        }
        
        // Onayı kaydet
        let mut approvals: Vec<Address> = e.storage().instance().get(&DataKey::MultiSigApproval(transaction_id)).unwrap_or_else(|| Vec::new(&e));
        if !approvals.contains(&sender) {
            approvals.push_back(sender.clone());
            e.storage().instance().set(&DataKey::MultiSigApproval(transaction_id), &approvals);
        }
        
        // Olayı yayınla
        e.events().publish(
            ("approve_multisig_transaction", sender), 
            transaction_id
        );
        
        // Yeterli onay varsa işlemi gerçekleştir
        Self::execute_multisig_transaction_if_approved(&e, transaction_id);
    }

    // Onaylanan çoklu imza işlemine ilişkin işlemi gerçekleştir
    fn execute_multisig_transaction_if_approved(e: &Env, transaction_id: u64) {
        // İşlemi al
        let transaction: MultiSigTransaction = e.storage().instance().get(&DataKey::MultiSigTransaction(transaction_id)).unwrap();
        
        // İşlem zaten gerçekleştirilmiş mi kontrol et
        if transaction.executed {
            panic!("transaction already executed");
        }
        
        // İşlem süresi dolmuş mu kontrol et
        let current_sequence = u64::from(e.ledger().sequence());
        if current_sequence > transaction.expiration {
            panic!("transaction expired");
        }
        
        // Onayları kontrol et
        let approvals: Vec<Address> = e.storage().instance().get(&DataKey::MultiSigApproval(transaction_id)).unwrap_or_else(|| Vec::new(e));
        let threshold: u32 = e.storage().instance().get(&DataKey::MultiSigThreshold).unwrap();
        
        if approvals.len() < threshold.try_into().unwrap() {
            return; // Yeterli onay yok, işlemi gerçekleştirme
        }
        
        // İşlemi gerçekleştir
        let op = transaction.operation.clone();
        if op == Symbol::new(e, "transfer") {
            let admin = read_administrator(e);
            admin.require_auth();
            spend_balance(e, transaction.target.clone(), transaction.amount);
            receive_balance(e, transaction.target.clone(), transaction.amount);
            TokenUtils::new(e).events().transfer(admin, transaction.target.clone(), transaction.amount);
        } else if op == Symbol::new(e, "mint") {
            let admin = read_administrator(e);
            admin.require_auth();
            receive_balance(e, transaction.target.clone(), transaction.amount);
            TokenUtils::new(e).events().mint(admin, transaction.target.clone(), transaction.amount);
        } else if op == Symbol::new(e, "burn") {
            let admin = read_administrator(e);
            admin.require_auth();
            spend_balance(e, transaction.target.clone(), transaction.amount);
            TokenUtils::new(e).events().burn(admin, transaction.amount);
        } else {
            panic!("unknown operation");
        }
        
        // İşlemi gerçekleştirildi olarak işaretle
        let mut updated_transaction = transaction.clone();
        updated_transaction.executed = true;
        e.storage().instance().set(&DataKey::MultiSigTransaction(transaction_id), &updated_transaction);
        
        // Olayı yayınla
        e.events().publish(
            ("execute_multisig_transaction", transaction_id), 
            (transaction.operation, transaction.target, transaction.amount)
        );
    }
}

#[contractimpl]
impl token::Interface for Token {
    fn allowance(e: Env, from: Address, spender: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_allowance(&e, from, spender).amount
    }

    fn approve(e: Env, from: Address, spender: Address, amount: i128, expiration_ledger: u32) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        write_allowance(&e, from.clone(), spender.clone(), amount, expiration_ledger);
        TokenUtils::new(&e)
            .events()
            .approve(from, spender, amount, expiration_ledger);
    }

    fn balance(e: Env, id: Address) -> i128 {
        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);
        read_balance(&e, id)
    }

    fn transfer(e: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Göndericinin hesabı dondurulmuş mu kontrol et
        if is_account_frozen(&e, &from) {
            panic!("account is frozen");
        }

        // Transferi gerçekleştir
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount);
    }

    fn transfer_from(e: Env, spender: Address, from: Address, to: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Göndericinin hesabı dondurulmuş mu kontrol et
        if is_account_frozen(&e, &from) {
            panic!("account is frozen");
        }

         // Transferi gerçekleştir
        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        receive_balance(&e, to.clone(), amount);
        TokenUtils::new(&e).events().transfer(from, to, amount)
    }

    fn burn(e: Env, from: Address, amount: i128) {
        from.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

        // Göndericinin hesabı dondurulmuş mu kontrol et
        if is_account_frozen(&e, &from) {
            panic!("account is frozen");
        }

        // Yakma işlemini gerçekleştir
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount);
    }

    fn burn_from(e: Env, spender: Address, from: Address, amount: i128) {
        spender.require_auth();

        check_nonnegative_amount(amount);

        e.storage()
            .instance()
            .extend_ttl(INSTANCE_LIFETIME_THRESHOLD, INSTANCE_BUMP_AMOUNT);

         // Göndericinin hesabı dondurulmuş mu kontrol et
         if is_account_frozen(&e, &from) {
            panic!("account is frozen");
        }

        // Yakma işlemini gerçekleştir
        spend_allowance(&e, from.clone(), spender, amount);
        spend_balance(&e, from.clone(), amount);
        TokenUtils::new(&e).events().burn(from, amount)
    }

    fn decimals(e: Env) -> u32 {
        read_decimal(&e)
    }

    fn name(e: Env) -> String {
        read_name(&e)
    }

    fn symbol(e: Env) -> String {
        read_symbol(&e)
    }
}
