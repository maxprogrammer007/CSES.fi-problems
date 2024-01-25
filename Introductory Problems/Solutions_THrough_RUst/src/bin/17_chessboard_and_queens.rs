use std::collections::HashSet as HS;

fn main() {
    let board: Vec<Vec<char>> = std::io::read_to_string(std::io::stdin())
        .unwrap()
        .lines()
        .map(|s| s.chars().collect())
        .collect();

    let (mut col, mut pos_diag, mut neg_diag) = (HS::new(), HS::new(), HS::new());

    println!(
        "{}",
        queens(&board, 0, &mut col, &mut pos_diag, &mut neg_diag)
    );
}

fn queens(
    board: &Vec<Vec<char>>,
    r: i8,
    col: &mut HS<i8>,
    pos_diag: &mut HS<i8>,
    neg_diag: &mut HS<i8>,
) -> i8 {
    let mut res = 0;

    if r == 8 { return 1; }

    for c in 0..8 {
        if board[r as usize][c as usize] == '*'
            || col.contains(&c)
            || pos_diag.contains(&(r + c))
            || neg_diag.contains(&(r - c))
        {
            continue;
        }

        col.insert(c); pos_diag.insert(r + c); neg_diag.insert(r - c);
        res += queens(board, r + 1, col, pos_diag, neg_diag);
        col.remove(&c); pos_diag.remove(&(r + c)); neg_diag.remove(&(r - c));
    }
    res
}
