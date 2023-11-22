export const idlFactory = ({ IDL }) => {
  const OwnerPayload = IDL.Record({
    'name' : IDL.Text,
    'address' : IDL.Text,
    'phone_number' : IDL.Text,
  });
  const PetPayload = IDL.Record({
    'sex' : IDL.Text,
    'image_url' : IDL.Text,
    'name' : IDL.Text,
    'breed' : IDL.Text,
    'date_of_birth' : IDL.Text,
  });
  const CreatePetPayload = IDL.Record({
    'owner_payload' : OwnerPayload,
    'pet_payload' : PetPayload,
  });
  const OwnerData = IDL.Record({
    'id' : IDL.Principal,
    'name' : IDL.Text,
    'address' : IDL.Text,
    'phone_number' : IDL.Text,
  });
  const PetRecord = IDL.Record({
    'id' : IDL.Nat64,
    'sex' : IDL.Text,
    'updated_at' : IDL.Opt(IDL.Nat64),
    'image_url' : IDL.Text,
    'created_at' : IDL.Nat64,
    'transfer_to' : IDL.Opt(IDL.Principal),
    'breed' : IDL.Text,
    'date_of_birth' : IDL.Text,
    'owner_details' : OwnerData,
  });
  const Result = IDL.Variant({ 'Ok' : PetRecord, 'Err' : IDL.Text });
  const OwnerUpdatePayload = IDL.Record({
    'name' : IDL.Opt(IDL.Text),
    'address' : IDL.Opt(IDL.Text),
    'phone_number' : IDL.Opt(IDL.Text),
  });
  const UpdatePetPayload = IDL.Record({
    'sex' : IDL.Opt(IDL.Text),
    'image_url' : IDL.Opt(IDL.Text),
    'name' : IDL.Opt(IDL.Text),
    'breed' : IDL.Opt(IDL.Text),
    'date_of_birth' : IDL.Opt(IDL.Text),
  });
  return IDL.Service({
    'add_pet_record' : IDL.Func([CreatePetPayload], [IDL.Opt(PetRecord)], []),
    'claim_pet' : IDL.Func([IDL.Nat64, OwnerPayload], [Result], []),
    'delete_pet_record' : IDL.Func([IDL.Nat64], [Result], []),
    'get_pet_record' : IDL.Func([IDL.Nat64], [Result], ['query']),
    'transfer_pet' : IDL.Func([IDL.Nat64, IDL.Principal], [Result], []),
    'update_owner_record' : IDL.Func(
        [IDL.Nat64, OwnerUpdatePayload],
        [Result],
        [],
      ),
    'update_pet_record' : IDL.Func([IDL.Nat64, UpdatePetPayload], [Result], []),
  });
};
export const init = ({ IDL }) => { return []; };
