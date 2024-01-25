fn main() {
    let inp: usize = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    (0..(0b1 << inp)).for_each(|b| {
        println!("{:<01$b}", (b >> 0b1) ^ b, inp);
    });
}
