use super::decode_url_component;
use super::hex::decoded_byte;

#[test]
fn decoded_byte_reads_upper_and_lower_hex_digits() {
    assert_eq!(decoded_byte(b'4', b'1'), Some(b'A'));
    assert_eq!(decoded_byte(b'a', b'f'), Some(0xaf));
    assert_eq!(decoded_byte(b'G', b'1'), None);
}

#[test]
fn decode_url_component_decodes_hex_and_plus() {
    assert_eq!(
        decode_url_component("https%3A%2F%2Fexample.com%2Fa+b"),
        "https://example.com/a b"
    );
}
