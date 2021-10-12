fn hex_to_base64(hex: String) -> String {
    hex
}

fn main() {
    println!("Hello, world!");
}

#[test]
fn test_hex_to_base64() {
    let input = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d".to_string();
    let output = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t".to_string();
    assert_eq!(hex_to_base64(input), output)
}