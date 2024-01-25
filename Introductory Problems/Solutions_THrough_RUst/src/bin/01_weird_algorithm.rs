fn main() {
    let mut inp: u64 = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    print!("{inp}");
    while inp > 1 {
        if inp % 2 == 0 { inp /= 2; }
        else { inp = inp * 3 + 1; }
        print!(" {inp}");
    }
    println!();
}
