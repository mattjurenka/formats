#![no_main]
use libfuzzer_sys::fuzz_target;
use base16ct::lower;

// TODO: add docs about arbitrary
fuzz_target!(|value: &[u8]| {
    let mut encode_buf = vec![0; value.len() * 2];
    let mut decode_buf = vec![0; value.len()];
    let encoded = lower::encode_str(value, encode_buf.as_mut_slice()).unwrap();
    let decoded = lower::decode(encoded, decode_buf.as_mut_slice()).unwrap();
    assert_eq!(value, decoded);
});