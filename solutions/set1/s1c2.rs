use rawmem::Rawmem;

fn main() {
    let input = "1c0111001f010100061a024b53535009181c";
    let xor_mask = Rawmem::from_hex("686974207468652062756c6c277320657965");
    let output = Rawmem::from_hex(&input).isolen_xor(&xor_mask);
    let target = Rawmem::from_hex("746865206b696420646f6e277420706c6179");

    print!("Set 1 - Challenge 2: ");
    if output == target {
        println!("\x1b[1;32mCorrect!\x1b[0m");
    } else {
        println!("\x1b[1;31mIncorrect!\x1b[0m");
    }
}
