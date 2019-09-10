use rawmem::Rawmem;

fn main() {
    let input =
      "49276d206b696c6c696e6720796f7572\
       20627261696e206c696b65206120706f\
       69736f6e6f7573206d757368726f6f6d";
    let target =
      "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBs\
       aWtlIGEgcG9pc29ub3VzIG11c2hyb29t";
    let output = Rawmem::from_hex(&input).as_base64();

    print!("Set 1 - Challenge 1: ");
    if output == target {
        println!("\x1b[1;32mCorrect!\x1b[0m");
    } else {
        println!("\x1b[1;31mIncorrect!\x1b[0m");
    }
}
