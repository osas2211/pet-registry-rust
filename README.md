# PET REGISTRY

This canister helps to keep track of pets in a community.

Owning a pet is a big responsibility, as you need to make sure your pet is registered with us, desexed and microchipped. Registration means that we have a record of your pet's name and your contact details, so that we can return them to you if we find them. It also helps keep the community safe.

Register Features:

- Users can add and delete their pet records to the register.
- Users can update pet information on the register.
- Users can update their contact information on the register.
- Users can also transfer their pet information to another user.

To learn more before you start working with pet_registry, see the following documentation available online:

- [Quick Start](https://internetcomputer.org/docs/quickstart/quickstart-intro)
- [SDK Developer Tools](https://internetcomputer.org/docs/developers-guide/sdk-guide)
- [Rust Canister Devlopment Guide](https://internetcomputer.org/docs/rust-guide/rust-intro)
- [ic-cdk](https://docs.rs/ic-cdk)
- [ic-cdk-macros](https://docs.rs/ic-cdk-macros)
- [Candid Introduction](https://internetcomputer.org/docs/candid-guide/candid-intro)
- [JavaScript API Reference](https://erxue-5aaaa-aaaab-qaagq-cai.raw.icp0.io)

If you want to start working on your project right away, you might want to try the following commands:

```bash
cd pet_registry/
dfx help
dfx canister --help
```

## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background --clean

# Deploys your canisters to the replica and generates your candid interface
npm run gen-deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.

If you have made changes to your backend canister, you can generate a new candid interface with

```bash
npm run generate
```

at any time. This is recommended before starting the frontend development server, and will be run automatically any time you run `npm run gen-deploy`.

## Testing

N/B update actions can only be performed by pet owner.

1. Add a new pet to the registry
2. Try to update your contact information
3. Try to update your pets records
4. Try to transfer your pet information to another user
5. User can then proceed to claim the pet using the pet id and their new information.
6. Try to delete pet.
