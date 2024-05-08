use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use structs::{EstablishmentUser, Donation};
use std::borrow::BorrowMut;
use std::{borrow::Cow, cell::RefCell};
use crate::structs::User;
use crate::enums::userResult;
pub mod structs;
pub mod enums;
use crate::enums::userError;



impl Storable for Donation {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl Storable for User {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl Storable for EstablishmentUser {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

type Memory = VirtualMemory<DefaultMemoryImpl>;
const MAX_VALUE_SIZE: u32 = 10000000;
impl BoundedStorable for User {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE; 
    const IS_FIXED_SIZE: bool = false;
}
impl BoundedStorable for EstablishmentUser {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE; 
    const IS_FIXED_SIZE: bool = false;
}
impl BoundedStorable for Donation {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE; 
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static USER_MAP: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|p| p.borrow().get(MemoryId::new(1))), 
        )
    );
    static EsUSER_MAP: RefCell<StableBTreeMap<u64, EstablishmentUser, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|p| p.borrow().get(MemoryId::new(2))), 
        )
    );
    static DONATION_MAP: RefCell<StableBTreeMap<u64, Donation, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|p| p.borrow().get(MemoryId::new(3))), 
        )
    );
    
}

#[ic_cdk::update]
fn create_user(name:String,lastname:String,email:String,password: String,birthday:String,totalDonate:u32)->Result<userResult, userError> {
    if name.is_empty() || lastname.is_empty() || email.is_empty()||password.is_empty(){
        return Err(userError::unfilled("boş alanlar mevcut".to_string()));
    }
    let inuseEmail=USER_MAP.with(|p| {
        let user_map = p.borrow();
       
        user_map.iter().any(|(_, user)| user.email == email)

    });
    if inuseEmail{
        return Err(userError::inUse("Girilen e-posta adresi zaten mevcut".to_string()));

    }else{
 
 
    
   USER_MAP.with(|p|{
    let mut user_map = p.borrow_mut();
    
    let new_user = User{
        name:name,
        lastname:lastname,
        password:password,
        email:email,
        birthday:birthday, 
        totalDonate:totalDonate,
        
    };
    let new_user_id=user_map.len();
user_map.insert(new_user_id,new_user);
   });
   Ok(userResult::Success("üyeliğiniz oluşturulmuştur".to_string()))}
}







#[ic_cdk::update]
fn create_establishment_user(name:String,description:String,email:String,password: String,year:String,totalDonate:u32)->Result<userResult, userError> {
    if name.is_empty() || description.is_empty() || email.is_empty()||password.is_empty(){
        return Err(userError::unfilled("boş alanlar mevcut".to_string()));
    }
    let inuseEmail=EsUSER_MAP.with(|p| {
        let esuser_map = p.borrow();
       
        esuser_map.iter().any(|(_, esuser)| esuser.email == email)

    });
    if inuseEmail{
        return Err(userError::inUse("Girilen e-posta adresi zaten mevcut".to_string()));

    }else{
 
 
    
   EsUSER_MAP.with(|p|{
    let mut esuser_map = p.borrow_mut();
    
    let new_user = EstablishmentUser{
        name:name,
        description:description,
        password:password,
        email:email,
        year:year, 
        totalDonate:totalDonate,
    };
    let new_user_id=esuser_map.len();
esuser_map.insert(new_user_id,new_user);
   });
   Ok(userResult::Success("üyeliğiniz oluşturulmuştur".to_string()))}
}



#[ic_cdk_macros::query]
fn sort_esusers()->Vec<EstablishmentUser> {
    let mut esusers = Vec::new();
    EsUSER_MAP.with(|p|{
        let user_map=p.borrow();
       
    for (_, user) in user_map.iter() {
    esusers.push(user.clone());
}
});
esusers   
} 
#[ic_cdk_macros::query]
fn sort_users()->Vec<User> {
    let mut users = Vec::new();
    USER_MAP.with(|p|{
        let user_map=p.borrow();
       
    for (_, user) in user_map.iter() {
    users.push(user.clone());
}
});
users   
} 

