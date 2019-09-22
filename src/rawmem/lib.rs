use std::u8;
use dbc::*;

#[derive(Debug)]
pub struct Rawmem {
    pub data: Vec<u8>
}


impl PartialEq for Rawmem {
    fn eq(&self, other: &Rawmem) -> bool {
        self.data == other.data
    }
}


impl Rawmem {
    pub fn from_hex(hex_string: &str) -> Rawmem {

        // TODO: Assert valid hex-characters & even character count
        require!(hex_string.len() % 2 == 0);

        let num_of_bytes = hex_string.len() / 2;
        let mut result = Rawmem { data: Vec::with_capacity(num_of_bytes) };
        for ii in 0..num_of_bytes {
            let hex_duo = &hex_string[ii*2 .. ii*2+2];
            let byte: u8 = u8::from_str_radix(hex_duo, 16)
                .expect("Can't convert hex-chars");
            result.data.push(byte);
        }
        result
    }


    pub fn from_base64(base64_string: &str) -> Rawmem {
        let len = base64_string.len();
        require!(len % 4 == 0);
        let capacity = (len * 4) / 3;

        let mut result = Rawmem { data: Vec::with_capacity(capacity) };

        for ii in (0..(len-4)).step_by(4) {
            let quadruplet = &base64_string[ii..ii+4].as_bytes();
            let byte_triplet = Rawmem::base64_quadruplet_as_byte_triplet(&quadruplet);
            result.data.push(byte_triplet[0]);
            result.data.push(byte_triplet[1]);
            result.data.push(byte_triplet[2]);
        }

        let quadruplet = &base64_string[len-4..len];
        let mut final_bytes = Rawmem::final_base64_quadruplet_as_bytes(&quadruplet.as_bytes());
        result.data.append(&mut final_bytes);
        result
    }


    pub fn from_vec(bytes: &[u8]) -> Rawmem {
        Rawmem { data: bytes.to_vec() }
    }


    pub fn from_ascii(ascii_string: &str) -> Rawmem {
        require!(ascii_string.chars().all(|c| c.is_ascii()));

        Rawmem { data: ascii_string.as_bytes().to_vec() }
    }


    pub fn as_hex(&self) -> String {
        let mut result = String::with_capacity(&self.data.len() * 2);
        for &byte in &self.data {
            result.push_str(&format!("{:02X}", byte));
        }
        result
    }


    // TODO: Replace non-printable chars with something printable
    pub fn as_ascii(&self) -> String {
        hamming::as_ascii(&self.data)
    }


    pub fn as_base64(&self) -> String {
        let max_capacity = 2f64 + (self.data.len() as f64 / 3 as f64).ceil() * 4f64;
        let mut result = String::with_capacity(max_capacity as usize);
        let meat_size_in_bytes = (self.data.len() / 3) * 3;
        const LSB6MASK: u64 = 0x0000003F; // masks the 6 least-significant bits
        const BASE64_CHARS: [char; 64] =
          ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P',
           'Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f',
           'g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v',
           'w','x','y','z','0','1','2','3','4','5','6','7','8','9','+','/'];
        // process chunks of 24 bits
        for ii in (0..meat_size_in_bytes).step_by(3) {
            // read three adjaceny bytes into a u64
            let chunk: u64 = ((self.data[ii+0] as u64) << 16) |
                             ((self.data[ii+1] as u64) << 8)  |
                             ((self.data[ii+2] as u64) << 0);
            // shift out sextetts and access base64 char
            for offset in [18, 12, 6, 0].iter() {
                let base64_char_index: usize = ((chunk >> offset) & LSB6MASK) as usize;
                result.push(BASE64_CHARS[base64_char_index]);
            }
        }

