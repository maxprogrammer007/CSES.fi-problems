fn main() {
    const MOD: u64 = 1_000_000_007;

    let n: u64 = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let res = (0..n).fold(1, |res, _| res * 2 % MOD);

    println!("{res}");
}
