use rawmem::Rawmem;
use rawmem::cracker;
use rawmem::hamming;
use std::fs;

const FILENAME: &str = "solutions/set1/6.txt";

fn main() {

    // De-base64
    let content = fs::read_to_string(&FILENAME).expect("Error reading file.");
    let content2 = content.replace("\n", "");
    let data = Rawmem::from_base64(&content2).data;

    // Guess key sizes and sort according to chunked hamming density
    let max_width = 48;
    let mut width_vs_density: Vec<(u64, f64)> = Vec::with_capacity(max_width);
    for width in 1..=max_width {
        let (dens, _) = hamming::chunked_hamming_density(&data, width as u64);
        width_vs_density.push((width as u64, dens));
    }
    width_vs_density.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

    for kk in 0..5 {
        let key_size = width_vs_density[kk].0;

        // Concatenate all bytes pertaining to the same key and decode the sequences
        let mut key: String = String::with_capacity(key_size as usize);
        let mut stridden_cleartexts: Vec<String> = Vec::with_capacity(key_size as usize);
        for indeks in 0..key_size {
            let mut start_index = indeks;
            let mut sequence: Vec<u8> = Vec::with_capacity((data.len() as usize / key_size as usize) + 1);
            while start_index < data.len() as u64 {
                sequence.push(data[start_index as usize]);
                start_index += key_size;
            }
            let stridden_cleartext_candidates = cracker::crack_single_byte_xor_encrypted_ascii_text(&sequence, 5);
            let stridden_cleartext = match stridden_cleartext_candidates.first() {
                Some(first) => {
                    key.push(first.1.clone() as char); // Save the key's chars
                    first.0.clone()
                },
                None => { String::from("") }
            };
            stridden_cleartexts.push(stridden_cleartext.clone());
        }

        // Interleave stridden cleartexts
        let mut result = String::with_capacity(data.len());
        for ii in 0..data.len() {
            let which_sequence = ii % key_size as usize;
            let which_index = ii / key_size as usize;
            match stridden_cleartexts.get(which_sequence) {
                Some(s) => {
                    match s.as_bytes().get(which_index) {
                        Some(c)  => { result.push(c.clone() as char) },
                        None => {}
                    };
                }
                None => { }
            }
        }
        println!("\x1b[1;32mKey: \x1b[0;32m{}\x1b[0m", key);
        println!("\x1b[33;1mCleartext:\x1b[0m \x1b[33m{}\x1b[0m", result);
    }
}
