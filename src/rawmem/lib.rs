use std::u8;
use dbc::*;

#[derive(Debug)]
pub struct Rawmem {
    data: Vec<u8>
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

        let capacity = (base64_string.len() * 4) / 3;
        let mut result = Rawmem { data: Vec::with_capacity(capacity) };

        let meat_size = base64_string.len() / 4;
        for ii in 0..meat_size {
            let quadruplet = &base64_string[ii..ii+4].as_bytes();
            println!("QUADRUPLET={:?}", quadruplet);
            let byte_triplet = Rawmem::byte_triplet_from_base64_quadruplet(&quadruplet);
            result.data.push(byte_triplet[0]);
            result.data.push(byte_triplet[1]);
            result.data.push(byte_triplet[2]);
        }
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


    pub fn as_ascii(&self) -> String {
        String::from_utf8(self.data.clone()).expect("Invalid UTF8 found")
        // let mut result = String::with_capacity(self.data.len());
        // for ii in 0..self.data.len() {
        //     result.push(self.data[ii] as char);
        // }
        // result
    }


    pub fn as_base64(&self) -> String {
        let max_capacity = 2f64 + (self.data.len() as f64 / 3 as f64).ceil() * 4f64;
        let mut result = String::with_capacity(max_capacity as usize);
        let meat_size_in_bytes = (self.data.len() / 3) * 3;
        const LSB6MASK: u32 = 0x0000003F; // masks the 6 least-significant bits
        const BASE64_CHARS: [char; 64] =
          ['A','B','C','D','E','F','G','H','I','J','K','L','M','N','O','P',
           'Q','R','S','T','U','V','W','X','Y','Z','a','b','c','d','e','f',
           'g','h','i','j','k','l','m','n','o','p','q','r','s','t','u','v',
           'w','x','y','z','0','1','2','3','4','5','6','7','8','9','+','/'];
        // process chunks of 24 bits
        for ii in (0..meat_size_in_bytes).step_by(3) {
            // read three adjaceny bytes into a u32
            let chunk: u32 = ((self.data[ii+0] as u32) << 16) |
                             ((self.data[ii+1] as u32) << 8)  |
                             ((self.data[ii+2] as u32) << 0);
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
                let chunk: u32 = (self.data[next_byte_index] as u32) << num_of_pad_bits;
                for offset in [6, 0].iter() {
                    let base64_char_index: usize = ((chunk >> offset) & LSB6MASK) as usize;
                    result.push(BASE64_CHARS[base64_char_index]);
                }
                result.push_str("==");
            }
            2 => {
                // 2 more bytes: 2 pad bits -> 3 more sextetts
                let num_of_pad_bits = 2;
                let chunk: u32 = (((self.data[next_byte_index] as u32) << 8)  |
                                  ((self.data[next_byte_index] as u32) << 0)) << num_of_pad_bits;
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
        require!(self.data.len() == other.data.len());

        let mut result = Rawmem { data: Vec::with_capacity(self.data.len()) };
        for ii in 0..self.data.len() {
            result.data.push(self.data[ii] ^ other.data[ii]);
        }
        result
    }


    // xor the data against a single key one, byte by byte
    pub fn single_byte_xor(&self, mask: u8) -> Rawmem {
        let mut result = Rawmem { data: Vec::with_capacity(self.data.len()) };
        for ii in 0..self.data.len() {
            result.data.push(self.data[ii] ^ mask);
        }
        result
    }


    pub fn repeating_key_xor(&self, key: &Rawmem) -> Rawmem {
        let mut result = Rawmem { data: Vec::with_capacity(self.data.len()) };

        for ii in 0..self.data.len() {
            let key_index = ii % key.data.len();
            result.data.push(self.data[ii] ^ key.data[key_index]);
        }
        result
    }

    pub fn num_of_set_bits(&self) -> u32 {
        let mut result: u32 = 0;
        for &byte in self.data.iter() {
            result += ACTIVE_BITS[byte as usize] as u32;
        }
        result
    }

    // returns bit-wise Hamming distance (number of differing bits)
    pub fn hamming_distance(&self, other: &Rawmem) -> u32 {
        self.isolen_xor(&other).num_of_set_bits()
    }


    pub fn hamming_weight(byte: u8) -> u32 {
        ACTIVE_BITS[byte as usize] as u32
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

    fn byte_triplet_from_base64_quadruplet(quadruplet: &[u8]) -> [u8; 3] {
        require!(quadruplet.len() == 4);

        let q1: u32 = Rawmem::base64_char_as_u8(quadruplet[0] as char).unwrap() as u32;
        let q2: u32 = Rawmem::base64_char_as_u8(quadruplet[1] as char).unwrap() as u32;
        let q3: u32 = Rawmem::base64_char_as_u8(quadruplet[2] as char).unwrap() as u32;
        let q4: u32 = Rawmem::base64_char_as_u8(quadruplet[3] as char).unwrap() as u32;

        let buffer: u32 = ((q1 << 18) | (q2 << 12) | (q3 << 6) | q4) as u32;
        println!("BUFFER = {:?}", buffer);

        [(buffer >> 16) as u8,
         ((buffer >> 8) & 0x000000FF) as u8,
         (buffer & 0x000000FF) as u8]
    }
}

const ACTIVE_BITS: [u8; 256] = [0,1,1,2,1,2,2,3,1,2,2,3,2,3,3,4,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_true() {
        Rawmem::from_hex("ABCDEF");
        assert_eq!(1, 1);
    }
}
