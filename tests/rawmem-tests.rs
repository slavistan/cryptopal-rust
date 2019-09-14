
#[cfg(test)]
mod tests {
    use rawmem::Rawmem;

    #[test]
    fn construct() {
        assert_eq!(Rawmem::from_hex("FF"), Rawmem::from_vec(&[255u8]));
        assert_eq!(Rawmem::from_hex("00"), Rawmem::from_vec(&[0]));
        assert_eq!(Rawmem::from_hex("0F"), Rawmem::from_vec(&[15]));
        assert_eq!(Rawmem::from_hex("10FF"), Rawmem::from_vec(&[16, 255]));
        assert_eq!(Rawmem::from_hex("00FF"), Rawmem::from_vec(&[0, 255]));
        assert_eq!(Rawmem::from_hex("01FF"), Rawmem::from_vec(&[1, 255]));
        assert_eq!(Rawmem::from_hex("DEADBEEF"),
            Rawmem::from_vec(&[222, 173, 190, 239]));
        assert_eq!(Rawmem::from_hex("FF"), Rawmem::from_hex("ff"));

        assert_eq!(Rawmem::from_hex("00").as_hex(), "00");
        assert_eq!(Rawmem::from_hex("FF").as_hex(), "FF");
        assert_eq!(Rawmem::from_hex("AFFE").as_hex(), "AFFE");
        assert_eq!(Rawmem::from_hex("DEADBEEF").as_hex(), "DEADBEEF");

        assert_eq!(Rawmem::from_hex("FF010F").as_base64(), "/wEP");
        assert_eq!(Rawmem::from_hex("01").as_base64(), "AQ==");
        assert_eq!(Rawmem::from_hex("FF010F").as_base64(), "/wEP");
        assert_eq!(Rawmem::from_hex("FF010F01").as_base64(), "/wEPAQ==");

        assert_eq!(Rawmem::from_hex("FF").single_byte_xor(0xFFu8), Rawmem::from_hex("00"));
        assert_eq!(Rawmem::from_hex("0F").single_byte_xor(0xF0u8), Rawmem::from_hex("FF"));

        assert_eq!(Rawmem::from_ascii("Aa"), Rawmem::from_vec(&[65, 97]));
        assert_eq!(Rawmem::from_ascii("Aa").as_ascii(), String::from("Aa"));

        assert_eq!(Rawmem::from_base64("AAAA"), Rawmem::from_hex("000000"));
        assert_eq!(Rawmem::from_base64("////"), Rawmem::from_hex("FFFFFF"));
        assert_eq!(Rawmem::from_base64("////AAAA"), Rawmem::from_hex("FFFFFF000000"));
        assert_eq!(Rawmem::from_base64("QQ=="), Rawmem::from_ascii("A"));
        assert_eq!(Rawmem::from_base64("QQ=="), Rawmem::from_ascii("A"));
        assert_eq!(Rawmem::from_base64("QUI="), Rawmem::from_ascii("AB"));
        assert_eq!(Rawmem::from_ascii("any carnal pleasure."), Rawmem::from_base64("YW55IGNhcm5hbCBwbGVhc3VyZS4="));
    }

    #[test]
    fn hamming() {
        let data1 = Rawmem::from_ascii("this is a test");
        let data2 = Rawmem::from_ascii("wokka wokka!!!");
        assert_eq!(data1.hamming_distance(&data2), 37);
    }

}
