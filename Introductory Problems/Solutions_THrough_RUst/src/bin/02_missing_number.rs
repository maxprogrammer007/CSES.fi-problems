fn main() {
    let mut buf = Vec::new();

    for l in std::io::stdin().lines().take(2) {
        buf.push(l.unwrap());
    }

    let n: u64 = buf[0].parse().unwrap();

    let sum: u64 = buf[1]
        .split_whitespace()
        .map(|n| n.parse::<u64>().unwrap())
        .sum();

    println!("{}", n * (n + 1) / 2 - sum);
}

