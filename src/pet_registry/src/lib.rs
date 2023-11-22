#[macro_use]
extern crate serde;
use candid::{Decode, Encode, Principal};
use ic_cdk::api::time;

use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

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
struct UpdatePetPayload {
    name: Option<String>,
    breed: Option<String>,
    sex: Option<String>,
    date_of_birth: Option<String>,
    image_url: Option<String>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct OwnerPayload {
    name: String,
    address: String,
    phone_number: String,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct OwnerUpdatePayload {
    name: Option<String>,
    address: Option<String>,
    phone_number: Option<String>,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct CreatePetPayload {
    pet_payload: PetPayload,
    owner_payload: OwnerPayload,
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct PetUpdatePayload {
    pet_payload: UpdatePetPayload,
    owner_payload: OwnerUpdatePayload,
}

#[ic_cdk::query]
fn get_pet_record(id: u64) -> Result<PetRecord, String> {
    match _get_record(&id) {
        Some(record) => Ok(record),
        None => Err(format!("pet record with id={} not found", id)),
    }
}

#[ic_cdk::update]
fn add_pet_record(payload: CreatePetPayload) -> Option<PetRecord> {
    let id = ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter");

    let owner_details = OwnerData {
        id: ic_cdk::caller(),
        name: payload.owner_payload.name,
        address: payload.owner_payload.address,
        phone_number: payload.owner_payload.phone_number,
    };

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

    do_insert(&record);
    Some(record)
}

#[ic_cdk::update]
fn update_pet_record(id: u64, payload: UpdatePetPayload) -> Result<PetRecord, String> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut record) => {
            // check if caller is owner
            if record.owner_details.id != ic_cdk::caller() {
                return Err(format!("Only pet owner can edit record"));
            }
            match payload.breed {
                Some(breed) => record.breed = breed,
                None => (),
            }
            match payload.date_of_birth {
                Some(date_of_birth) => record.date_of_birth = date_of_birth,
                None => (),
            }
            match payload.sex {
                Some(sex) => record.sex = sex,
                None => (),
            }
            match payload.image_url {
                Some(image_url) => record.image_url = image_url,
                None => (),
            }

            record.updated_at = Some(time());
            do_insert(&record);
            Ok(record)
        }
        None => Err(format!(
            "couldn't update pet record with id={}. record not found",
            id
        )),
    }
}

#[ic_cdk::update]
fn update_owner_record(id: u64, payload: OwnerUpdatePayload) -> Result<PetRecord, String> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut record) => {
            // check if caller is owner
            if record.owner_details.id != ic_cdk::caller() {
                return Err(format!("Only pet owner can edit record"));
            }

            match payload.name {
                Some(name) => record.owner_details.name = name,
                None => (),
            }
            match payload.address {
                Some(address) => record.owner_details.address = address,
                None => (),
            }
            match payload.phone_number {
                Some(phone_number) => record.owner_details.phone_number = phone_number,
                None => (),
            }

            record.updated_at = Some(time());
            do_insert(&record);
            Ok(record)
        }
        None => Err(format!(
            "couldn't update pet record with id={}. record not found",
            id
        )),
    }
}

#[ic_cdk::update]
fn transfer_pet(id: u64, to: Principal) -> Result<PetRecord, String> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut record) => {
            // check if caller is owner
            if record.owner_details.id != ic_cdk::caller() {
                return Err(format!("Only pet owner can edit record"));
            }
            record.transfer_to = Some(to);
            record.updated_at = Some(time());
            do_insert(&record);
            Ok(record)
        }
        None => Err(format!(
            "couldn't update pet record with id={}. record not found",
            id
        )),
    }
}

#[ic_cdk::update]
fn claim_pet(id: u64, payload: OwnerPayload) -> Result<PetRecord, String> {
    match STORAGE.with(|service| service.borrow().get(&id)) {
        Some(mut record) => {
            // check if pet is assigned to anyone
            if record.transfer_to.is_none() {
                return Err(format!(
                    "couldn't claim pet record with id={}. pet not assigned for transfer",
                    id
                ));
            }

            // check if pet is assigned to caller
            if record.transfer_to != Some(ic_cdk::caller()) {
                return Err(format!(
                    "couldn't claim pet record with id={}. pet not assigned to you",
                    id
                ));
            }

            // update records
            record.owner_details.id = ic_cdk::caller();
            record.owner_details.name = payload.name;
            record.owner_details.address = payload.address;
            record.owner_details.phone_number = payload.phone_number;
            record.updated_at = Some(time());
            do_insert(&record);
            Ok(record)
        }
        None => Err(format!(
            "couldn't update pet record with id={}. record not found",
            id
        )),
    }
}

#[ic_cdk::update]
fn delete_pet_record(id: u64) -> Result<PetRecord, String> {
    match STORAGE.with(|service| service.borrow_mut().get(&id)) {
        Some(record) => {
            // check if caller is owner
            if record.owner_details.id != ic_cdk::caller() {
                return Err(format!("Only pet owner can edit record"));
            }

            // remove from registry
            STORAGE.with(|service| service.borrow_mut().remove(&id));
            Ok(record)
        }

        None => Err(format!(
            "couldn't delete pet record with id={}. record not found.",
            id
        )),
    }
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
