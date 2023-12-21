use itertools::{iproduct, Itertools};

#[test]
fn day21() {
    let txt = aoc::get_input(21).unwrap();

    let m = txt
        .trim()
        .lines()
        .map(|l| l.trim().bytes().collect_vec())
        .collect_vec();
    let (h, w) = (m.len() as isize, m[0].len() as isize);

    let (sx, sy) = iproduct!(0..h, 0..w)
        .find(|&(i, j)| m[i as usize][j as usize] == b'S')
        .unwrap();

    let count = |step| -> usize {
        let mut cur = vec![(sx, sy)];
        for _ in 0..step {
            let mut nxt = vec![];
            let mut saw = ahash::AHashSet::new();
            for (x, y) in cur {
                for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                    let (nx, ny) = (x + dx, y + dy);
                    if m[nx.rem_euclid(h) as usize][ny.rem_euclid(w) as usize] == b'#' {
                        continue;
                    }
                    if saw.insert((nx, ny)) {
                        nxt.push((nx, ny));
                    }
                }
            }
            cur = nxt;
        }
        cur.len()
    };
    // part one
    dbg!(count(64));

    // part two
    // well... hint from reddit, sovle quadratic funciton
    let c = count(65);
    let a_b_c = count(65 + 131);
    let a4_b2_c = count(65 + 131 * 2);
    dbg!(c, a_b_c, a4_b2_c);
    let a = (a4_b2_c + c - 2 * a_b_c) / 2;
    let b = a_b_c - c - a;
    dbg!(a, b, c);
    let x = 202300;
    dbg!(a * x * x + b * x + c);
}
