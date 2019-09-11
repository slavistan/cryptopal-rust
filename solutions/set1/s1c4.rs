use rawmem::Rawmem;
use std::fs;
use std::io::{BufRead, BufReader};

const FILENAME: &str = "solutions/set1/4.txt";

fn main() {
    let filehandle = fs::File::open(FILENAME).unwrap();
    let reader = BufReader::new(filehandle);

    // Try every possible key and filter out all text which may be proper English
    let mut candidates: Vec<String> = Vec::new();
    for line in reader.lines() {
        let raw_data = Rawmem::from_hex(&line.unwrap());

        for key in 0u8..=255u8 {
            let cleartext: String = raw_data.single_byte_xor(key).as_ascii();
            if might_be_english(&cleartext) {
                candidates.push(cleartext);
            }
        }
    }

    // Now sort that shit and spit out the top results
    candidates.sort_by(|a, b| less_special_chars(&a, &b));
    for ii in 0..std::cmp::min(5, candidates.len()) {
        println!("{}", &candidates[ii]);
    }
}


fn might_be_english(s: &String) -> bool {
    s.chars().all(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
        && s.chars().any(|c| c == ' ')
}


fn less_special_chars(a: &String, b: &String) -> std::cmp::Ordering {

    let num_a = num_of_special_chars(&a);
    let num_b = num_of_special_chars(&b);

    num_a.cmp(&num_b)
}


fn num_of_special_chars(a: &String) -> u32 {
    let mut result: u32 = 0;
    a.chars().for_each(|c| { if c.is_ascii_punctuation() { result += 1 }} );
    result
}
