import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface CreatePetPayload {
  'owner_payload' : OwnerPayload,
  'pet_payload' : PetPayload,
}
export interface OwnerData {
  'id' : Principal,
  'name' : string,
  'address' : string,
  'phone_number' : string,
}
export interface OwnerPayload {
  'name' : string,
  'address' : string,
  'phone_number' : string,
}
export interface OwnerUpdatePayload {
  'name' : [] | [string],
  'address' : [] | [string],
  'phone_number' : [] | [string],
}
export interface PetPayload {
  'sex' : string,
  'image_url' : string,
  'name' : string,
  'breed' : string,
  'date_of_birth' : string,
}
export interface PetRecord {
  'id' : bigint,
  'sex' : string,
  'updated_at' : [] | [bigint],
  'image_url' : string,
  'created_at' : bigint,
  'transfer_to' : [] | [Principal],
  'breed' : string,
  'date_of_birth' : string,
  'owner_details' : OwnerData,
}
export type Result = { 'Ok' : PetRecord } |
  { 'Err' : string };
export interface UpdatePetPayload {
  'sex' : [] | [string],
  'image_url' : [] | [string],
  'name' : [] | [string],
  'breed' : [] | [string],
  'date_of_birth' : [] | [string],
}
export interface _SERVICE {
  'add_pet_record' : ActorMethod<[CreatePetPayload], [] | [PetRecord]>,
  'claim_pet' : ActorMethod<[bigint, OwnerPayload], Result>,
  'delete_pet_record' : ActorMethod<[bigint], Result>,
  'get_pet_record' : ActorMethod<[bigint], Result>,
  'transfer_pet' : ActorMethod<[bigint, Principal], Result>,
  'update_owner_record' : ActorMethod<[bigint, OwnerUpdatePayload], Result>,
  'update_pet_record' : ActorMethod<[bigint, UpdatePetPayload], Result>,
}