#[ic_cdk::update]
fn donate(amount:u32,email:String,email2:String)->Result<userResult, userError> {
   
    if amount.to_string().is_empty(){
        return Err(userError::unfilled("Enter Amount".to_string()));
    }
   USER_MAP.with(|p| {
        let  user_map = p.borrow_mut();
        for (_, mut user) in user_map.iter() {
            if user.email == email {
                user.totalDonate += amount;
            }
        }
        
    });

   EsUSER_MAP.with(|p| {
        let user_map = p.borrow_mut();
        for (_, mut user) in user_map.iter() {
            if user.email == email2 {
                user.totalDonate += amount;
                
            }
        }
        
    });


    DONATION_MAP.with(|p|{
        let mut donation_map = p.borrow_mut();
        
        let new_user = Donation{
            donor_user:email,
            donor_establishment:email2,
            amount:amount,
        };
        let new_user_id=donation_map.len();
        donation_map.insert(new_user_id,new_user);
       });

    // user.totalDonate += donation.amount;
    // establishment.totalDonate += donation.amount;
    Ok(userResult::Success("Donation is completed".to_string()))
}
// #[ic_cdk::update]
// fn donate(email:String,email2:String,amount:u32)->Result<userResult,userError>{
//     if amount.to_string().is_empty(){
//                  return Err(userError::unfilled("Miktar giriniz".to_string()));}
//                  let user_found = USER_MAP.with(|p| {
//                             let mut user_map = p.borrow_mut();
//                             for (_, mut user) in user_map.iter() {
//                                 if user.email == email {
//                                     user.totalDonate += amount;
//                                     return true;
//                                 }
//                             }
//                             false
//                         });
//                  let esuser_found = EsUSER_MAP.with(|p| {
//                             let mut user_map = p.borrow_mut();
//                             for (_, mut user) in user_map.iter() {
//                                 if user.email == email2 {
//                                     user.totalDonate += amount;
//                                     return true;
//                                 }
//                             }
//                             false
//                         });
//                         Ok(userResult::Success("bağışınız yapılmıştır".to_string()))


// }

#[ic_cdk_macros::query]
fn list_donate()->Vec<Donation> {
    let mut donaties = Vec::new();
    DONATION_MAP.with(|p|{
        let user_map=p.borrow();
       
    for (_, user) in user_map.iter() {
    donaties.push(user.clone());
}
});
donaties   
} 

#[ic_cdk_macros::query]
fn list_top_donors() -> Vec<String> {
    let mut top_donors: Vec<String> = Vec::new();
    let mut max_donation = 0;

    DONATION_MAP.with(|p| {
        let donation_map = p.borrow();
        for (_, donation) in donation_map.iter() {
            if donation.amount > max_donation {
                max_donation = donation.amount;
                top_donors.clear();
                top_donors.push(donation.donor_user.clone());
            } else if donation.amount == max_donation {
                top_donors.push(donation.donor_user.clone());
            }
        }
    });

    top_donors
}

#[ic_cdk::query]
fn get_donations_by_email(email: String) -> Vec<Donation> {
    let mut result = Vec::new();

    DONATION_MAP.with(|p| {
        let donation_map = p.borrow();
        for (_, donation) in donation_map.iter() {
            if donation.donor_user == email {
                result.push(donation.clone());
            }
        }
    });

    result
}
#[ic_cdk::update]
fn login_user(email: String, password: String) -> Result<userResult, userError> {
    USER_MAP.with(|user_map_ref| {
        let user_map = user_map_ref.borrow();
        for (_, user) in user_map.iter() {
            if user.email == email {
                if user.password == password {
                    return Ok(userResult::Success("Giriş başarılı.".to_string()));
                } else {
                    return Err(userError::incorrectPassword("E-mail or password is incorrect.".to_string()));
                }
            }
        }
        Err(userError::incorrectEmail("E-mail or password is incorrect".to_string()))
    })
}

#[ic_cdk_macros::query]
fn get_user_by_email(email: String) -> Option<User> {
    USER_MAP.with(|user_map_ref| {
        let user_map = user_map_ref.borrow();
        for (_, user) in user_map.iter() {
            if user.email == email {
                return Some(user.clone());
            }
        }
        None
    })
}



