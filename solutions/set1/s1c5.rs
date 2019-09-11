use rawmem::Rawmem;

fn main() {

    let input = Rawmem::from_ascii("Burning 'em, if you ain't quick and nimble\n\
                 I go crazy when I hear a cymbal");
    let target = Rawmem::from_hex("0b3637272a2b2e63622c2e69692a23\
                    693a2a3c6324202d623d63343c2a26226324272765272\
                    a282b2f20430a652e2c652a3124333a653e2b2027630c\
                    692b20283165286326302e27282f");
    let key = Rawmem::from_ascii("ICE");

    let output = input.repeating_key_xor(&key);

    print!("Set 1 - Challenge 5: ");
    if output == target {
        println!("\x1b[1;32mCorrect!\x1b[0m");
    } else {
        println!("\x1b[1;31mIncorrect!\x1b[0m");
    }
}
