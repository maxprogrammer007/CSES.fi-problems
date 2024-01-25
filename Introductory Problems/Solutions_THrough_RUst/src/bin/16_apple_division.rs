fn main() {
    let inp: Vec<i64> = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .lines()
        .skip(1)
        .flat_map(|s| s.split_whitespace())
        .map(|n| n.parse().unwrap())
        .collect();

    println!("{}", min_diff(&inp, inp.len(), 0, 0));
}

fn min_diff(v: &Vec<i64>, n: usize, sum1: i64, sum2: i64) -> i64 {
    if n == 0 { return (sum1 - sum2).abs(); }

    let diff1 = min_diff(v, n - 1, sum1 + v[n - 1], sum2);
    let diff2 = min_diff(v, n - 1, sum1, sum2 + v[n - 1]);

    diff1.min(diff2)
}
