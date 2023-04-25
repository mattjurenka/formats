#![no_main]
use libfuzzer_sys::fuzz_target;
use base32ct::{Base32Upper, Encoding};

fuzz_target!(|value: &[u8]| {
    let mut encode_buf = vec![0; Base32Upper::encoded_len(value)];
    let mut decode_buf = vec![0; Base32Upper::encoded_len(value)];
    let encoded = Base32Upper::encode(value, encode_buf.as_mut_slice()).unwrap();
    let decoded = Base32Upper::decode(encoded, decode_buf.as_mut_slice()).unwrap();
    assert_eq!(decoded, value);
});