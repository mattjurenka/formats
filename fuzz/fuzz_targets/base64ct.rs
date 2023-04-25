#![no_main]
use libfuzzer_sys::fuzz_target;
use base64ct::{Base64, Encoding};

fuzz_target!(|value: &[u8]| {
    let mut encode_buf = vec![0; Base64::encoded_len(value)];
    let mut decode_buf = vec![0; Base64::encoded_len(value)];
    let encoded = Base64::encode(value, encode_buf.as_mut_slice()).unwrap();
    let decoded = Base64::decode(encoded, decode_buf.as_mut_slice()).unwrap();
    assert_eq!(decoded, value);
});