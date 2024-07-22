#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use regex::Regex;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct User {
    id: u64,
    username: String,
    email: String,
    created_at: u64,
    balance: u64, // Simplified balance for the demo
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Transaction {
    id: u64,
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
    created_at: u64,
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for Transaction {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Transaction {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USER_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static TRANSACTION_STORAGE: RefCell<StableBTreeMap<u64, Transaction, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2)))
    ));
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct UserPayload {
    username: String,
    email: String,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
struct TransactionPayload {
    from_user_id: u64,
    to_user_id: u64,
    amount: u64,
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Message {
    Success(String),
    Error(String),
    NotFound(String),
    InvalidPayload(String),
    Unauthorized(String),
}

#[ic_cdk::update]
fn create_user(payload: UserPayload) -> Result<User, Message> {
    if payload.username.is_empty() || payload.email.is_empty() {
        return Err(Message::InvalidPayload(
            "Ensure 'username' and 'email' are provided.".to_string(),
        ));
    }

    let email_regex = Regex::new(r"^[^\s@]+@[^\s@]+\.[^\s@]+$").unwrap();
    if !email_regex.is_match(&payload.email) {
        return Err(Message::InvalidPayload(
            "Invalid email address format".to_string(),
        ));
    }

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let user = User {
        id,
        username: payload.username,
        email: payload.email,
        created_at: current_time(),
        balance: 0, // Initialize balance to 0
    };
    USER_STORAGE.with(|storage| storage.borrow_mut().insert(id, user.clone()));
    Ok(user)
}

#[ic_cdk::update]
fn send_transaction(payload: TransactionPayload) -> Result<Transaction, Message> {
    if payload.amount == 0 {
        return Err(Message::InvalidPayload("Amount must be greater than 0.".to_string()));
    }

    let from_user = USER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, user)| user.id == payload.from_user_id)
            .map(|(_, user)| user.clone())
    });

    if from_user.is_none() {
        return Err(Message::NotFound("Sender not found".to_string()));
    }

    let to_user = USER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, user)| user.id == payload.to_user_id)
            .map(|(_, user)| user.clone())
    });

    if to_user.is_none() {
        return Err(Message::NotFound("Recipient not found".to_string()));
    }

    let mut from_user = from_user.unwrap();
    let mut to_user = to_user.unwrap();

    if from_user.balance < payload.amount {
        return Err(Message::Error("Insufficient balance.".to_string()));
    }

    from_user.balance -= payload.amount;
    to_user.balance += payload.amount;

    USER_STORAGE.with(|storage| {
        storage.borrow_mut().insert(from_user.id, from_user.clone());
        storage.borrow_mut().insert(to_user.id, to_user.clone());
    });

    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("Cannot increment ID counter");

    let transaction = Transaction {
        id,
        from_user_id: payload.from_user_id,
        to_user_id: payload.to_user_id,
        amount: payload.amount,
        created_at: current_time(),
    };

    TRANSACTION_STORAGE.with(|storage| storage.borrow_mut().insert(id, transaction.clone()));
    Ok(transaction)
}

#[ic_cdk::query]
fn get_transaction_history(user_id: u64) -> Result<Vec<Transaction>, Message> {
    TRANSACTION_STORAGE.with(|storage| {
        let transactions: Vec<Transaction> = storage
            .borrow()
            .iter()
            .filter(|(_, transaction)| {
                transaction.from_user_id == user_id || transaction.to_user_id == user_id
            })
            .map(|(_, transaction)| transaction.clone())
            .collect();

        if transactions.is_empty() {
            Err(Message::NotFound("No transactions found".to_string()))
        } else {
            Ok(transactions)
        }
    })
}

#[ic_cdk::query]
fn get_user_balance(user_id: u64) -> Result<u64, Message> {
    USER_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .find(|(_, user)| user.id == user_id)
            .map(|(_, user)| user.balance)
            .ok_or(Message::NotFound("User not found".to_string()))
    })
}

fn current_time() -> u64 {
    time()
}

#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    Unauthorized { msg: String },
}

ic_cdk::export_candid!();
