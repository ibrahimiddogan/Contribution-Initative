
use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use serde::Serialize;
use std::{borrow::Cow, cell::RefCell};




#[derive(Clone,CandidType, Deserialize)]
pub struct EstablishmentUser{
    pub name:String,
    pub email:String,
    pub password:String,
    pub year:String,
    pub description:String,
    pub totalDonate:u32,
}
#[derive(Clone,CandidType, Deserialize)]
pub struct User{
    pub name:String,
    pub lastname:String,
    pub email:String,
    pub password:String,
    pub birthday:String,
    pub totalDonate:u32,
}
//#[derive(Clone, Serialize, Deserialize)]

#[derive(Clone, CandidType,Deserialize)]
pub struct Donation {
   pub donor_user: String,
   pub donor_establishment: String,
   pub amount: u32,
}


