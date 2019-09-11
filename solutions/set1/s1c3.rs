use rawmem::Rawmem;

fn main() {
    let input = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";

    // Show only printable character sequences and such which contain whitespaces
    let is_printable_ascii = |c: char| c.is_ascii_graphic() || c.is_ascii_whitespace();
    let data = Rawmem::from_hex(&input);
    for key in 0u8..=255u8 {
        let cleartext = data.single_byte_xor(key).as_ascii();
        if cleartext.chars().all(is_printable_ascii)
            && cleartext.chars().any(|c| c.is_ascii_whitespace()) {
            println!("0x{:02X}: {}", key, cleartext);
        }
    }
}
