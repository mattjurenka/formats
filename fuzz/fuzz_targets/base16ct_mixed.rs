#![no_main]
use libfuzzer_sys::fuzz_target;
use base16ct::mixed;
use base16ct::upper;
use base16ct::lower;

fuzz_target!(|value: &[u8]| {
    let mut encode_buf = vec![0; value.len() * 2];
    let mut decode_buf = vec![0; value.len()];
    let encoded = upper::encode(value, encode_buf.as_mut_slice()).unwrap();
    let decoded = mixed::decode(encoded, decode_buf.as_mut_slice()).unwrap();
    assert_eq!(value, decoded);

    let mut encode_buf = vec![0; value.len() * 2];
    let mut decode_buf = vec![0; value.len()];
    let encoded = lower::encode(value, encode_buf.as_mut_slice()).unwrap();
    let decoded = mixed::decode(encoded, decode_buf.as_mut_slice()).unwrap();
    assert_eq!(value, decoded);
});