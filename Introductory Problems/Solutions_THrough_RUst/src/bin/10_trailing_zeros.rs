fn main() {
    let mut n: u64 = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let mut res = 0;

    while n > 0 { res += n / 5; n /= 5; }

    println!("{res}");
}
