fn main() {
    let inp: Vec<usize> = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .lines()
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();

    for mut k in inp {
        let (mut block, mut dig) = (1, 1);

        while k > 9 * block * dig {
            k -= 9 * block * dig;

            block *= 10;
            dig += 1;
        }

        let (quot, rem) = ((k - 1) / dig, (k - 1) % dig);

        if let Some(d) = (block + quot).to_string().chars().nth(rem) {
            println!("{d}");
        }
    }
}
