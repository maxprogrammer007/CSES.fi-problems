use std::collections::BTreeSet;

fn perm(left: usize, chars: &mut Vec<char>, res: &mut BTreeSet<String>) {
    if left == chars.len() {
        res.insert(chars.iter().collect());
        return;
    }

    (left..chars.len()).for_each(|right| {
        chars.swap(left, right);
        perm(left + 1, chars, res);
        chars.swap(left, right);
    });
}

fn main() {
    let mut inp: Vec<char> = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .trim()
        .chars()
        .collect();

    let mut res = BTreeSet::new();

    perm(0, &mut inp, &mut res);

    println!("{}", res.len());
    res.iter().for_each(|s| println!("{s}"));
}
