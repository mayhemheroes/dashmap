#![no_main]
#![feature(array_chunks)]
use libfuzzer_sys::fuzz_target;

use dashmap::DashMap;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let mapping = DashMap::with_capacity(128);
    let mut count = 0;
    for [key, val] in data.array_chunks() {
        if !mapping.contains_key(key) {
            count = count + 1;
        }
        mapping.insert(key, val);
    }
    assert_eq!(mapping.iter().count(), count);
});
