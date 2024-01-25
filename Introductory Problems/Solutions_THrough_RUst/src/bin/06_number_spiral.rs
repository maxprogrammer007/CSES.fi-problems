fn main() {
    let inp: Vec<(u64, u64)> = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .lines()
        .skip(1)
        .flat_map(|s| s.split_once(' '))
        .map(|(r, c)| (r.parse().unwrap(), c.parse().unwrap()))
        .collect();

    let mut res: u64;

    for (r, c) in inp {
        if r > c {
            if r % 2 == 0 { res = r.pow(2) - c + 1; }
            else { res = (r - 1).pow(2) + c; }
        } else {
            if c % 2 == 0 { res = (c - 1).pow(2) + r; }
            else { res = c.pow(2) - r + 1;
            }
        }
        println!("{res}");
    }
}

