fn main() {
    let inp = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .parse::<u64>()
        .unwrap();

    if inp == 2 || inp == 3 {
        println!("NO SOLUTION");
    } else {
        (2..=inp)
            .step_by(2)
            .chain((1..=inp).step_by(2))
            .for_each(|e| print!("{e} "));
        println!("");
    }
}
