fn main() {
    let a = "Hallo!";
    let mut b = a.clone();

    unsafe {
        println!("{:?}", b.get_unchecked(0..100));
    }
}


fn foo(s: &str) -> &str {

    if s.len() > 1 {
        &s[1..2]
    } else {
        &s[0..1]
    }
}
