mod rawmem;
use rawmem::Rawmem;

fn main() {
    let data = Rawmem::from_hex("AABBCCDD");
    println!("{:?}", data);

    println!("base64: {}", data.as_base64());
}


