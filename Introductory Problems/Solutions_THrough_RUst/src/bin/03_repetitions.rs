fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();

    let mut dna = inp.chars().peekable();
    let mut cnt = 1;
    let mut max = 1;

    while let (Some(c), Some(&n)) = (dna.next(), dna.peek()) {
        if c == n {
            cnt += 1;
        } else {
            max = std::cmp::max(max, cnt);
            cnt = 1;
        }
    }
    println!("{max}");
}
