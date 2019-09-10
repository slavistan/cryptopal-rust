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


    pub fn from_vec(bytes: &[u8]) -> Rawmem {
        Rawmem { data: bytes.to_vec() }
    }


    pub fn as_hex(&self) -> String {
        let mut result = String::with_capacity(&self.data.len() * 2);
        for &byte in &self.data {
            result.push_str(&format!("{:02X}", byte));
        }
        result
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
