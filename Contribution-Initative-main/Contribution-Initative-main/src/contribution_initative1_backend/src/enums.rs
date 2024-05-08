use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};


#[derive(CandidType, Deserialize)]
pub enum userError {
    unfilled(String),
    incorrectEmail(String),
    incorrectPassword(String),
    inUse(String),
}
#[derive(CandidType,Deserialize)]
pub enum userResult {
    Success(String),
    Error(userError),
}