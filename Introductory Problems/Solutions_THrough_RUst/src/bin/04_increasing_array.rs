fn main() {
    let mut inp: Vec<u64> = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .lines()
        .skip(1)
        .flat_map(|s| s.split_whitespace())
        .map(|n| n.parse::<u64>().unwrap())
        .collect();

    let mut res = 0;

    for i in 1..inp.len() {
        if inp[i] < inp[i - 1] {
            res += inp[i - 1] - inp[i];
            inp[i] = inp[i - 1];
        }
    }

    println!("{res}");
}
