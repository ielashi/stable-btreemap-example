# UPDATE: The examples have been migrated to https://github.com/dfinity/stable-structures and this repo is now archived.

# `StableBTreeMap` in Canisters

`StableBTreeMap` is a data structure that was developed for the bitcoin integration.

It is a data structure that has a similar interface to a standard `BTreeMap`, but the data
lives entirely in stable memory. This data structure allows canisters to have K/V stores
that can grow to several gigabytes in size, without having to define `pre_upgrade`/`post_upgrade`
hooks in the canister.

While this data structure has been tested with tens of millions of keys, its use in
canisters is still considered experimental.

This repo contains two examples:

## Basic Example

This example showcases how to initialize a `StableBTreeMap` that holds primitive types.

```bash
# Start the replica, running in the background
dfx start --background

# Deploys the examples.
dfx deploy

# Insert some data into the basic_example canister.
dfx canister call basic_example insert '("alice", blob "12341234")'
dfx canister call basic_example insert '("bob", blob "789789789")'

# Upgrade the canister, which clears all the data in the heap.
dfx deploy --upgrade-unchanged basic_example

# Even though the canister has been upgraded and its heap is cleared,
# querying the canister should still return the data stored prior to
# the upgrade.
dfx canister call basic_example get '("alice")'
> (opt blob "12341234") 

dfx canister call basic_example get '("bob")'
> (opt blob "789789789")
```

# Custom Types Example

`StableBTreeMap` supports generics, and you can use it to store your own `struct`s.
This example showcases how you can define a struct and be able to store it in a
`StableBTreeMap`.

We define a `UserProfile` struct in this example that stores a user's name and age.

Example usage:

```bash
# Insert some data into the basic_example canister.
dfx canister call custom_types_example insert '(1, record { age = 32; name = "Some Name"})'
dfx canister call custom_types_example insert '(2, record { age = 48; name = "Other Name"})'

# Upgrade the canister, which clears all the data in the heap.
dfx deploy --upgrade-unchanged custom_types_example

# Even though the canister has been upgraded and its heap is cleared,
# querying the canister should still return the data stored prior to
# the upgrade.
$ dfx canister call custom_types_example get '(1)'
> (opt record { age = 32 : nat8; name = "Some Name" })

dfx canister call custom_types_example get '(2)'
> (opt record { age = 48 : nat8; name = "Other Name" })
```