        // process residue (0, 1 or 2 bytes)
        let remainder_size_in_bytes = self.data.len() - meat_size_in_bytes;
        let next_byte_index = self.data.len() - remainder_size_in_bytes;
        match remainder_size_in_bytes {
            1 => {
                // 1 more byte: 4 pad bits -> 2 more sextetts
                let num_of_pad_bits = 4;
                let chunk: u64 = (self.data[next_byte_index] as u64) << num_of_pad_bits;
                for offset in [6, 0].iter() {
                    let base64_char_index: usize = ((chunk >> offset) & LSB6MASK) as usize;
                    result.push(BASE64_CHARS[base64_char_index]);
                }
                result.push_str("==");
            }
            2 => {
                // 2 more bytes: 2 pad bits -> 3 more sextetts
                let num_of_pad_bits = 2;
                let chunk: u64 = (((self.data[next_byte_index] as u64) << 8)  |
                                  ((self.data[next_byte_index] as u64) << 0)) << num_of_pad_bits;
                for offset in [12, 6, 0].iter() {
                    let base64_char_index: usize = ((chunk >> offset) & LSB6MASK) as usize;
                    result.push(BASE64_CHARS[base64_char_index]);
                }
                result.push_str("=");
            }
            _ => {}
        }
        result
    }


    // xor the data from two Rawmems returning a fresh Rawmem
    pub fn isolen_xor(&self, other: &Rawmem) -> Rawmem {
        let bytes: Vec<u8> = hamming::isolen_xor(&self.data, &other.data);
        Rawmem { data: bytes }
    }


    // xor the data against a single key one, byte by byte
    pub fn single_byte_xor(&self, mask: u8) -> Rawmem {
        let bytes: Vec<u8> = hamming::single_byte_xor(&self.data, mask);
        Rawmem { data: bytes }
    }

    // TODO: Rename to vigenere_xor
    pub fn repeating_key_xor(&self, key: &Rawmem) -> Rawmem {
        let mut result = Rawmem { data: Vec::with_capacity(self.data.len()) };

        for ii in 0..self.data.len() {
            let key_index = ii % key.data.len();
            result.data.push(self.data[ii] ^ key.data[key_index]);
        }
        result
    }

    pub fn hamming_weight(&self) -> u64 {
        hamming::hamming_weight(&self.data)
    }

    pub fn chunked_hamming_density(&self, width: u64) -> (f64, u64) {
        hamming::chunked_hamming_density(&self.data, width)
    }

    // returns bit-wise hamming distance (number of differing bits)
    pub fn hamming_distance(&self, other: &Rawmem) -> u64 {
        hamming::hamming_distance(&self.data, &other.data)
    }


    fn base64_char_as_u8(c: char) -> Option<u8> {

        if c.is_ascii_uppercase() {
            let distance_to_A: u8 = (c as u8) - b'A';
            Some(distance_to_A)
        } else if c.is_ascii_lowercase() {
            let distance_to_a: u8 = (c as u8) - b'a';
            Some(26u8 + distance_to_a)
        } else if c.is_ascii_digit() {
            let distance_to_0: u8 = (c as u8) - b'0';
            Some(52u8 + distance_to_0)
        } else if c == '+' {
            Some(62u8)
        } else if c == '/' {
            Some(63u8)
        } else if c == '=' {
            None
        } else {
            panic!()
        }
    }

    // Parses the non-final part of a base64 string as chunks of 4 6-bit quadruplets
    // TODO: Base64 quadruplets are characters, not necessarily utf8 encoded. Use char
    //       type as input.
    fn base64_quadruplet_as_byte_triplet(quadruplet: &[u8]) -> [u8; 3] {
    //    require!(quadruplet.len() == 4);
    //    require!(quadruplet[0] < 64);
    //    require!(quadruplet[1] < 64);
    //    require!(quadruplet[2] < 64);
    //    require!(quadruplet[3] < 64);

        let q1: u64 = Rawmem::base64_char_as_u8(quadruplet[0] as char).unwrap() as u64;
        let q2: u64 = Rawmem::base64_char_as_u8(quadruplet[1] as char).unwrap() as u64;
        let q3: u64 = Rawmem::base64_char_as_u8(quadruplet[2] as char).unwrap() as u64;
        let q4: u64 = Rawmem::base64_char_as_u8(quadruplet[3] as char).unwrap() as u64;

        let buffer: u64 = ((q1 << 18) | (q2 << 12) | (q3 << 6) | q4) as u64;

        [(buffer >> 16) as u8,
         ((buffer >> 8) & 0x000000FF) as u8,
         (buffer & 0x000000FF) as u8]
    }

    // Parse last base64 quadruplet. Takes care of padding.
    fn final_base64_quadruplet_as_bytes(quadruplet: &[u8]) -> Vec<u8> {
    //    require!(quadruplet.len() == 4);
    //    require!(quadruplet[0] < 64);
    //    require!(quadruplet[1] < 64);
    //    require!(quadruplet[2] < 64 || (quadruplet[2] == b'=' && quadruplet[3] == b'='));
    //    require!(quadruplet[3] < 64 || quadruplet[3] == b'=');

        if quadruplet[3] != b'=' { // 'XXXX': Last group had 3 bytes
            Rawmem::base64_quadruplet_as_byte_triplet(&quadruplet).to_vec()
        } else if quadruplet[2] == b'=' { // 'XX==': Last group had 1 byte

            let q1: u64 = Rawmem::base64_char_as_u8(quadruplet[0] as char).unwrap() as u64;
            let q2: u64 = Rawmem::base64_char_as_u8(quadruplet[1] as char).unwrap() as u64;

            let buffer: u64 = ((q1 << 6) | q2) >> 4;
            let result: Vec<u8> = vec![(buffer & 0x000000FF) as u8];
            result
        } else if quadruplet[3] == b'=' { // 'XXX=': Last group had 2 bytes

            let q1: u64 = Rawmem::base64_char_as_u8(quadruplet[0] as char).unwrap() as u64;
            let q2: u64 = Rawmem::base64_char_as_u8(quadruplet[1] as char).unwrap() as u64;
            let q3: u64 = Rawmem::base64_char_as_u8(quadruplet[2] as char).unwrap() as u64;
            let buffer: u64 = ((q1 << 12) | (q2 << 6) | q3 ) >> 2;
            let result: Vec<u8> = vec![((buffer >> 8) & 0x000000FF) as u8,
                (buffer & 0x000000FF) as u8];
            result
        } else {
            panic!();
        }

    }
}

