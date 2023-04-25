#![no_main]
use libfuzzer_sys::fuzz_target;
use base16ct::upper;

fuzz_target!(|value: &[u8]| {
    let mut encode_buf = vec![0; value.len() * 2];
    let mut decode_buf = vec![0; value.len()];
    let encoded = upper::encode_str(value, encode_buf.as_mut_slice()).unwrap();
    let decoded = upper::decode(encoded, decode_buf.as_mut_slice()).unwrap();
    assert_eq!(value, decoded);
});