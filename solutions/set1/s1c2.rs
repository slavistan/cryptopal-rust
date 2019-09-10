use rawmem::Rawmem;

fn main() {
    let input = "1c0111001f010100061a024b53535009181c";
    let xor_mask = Rawmem::from_hex("686974207468652062756c6c277320657965");
    let output = Rawmem::from_hex(&input).isolen_xor(&xor_mask);

    let target = Rawmem::from_hex("746865206b696420646f6e277420706c6179");
    assert_eq!(target, output);
}