// high-level wrappers
pub mod cracker {

    use std::u8;
    use dbc::*;
    use super::hamming;

    pub fn crack_single_byte_xor_encrypted_ascii_text(bytes: &[u8], num_of_suggestions: i32)
    -> Vec<(String, u8)> {
        require!(num_of_suggestions > 0);

        // Decrypt and apply a very basic filter
        let mut result: Vec<(String, u8)> = Vec::with_capacity(256);
        for key in 0u8..=255u8 {
            let decrypted: Vec<u8> = hamming::single_byte_xor(&bytes, key);
            if decrypted.iter().all(|c| c.is_ascii_graphic() || c.is_ascii_whitespace()) 
                && decrypted.iter().any(|c| c.is_ascii_whitespace()) {
                result.push((String::from_utf8(decrypted).expect("Invalid utf8 character"), key));
            }
        }

        // sort according to number of special characters
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

        result.sort_by(|a, b| less_special_chars(&a.0, &b.0));
        result.truncate(num_of_suggestions as usize);
        result
    }
}


// Bare-metal byte manipulations
pub mod hamming {

    use dbc::*;
    use std::slice;

    const NUM_OF_ACTIVE_BITS_IN_BYTE: [u8; 256] =
       [0,1,1,2,1,2,2,3,1,2,2,3,2,3,3,4,
        1,2,2,3,2,3,3,4,2,3,3,4,3,4,4,5,1,2,2,
        3,2,3,3,4,2,3,3,4,3,4,4,5,2,3,3,4,3,4,
        4,5,3,4,4,5,4,5,5,6,1,2,2,3,2,3,3,4,2,
        3,3,4,3,4,4,5,2,3,3,4,3,4,4,5,3,4,4,5,
        4,5,5,6,2,3,3,4,3,4,4,5,3,4,4,5,4,5,5,
        6,3,4,4,5,4,5,5,6,4,5,5,6,5,6,6,7,1,2,
        2,3,2,3,3,4,2,3,3,4,3,4,4,5,2,3,3,4,3,
        4,4,5,3,4,4,5,4,5,5,6,2,3,3,4,3,4,4,5,
        3,4,4,5,4,5,5,6,3,4,4,5,4,5,5,6,4,5,5,
        6,5,6,6,7,2,3,3,4,3,4,4,5,3,4,4,5,4,5,
        5,6,3,4,4,5,4,5,5,6,4,5,5,6,5,6,6,7,3,
        4,4,5,4,5,5,6,4,5,5,6,5,6,6,7,4,5,5,6,
        5,6,6,7,5,6,6,7,6,7,7,8];

