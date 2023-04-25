#![no_main]
use libfuzzer_sys::fuzz_target;
use base32ct::{Base32Unpadded, Encoding};

fuzz_target!(|value: &[u8]| {
    let mut encode_buf = vec![0; Base32Unpadded::encoded_len(value)];
    let _ = Base32Unpadded::encode(value, encode_buf.as_mut_slice()).unwrap();
});