fn main() {
    let n: u8 = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    println!("{}", (0b1 << n) - 1);
    hanoi(n, 1, 2, 3);
}

fn hanoi(n: u8, src: u8, tmp: u8, tgt: u8) {
    if n == 0 { return; }

    hanoi(n - 1, src, tgt, tmp);
    println!("{} {}", src, tgt);
    hanoi(n - 1, tmp, src, tgt);
}
