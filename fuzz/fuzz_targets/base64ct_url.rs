#![no_main]
use libfuzzer_sys::fuzz_target;
use base64ct::{Base64Url, Encoding};

fuzz_target!(|value: &[u8]| {
    let mut encode_buf = vec![0; Base64Url::encoded_len(value)];
    let mut decode_buf = vec![0; Base64Url::encoded_len(value)];
    let encoded = Base64Url::encode(value, encode_buf.as_mut_slice()).unwrap();
    let decoded = Base64Url::decode(encoded, decode_buf.as_mut_slice()).unwrap();
    assert_eq!(decoded, value);
});2