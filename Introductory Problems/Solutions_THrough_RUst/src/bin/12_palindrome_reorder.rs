use std::collections::HashMap;

fn main() {
    let inp = std::io::read_to_string(std::io::stdin()).unwrap();

    let map = inp.trim().chars().fold(HashMap::new(), |mut map, c| {
        map.entry(c.to_string())
            .and_modify(|n| *n += 1)
            .or_insert(1);
        map
    });

    if map.values().filter(|&e| *e % 2 != 0).count() > 1 {
        println!("NO SOLUTION");
        return;
    }

    let (mut even, mut odd) = (String::new(), String::new());

    for (ch, cnt) in map {
        if cnt % 2 != 0 { odd.push_str(&ch.repeat(cnt)); }
        else { even.push_str(&ch.repeat(cnt / 2)); }
    }
    println!("{}{}{}", even, odd, even.chars().rev().collect::<String>());
}
