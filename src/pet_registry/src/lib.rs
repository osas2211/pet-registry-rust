#[macro_use]
extern crate serde;
use candid::{Decode, Encode, Principal};
use ic_cdk::api::time;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};
use std::collections::HashMap;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct PetRecord {
    id: u64,
    breed: String,
    sex: String,
    date_of_birth: String,
    image_url: String,
    created_at: u64,
    updated_at: Option<u64>,
    transfer_to: Option<Principal>,
    owner_details: OwnerData,
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct Account {
    new_user_instance: Option<Principal>
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct ID {
    pet_id: u64
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct OwnerData {
    id: Principal,
    name: String,
    address: String,
    phone_number: String,
}

impl Default for OwnerData {
    fn default() -> Self {
        OwnerData {
            id: Principal::anonymous(),
            name: format!(""),
            address: format!(""),
            phone_number: format!(""),
        }
    }
}

// a trait that must be implemented for a struct that is stored in a stable struct
impl Storable for PetRecord {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

// another trait that must be implemented for a struct that is stored in a stable struct
impl BoundedStorable for PetRecord {
    const MAX_SIZE: u32 = 2048;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
struct UsersPetData {
    pending: HashMap<Principal, Vec<u64>>,
    pets:  HashMap<Principal, Vec<u64>>
}

impl Default for UsersPetData {
    fn default() -> Self {
        UsersPetData{
            pending: HashMap::new(),
            pets: HashMap::new()
        }
    }
}

impl UsersPetData{
    pub fn add_pet(&mut self, pet_id: &u64, to: Principal)-> Result<(), String> {
        // get the pets record from storage
        let pets = self
        .pets
        .get_mut(&to);

        match pets {
            Some(pets) => {
                if pets.contains(&pet_id) {
                    return Err(format!(
                        "Pet record with id already exists {}",
                        pet_id
                    ));
                }
                pets.push(*pet_id);
                Ok(())
            },
            None => {
                let mut pets: Vec<u64> = Vec::new();
                pets.push(*pet_id);
                self.pets.insert(to, pets);
                Ok(())
            }
        }
        // self.added.push(*pet_id);
    }

    pub fn remove_pet(&mut self, pet_id: &u64, to: Principal)-> Result<(), String> {
        // get the pending pets record from storage
        let pets = self
        .pets
        .get_mut(&to)
        .ok_or_else(|| format!("You do not have any pet record"))?;

        // check that user list contains the pet id to be removed
        if !pets.contains(&pet_id) {
            return Err(format!(
                "Pet id not found {}",
                pet_id
            ));
        }
        // get pet id index and remove pet.
        let index = pets.iter().position(|x: &u64| *x == *pet_id).unwrap();
        pets.remove(index);
        Ok(())
    }

    pub fn get_pets(&self, account: Principal)-> Vec<u64> {
        // get the pending pets record from storage
        let pets = self.pets.get(&account);
        match pets {
            Some(pets) => {
                pets.to_vec()
            },
            None => {
                [].to_vec()
            }
        }
     }

    pub fn add_pending(&mut self, pet_id: &u64, to: Principal)-> Result<(), String> {
        // get the pending pets record from storage
        let pending_pets = self
        .pending
        .get_mut(&to);

        // match pets and create a new record if user does not have record. 
        // check for errors 
        match pending_pets {
            Some(pets) => {
                if pets.contains(&pet_id) {
                    return Err(format!(
                        "Pet is already pending {}",
                        pet_id
                    ));
                }
                pets.push(*pet_id);
                Ok(())
            },
            None => {
                let mut pets: Vec<u64> = Vec::new();
                pets.push(*pet_id);
                self.pending.insert(to, pets);
                Ok(())
            }
        }
    }
    pub fn remove_pending(&mut self, pet_id: &u64, to: Principal)-> Result<(), String> {
        // get the pending pets record from storage
        let pending_pets = self
        .pending
        .get_mut(&to)
        .ok_or_else(|| format!("You do not have any pending pet"))?;

        // check that user list contains the pet id to be removed
        if !pending_pets.contains(&pet_id) {
            return Err(format!(
                "Pet id not found {}",
                pet_id
            ));
        }
        // get pet id index and remove pet.
        let index = pending_pets.iter().position(|x: &u64| *x == *pet_id).unwrap();
        pending_pets.remove(index);
        Ok(())
    }

    pub fn get_pending_pets(&self, account: Principal)-> Vec<u64> {
        // get the pending pets record from storage
        let pending_pets = self.pending.get(&account);
        match pending_pets {
            Some(pets) => {
                pets.to_vec()
            },
            None => {
                [].to_vec()
            }
        }
    }
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static STORAGE: RefCell<StableBTreeMap<u64, PetRecord, Memory>> =
        RefCell::new(StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));

    static PETSDATA: RefCell<UsersPetData>= RefCell::default();
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct PetPayload {
    name: String,
    breed: String,
    sex: String,
    date_of_birth: String,
    image_url: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct OwnerPayload {
    name: String,
    address: String,
    phone_number: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct AddPetPayload {
    pet_payload: PetPayload,
    owner_payload: OwnerPayload,
}

#[ic_cdk::query]
fn get_pet_record(id: ID) -> Result<PetRecord, String> {
    match _get_record(&id.pet_id) {
        Some(record) => Ok(record),
        None => Err(format!("pet record with id={} not found", id.pet_id)),
    }
}

#[ic_cdk::update]
fn add_pet_record(payload: AddPetPayload, other_user: Account) -> Result<Option<PetRecord>, String> {
    // set caller to ic_cdk caller instance
    let mut caller: Principal = ic_cdk::caller();

    // check if the other_user flag is activated
    if other_user.new_user_instance.is_some() {
        caller = other_user.new_user_instance.unwrap();
    }
    // get new id
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");
    // generate owner details
    let owner_details = OwnerData {
        id: caller,
        name: payload.owner_payload.name,
        address: payload.owner_payload.address,
        phone_number: payload.owner_payload.phone_number,
    };
    // generate record information
    let record = PetRecord {
        id,
        breed: payload.pet_payload.breed,
        sex: payload.pet_payload.sex,
        date_of_birth: payload.pet_payload.date_of_birth,
        image_url: payload.pet_payload.image_url,
        transfer_to: None,
        owner_details,
        created_at: time(),
        updated_at: None,
    };
    // add pet to user pet list and handle errors
    let add_result = PETSDATA.with(|storage| storage.borrow_mut().add_pet(&id, caller));
    match add_result {
        Ok(_value) => {},
        Err(error) => return Err(error),
    }
    // update records
    do_insert(&record);
    Ok(Some(record))
}

#[ic_cdk::update]
fn update_pet_record(id: ID, payload: PetPayload, other_user: Account) -> Result<PetRecord, String> {
    // set caller to ic_cdk caller instance
    let mut caller: Principal = ic_cdk::caller();

    // check if the other_user flag is activated
    if other_user.new_user_instance.is_some() {
        caller = other_user.new_user_instance.unwrap();
    }
    match STORAGE.with(|service| service.borrow().get(&id.pet_id)) {
        Some(mut record) => {
            // check if caller is owner
            if record.owner_details.id != caller {
                return Err(format!("Only pet owner can edit record"));
            }
            // update records
            record.breed = payload.breed;
            record.date_of_birth = payload.date_of_birth;
            record.sex = payload.sex;
            record.image_url = payload.image_url;
            record.updated_at = Some(time());
            do_insert(&record);
            Ok(record)
        }
        None => Err(format!(
            "couldn't update pet record with id={}. record not found",
            id.pet_id
        )),
    }
}

#[ic_cdk::update]
fn update_owner_record(id: ID, payload: OwnerPayload, other_user: Account) -> Result<PetRecord, String> {
    // set caller to ic_cdk caller instance
    let mut caller: Principal = ic_cdk::caller();

    // check if the other_user flag is activated
    if other_user.new_user_instance.is_some() {
        caller = other_user.new_user_instance.unwrap();
    }
    match STORAGE.with(|service| service.borrow().get(&id.pet_id)) {
        Some(mut record) => {
            // check if caller is owner
            if record.owner_details.id != caller {
                return Err(format!("Only pet owner can edit record"));
            }
            // update records
            record.owner_details.name = payload.name;
            record.owner_details.address = payload.address;
            record.owner_details.phone_number = payload.phone_number;
            record.updated_at = Some(time());
            do_insert(&record);
            Ok(record)
        }
        None => Err(format!(
            "couldn't update pet record with id={}. record not found",
            id.pet_id
        )),
    }
}

#[ic_cdk::update]
fn transfer_pet(id: ID, to: Principal, other_user: Account) -> Result<PetRecord, String> {
    // set caller to ic_cdk caller instance
    let mut caller: Principal = ic_cdk::caller();

    // check if the other_user flag is activated
    if other_user.new_user_instance.is_some() {
        caller = other_user.new_user_instance.unwrap();
    }
    match STORAGE.with(|service| service.borrow().get(&id.pet_id)) {
        Some(mut record) => {
            // check if caller is owner
            if record.owner_details.id != caller {
                return Err(format!("Only pet owner can edit record"));
            }
            // check if pet is assigned to anyone
            if record.transfer_to.is_some() {
                return Err(format!(
                    "pet record with id={} is already assigned for transfer",
                    id.pet_id
                ));
            }
            // update records
            record.transfer_to = Some(to);
            record.updated_at = Some(time());
            // add pet to the list of users pending pets amd handle errors.
            let add_result = PETSDATA.with(|storage| storage.borrow_mut().add_pending(&id.pet_id, to));
            match add_result {
                Ok(_value) => {},
                Err(error) => return Err(error),
            }
            // update records
            do_insert(&record);
            Ok(record)
        }
        None => Err(format!(
            "couldn't update pet record with id={}. record not found",
            id.pet_id
        )),
    }
}

#[ic_cdk::update]
fn revoke_transfer(id: ID,other_user: Account ) -> Result<PetRecord, String> {
    // set caller to ic_cdk caller instance
    let mut caller: Principal = ic_cdk::caller();

    // check if the other_user flag is activated
    if other_user.new_user_instance.is_some() {
        caller = other_user.new_user_instance.unwrap();
    }
    match STORAGE.with(|service| service.borrow().get(&id.pet_id)) {
        Some(mut record) => {
            // check if caller is owner
            if record.owner_details.id != caller {
                return Err(format!("Only pet owner can edit record"));
            }
            // check if pet is assigned to anyone
            if record.transfer_to.is_none() {
                return Err(format!(
                    "pet record with id={} not assigned for transfer",
                    id.pet_id
                ));
            }
            let to = record.transfer_to.unwrap();
            // update records
            record.transfer_to = None;
            record.updated_at = Some(time());
            // remove pet id from list of users pending pets and handle errors
            let remove_result = PETSDATA.with(|storage| storage.borrow_mut().remove_pending(&id.pet_id, to));
            match remove_result {
                Ok(_value) => {},
                Err(error) => return Err(error),
            }
            // update records
            do_insert(&record);
            Ok(record)
        }
        None => Err(format!(
            "couldn't update pet record with id={}. record not found",
            id.pet_id
        )),
    }
}

#[ic_cdk::update]
fn claim_pet(id: ID, payload: OwnerPayload, other_user: Account) -> Result<PetRecord, String> {
    // set caller to ic_cdk caller instance
    let mut caller: Principal = ic_cdk::caller();

    // check if the other_user flag is activated
    if other_user.new_user_instance.is_some() {
        caller = other_user.new_user_instance.unwrap();
    }

    match STORAGE.with(|service| service.borrow().get(&id.pet_id)) {
        Some(mut record) => {
            // check if pet is assigned to anyone
            if record.transfer_to.is_none() {
                return Err(format!(
                    "couldn't claim pet record with id={}. pet not assigned for transfer",
                    id.pet_id
                ));
            }
            // check if pet is assigned to caller
            if record.transfer_to != Some(caller) {
                return Err(format!(
                    "couldn't claim pet record with id={}. pet not assigned to you",
                    id.pet_id
                ));
            }

            let past_owner = record.owner_details.id;
            // update records
            record.owner_details.id = caller;
            record.owner_details.name = payload.name;
            record.owner_details.address = payload.address;
            record.owner_details.phone_number = payload.phone_number;
            record.updated_at = Some(time());

            // remove pet from past owner list and handle errors
            let remove_result_1 =PETSDATA.with(|storage| storage.borrow_mut().remove_pet(&id.pet_id, past_owner));
            match remove_result_1 {
                Ok(_value) => {},
                Err(error) => return Err(error),
            }

            // remove pet from user pending list pets and handle errors
            let remove_result_2 = PETSDATA.with(|storage| storage.borrow_mut().remove_pending(&id.pet_id, caller));
            match remove_result_2 {
                Ok(_value) => {},
                Err(error) => return Err(error),
            }

            // add pet to user pet list and handle errors
            let add_result =PETSDATA.with(|storage| storage.borrow_mut().add_pet(&id.pet_id, caller));
            match add_result {
                Ok(_value) => {},
                Err(error) => return Err(error),
            }

            // update records
            do_insert(&record);
            Ok(record)
        }
        None => Err(format!(
            "couldn't update pet record with id={}. record not found",
            id.pet_id
        )),
    }
}

#[ic_cdk::update]
fn delete_pet_record(id: ID, other_user: Account) -> Result<PetRecord, String> {
    // set caller to ic_cdk caller instance
    let mut caller: Principal = ic_cdk::caller();

    // check if the other_user flag is activated
    if other_user.new_user_instance.is_some() {
        caller = other_user.new_user_instance.unwrap();
    }
    match STORAGE.with(|service| service.borrow_mut().get(&id.pet_id)) {
        Some(record) => {
            // check if caller is owner
            if record.owner_details.id != caller {
                return Err(format!("Only pet owner can edit record"));
            }
            // check if pet is assigned for transfer
            if record.transfer_to.is_some() {
                return Err(format!(
                    "couldn't delete pet record with id={}. pet is already assigned for transfer, revoke transfer and try to delete again.",
                    id.pet_id
                ));
            }

            // remove pet from user list and handle errors
            let remove_result =PETSDATA.with(|storage| storage.borrow_mut().remove_pet(&id.pet_id, caller));
            match remove_result {
                Ok(_value) => {},
                Err(error) => return Err(error),
            }

            // remove from registry
            STORAGE.with(|service| service.borrow_mut().remove(&id.pet_id));
            Ok(record)
        }
        None => Err(format!(
            "couldn't delete pet record with id={}. record not found.",
            id.pet_id
        )),
    }
}

#[ic_cdk::query]
fn view_pending_pets(user: Principal) -> Vec<u64> {
    PETSDATA.with(|storage| storage.borrow_mut().get_pending_pets(user))
}

#[ic_cdk::query]
fn view_pets_list(user: Principal) -> Vec<u64> {
    PETSDATA.with(|storage| storage.borrow_mut().get_pets(user))
}

#[ic_cdk::query]
fn get_your_principal() -> Principal {
    ic_cdk::caller()
}

// helper method to perform insert.
fn do_insert(record: &PetRecord) {
    STORAGE.with(|service| service.borrow_mut().insert(record.id, record.clone()));
}

// a helper method to get a pet record by id.
fn _get_record(id: &u64) -> Option<PetRecord> {
    STORAGE.with(|service| service.borrow().get(id))
}

// need this to generate candid
ic_cdk::export_candid!();
