fn main() {
    let n = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse::<i64>()
        .unwrap();

    (1..=n).for_each(|k| {
        println!(
            "{}",
            (k.pow(2) * (k.pow(2) - 1) / 2) - (4 * (k - 2) * (k - 1))
        );
    });
}
