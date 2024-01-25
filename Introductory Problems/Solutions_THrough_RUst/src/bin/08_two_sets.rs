fn main() {
    let n: u64 = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse()
        .unwrap();

    let (mut a, mut b) = (Vec::new(), Vec::new());
    let mut sum = n * (n + 1) / 2;

    if sum % 2 != 0 { println!("NO"); return; }
    else { println!("YES"); sum /= 2; }

    for i in (1..=n).rev() {
        if i <= sum { sum -= i; a.push(i); }
        else { b.push(i) }
    }

    println!("{}", a.len());
    a.iter().for_each(|e| print!("{e} "));
    println!("\n{}", b.len());
    b.iter().for_each(|e| print!("{e} "));
    println!();
}