    /// Returns the number of set bits in a slice.
    ///
    /// # Examples
    ///
    /// ```
    /// use rawmem::hamming::hamming_weight;
    /// use std::slice;
    ///
    /// let byte = 3u8;
    /// let hw = hamming_weight(slice::from_ref(&byte));
    ///
    /// assert_eq!(hw, 2);
    /// ```
    pub fn hamming_weight(bytes: &[u8]) -> u64 {
        let mut result: u64 = 0;
        for &byte in bytes.iter() {
            result += NUM_OF_ACTIVE_BITS_IN_BYTE[byte as usize] as u64;
        }
        result
    }

    pub fn hamming_distance(a: &[u8], b: &[u8]) -> u64 {
        require!(a.len() == b.len());

        let mut result: u64 = 0;
        for ii in 0..a.len() {
            let byte = a[ii] ^ b[ii];
            result += hamming_weight(slice::from_ref(&byte));
        }
        result
    }

    pub fn isolen_xor(a: &[u8], b: &[u8]) -> Vec<u8> {
        require!(a.len() == b.len());

        let mut result: Vec<u8> = Vec::with_capacity(a.len());
        for ii in 0..a.len() {
            result.push(a[ii] & b[ii]);
        }
        result
    }

    pub fn single_byte_xor(bytes: &[u8], key: u8) -> Vec<u8> {
        let mut result: Vec<u8> = bytes.to_vec();
        single_byte_xor_inplace(&mut result[..], key);
        result
    }

    pub fn single_byte_xor_inplace(bytes: &mut [u8], key: u8) {
        for ii in 0..bytes.len() {
            bytes[ii] = bytes[ii] ^ key;
        }
    }

    // Repeating-key xor
    pub fn vigenere_xor(bytes: &[u8], key: &[u8]) -> Vec<u8> {
        require!(key.len() != 0);

        let mut result: Vec<u8> = Vec::with_capacity(bytes.len());
        for ii in 0..bytes.len() {
            let key_index = ii % key.len();
            result.push(bytes[ii] ^ key[key_index]);
        }
        result
    }

    // Returns normalized hamming-distance between neighboring chunks of
    // given width, tupled with the number of discarded bytes at the end of
    // input slice.
    pub fn chunked_hamming_density(bytes: &[u8], width: u64) -> (f64, u64) {
        let num_of_iterations = (bytes.len() as u64 / width) - 1;

        require!(width > 0);
        require!(num_of_iterations > 0);

        let mut result: f64 = 0.0;
        for ii in 0..num_of_iterations {
            let range_a = (ii*width) as usize..((ii+1)*width) as usize;
            let range_b = ((ii+1)*width) as usize..((ii+2)*width) as usize;
            result += hamming_distance(&bytes[range_a], &bytes[range_b]) as f64;
        }
        result /= (num_of_iterations * width) as f64;
        (result, bytes.len() as u64 - ((num_of_iterations+1) as u64 * width))
    }

    pub fn as_ascii(bytes: &[u8]) -> String {
        String::from_utf8(bytes.to_vec()).expect("Invalid UTF8 found")
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true() {
        Rawmem::from_hex("ABCDEF");
        assert_eq!(1, 1);
    }
}
