#![no_main]
use libfuzzer_sys::fuzz_target;

use dashmap::DashSet;

fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
    let set = DashSet::with_capacity(128);
    let mut count = 0;
    for byte in data {
        if !set.contains(byte) {
            count = count + 1;
        }
        set.insert(byte);
    }
    assert_eq!(set.iter().count(), count);
});
