use std::{
    collections::HashMap,
    io::{stdin, BufRead},
};

fn find_perms(
    lines: &Vec<Vec<u8>>,
    row: usize,
    col: usize,
    cache: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if let Some(perms) = cache.get(&(row, col)) {
        return *perms;
    }

    let perms = if row == lines.len() {
        1
    } else if lines[row][col] == b'^' {
        (if col > 0 {
            find_perms(lines, row + 1, col - 1, cache)
        } else {
            0
        }) + (if col < lines[0].len() - 1 {
            find_perms(lines, row + 1, col + 1, cache)
        } else {
            0
        })
    } else {
        find_perms(lines, row + 1, col, cache)
    };
    cache.insert((row, col), perms);
    perms
}

fn main() {
    let lines: Vec<_> = stdin()
        .lock()
        .lines()
        .map(|l| l.unwrap().into_bytes())
        .collect();
    let start = lines[0].iter().position(|&b| b == b'S').unwrap();

    let total_perms = find_perms(&lines, 0, start, &mut HashMap::new());

    println!("Total permutations = {total_perms}");
}
