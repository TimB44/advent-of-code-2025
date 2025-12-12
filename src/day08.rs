use std::{collections::HashMap, io::stdin};

struct DisjointSet {
    set: Vec<usize>,
}

impl DisjointSet {
    fn new(size: usize) -> Self {
        Self {
            set: (0..size).collect(),
        }
    }

    fn union(&mut self, l: usize, r: usize) {
        let l_rep = self.rep(l);
        let r_rep = self.rep(r);
        self.set[l_rep] = r_rep;
    }

    fn rep(&mut self, n: usize) -> usize {
        let rep = self.set[n];
        if rep == n {
            n
        } else {
            let rep = self.rep(rep);
            self.set[n] = rep;
            rep
        }
    }
}

fn dist([x1, y1, z1]: [f64; 3], [x2, y2, z2]: [f64; 3]) -> f64 {
    ((x1 - x2).powi(2) + (y1 - y2).powi(2) + (z1 - z2).powi(2)).sqrt()
}

fn main() {
    let ranges: Vec<[f64; 3]> = stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .split(',')
                .map(|s| s.parse::<f64>().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap()
        })
        .collect();
    let n = ranges.len();

    let mut edges: Vec<_> = ranges
        .iter()
        .enumerate()
        .flat_map(|(v1, &v1_loc)| {
            ranges
                .iter()
                .enumerate()
                .filter(move |(v2, _)| v1 < *v2)
                .map(move |(v2, &v2_loc)| (v1, v2, dist(v1_loc, v2_loc)))
        })
        .collect();

    edges.sort_unstable_by(|l, r| l.2.total_cmp(&r.2));
    edges.truncate(n);

    let mut set = DisjointSet::new(n);
    for (v1, v2, _) in edges {
        dbg!(v1, v2);
        set.union(v1, v2);
    }

    let mut map: HashMap<usize, usize> = HashMap::new();
    for rep in (0..n).map(|i| set.rep(i)) {
        map.entry(rep).and_modify(|v| *v += 1).or_insert(1);
    }

    let mut counts: Vec<_> = map.values().copied().collect();
    let partition_point = counts.len() - 3;
    counts.select_nth_unstable(partition_point);
    let ans: usize = counts.iter().copied().rev().take(3).product();
    println!("Answer = {ans}");
}
