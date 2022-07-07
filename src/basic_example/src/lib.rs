use stable_structures::{Ic0StableMemory, RestrictedMemory, StableBTreeMap};
use std::cell::RefCell;

type Memory = RestrictedMemory<Ic0StableMemory>;

// `StableBTreeMap` requires specifying the maximum size in bytes that
// keys/values can hold. In this particular example, we limit the key size to 10
// bytes and limit the value to a blob of 100 bytes.
//
// Note that an entry in the map always takes up the maximum size in memory
// (i.e. MAX_KEY_SIZE + MAX_VALUE_SIZE), so you shouldn't specify sizes here
// that are larger than necessary.
//
// If your entries vary a lot in size, consider bucketizing them. For instance,
// you can create two different maps, one for holding "small" entries, and
// another for holding "large" entries.
const MAX_KEY_SIZE: u32 = 10;
const MAX_VALUE_SIZE: u32 = 100;

thread_local! {

    // Initialize a `StableBTreeMap`. We're providing the map a `RestrictedMemory`,
    // which allows us to divide the stable memory into non-intersecting ranges
    // so that we can store multiple stable structures if we later wish.
    //
    // In this case, this map is given the range [0, 99], so it has access to the first
    // 100 pages in stable memory. Note that a page is 64KiB.
    //
    // Note that we can safely increase the range at any time (e.g. from 0..99 to 0..999)
    // to give the map more space to grow.
    static MAP: RefCell<StableBTreeMap<Memory, String, Vec<u8>>> = RefCell::new(
        StableBTreeMap::init(RestrictedMemory::new(Ic0StableMemory, 0..99), MAX_KEY_SIZE, MAX_VALUE_SIZE)
    );
}

// Retrieves the value associated with the given key if it exists.
#[ic_cdk_macros::query]
fn get(key: String) -> Option<Vec<u8>> {
    MAP.with(|p| p.borrow().get(&key))
}

// Inserts an entry into the map and returns the previous value of the key if it exists.
#[ic_cdk_macros::update]
fn insert(key: String, value: Vec<u8>) -> Option<Vec<u8>> {
    MAP.with(|p| p.borrow_mut().insert(key, value)).unwrap()
}
