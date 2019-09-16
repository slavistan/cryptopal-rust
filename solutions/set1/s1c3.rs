use rawmem::Rawmem;
use rawmem::cracker;

fn main() {
    let input_data = Rawmem::from_hex("1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736");
    let candidates = cracker::crack_single_byte_xor_encrypted_ascii_text(&input_data.data, 5);
     for (candidate, _) in candidates {
         println!("{}", &candidate);
     }
}
