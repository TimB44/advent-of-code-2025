use std::{
    collections::{HashMap, VecDeque},
    io::{stdin, BufRead},
    iter::repeat,
};

const SURROUNDED_THRESHOLD: usize = 4;

fn main() {
    let grid: Vec<Vec<bool>> = stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .map(|line| line.bytes().map(|b| b == b'@').collect())
        .collect();
    let rows = grid.len();
    let cols = grid[0].len();
    let mut graph: HashMap<_, Vec<_>> = HashMap::new();

    let fetch_neighbors = |row_i: usize, col_i: usize| {
        let start_row_i = row_i.saturating_sub(1);
        let start_col_i = col_i.saturating_sub(1);

        let end_row_i = (row_i + 2).min(rows);
        let end_col_i = (col_i + 2).min(cols);

        (start_row_i..end_row_i)
            .flat_map(|cur_row_i| repeat(cur_row_i).zip(start_col_i..end_col_i))
            .filter(|(r, c)| !(*r == row_i && *c == col_i) && grid[*r][*c])
            .collect()
    };

    for (row_i, col_i) in grid
        .iter()
        .enumerate()
        .flat_map(|(row_i, row)| row.iter().enumerate().map(move |(col_i, _)| (row_i, col_i)))
        .filter(|&(row_i, col_i)| grid[row_i][col_i])
    {
        graph.insert((row_i, col_i), fetch_neighbors(row_i, col_i));
    }

    let mut in_counts: HashMap<_, _> = graph
        .iter()
        .map(|(cords, edges)| (cords, edges.len()))
        .collect();

    let mut queue: VecDeque<_> = in_counts
        .iter()
        .filter(|&(_, &count)| count < SURROUNDED_THRESHOLD)
        .map(|t| *t.0)
        .copied()
        .collect();

    let mut removed = 0;
    while let Some(cords) = queue.pop_front() {
        removed += 1;
        for v in graph.get(&cords).unwrap() {
            if in_counts[v] == SURROUNDED_THRESHOLD {
                queue.push_back(*v);
            }
            *in_counts.get_mut(v).unwrap() -= 1;
        }
    }

    println!("Removed count = {removed}");
}
