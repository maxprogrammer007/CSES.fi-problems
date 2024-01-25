fn main() {
    let inp: Vec<_> = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .lines()
        .skip(1)
        .flat_map(|s| s.split_once(' '))
        .map(|(a, b)| (a.parse::<u64>().unwrap(), b.parse::<u64>().unwrap()))
        .collect();

    for (a, b) in inp {
        if (a + b) % 3 == 0 && 2 * a.min(b) >= a.max(b) {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
