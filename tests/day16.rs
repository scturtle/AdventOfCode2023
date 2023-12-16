use itertools::{iproduct, Itertools};

#[test]
fn day16() {
    let txt = aoc::get_input(16).unwrap();
    let m = txt
        .trim()
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect_vec();
    let (h, w) = (m.len(), m[0].len());
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    let f = |init: (i32, i32, usize)| {
        let mut c = vec![vec![[false; 4]; w]; h];
        let mut q = vec![];
        q.push(init);
        while let Some((px, py, d)) = q.pop() {
            let (dx, dy) = dirs[d];
            let (x, y) = (px + dx, py + dy);
            if x < 0 || y < 0 || x == h as i32 || y == h as i32 {
                continue;
            }
            if c[x as usize][y as usize][d] {
                continue;
            } else {
                c[x as usize][y as usize][d] = true;
            }
            match m[x as usize][y as usize] {
                b'.' => {
                    q.push((x, y, d));
                }
                b'/' => {
                    // 0 -> 3, 1 -> 2, 2 -> 1, 3 -> 0
                    q.push((x, y, 3 - d));
                }
                b'\\' => {
                    // 0 -> 1, 1 -> 0, 2 -> 3, 3 -> 2
                    let d = if d < 2 { 1 - d } else { 5 - d };
                    q.push((x, y, d));
                }
                b'|' => {
                    if d == 0 || d == 2 {
                        q.push((x, y, d));
                    } else {
                        q.push((x, y, 0));
                        q.push((x, y, 2));
                    }
                }
                b'-' => {
                    if d == 1 || d == 3 {
                        q.push((x, y, d));
                    } else {
                        q.push((x, y, 1));
                        q.push((x, y, 3));
                    }
                }
                _ => unreachable!(),
            }
        }
        iproduct!(0..h, 0..w)
            .filter(|&(i, j)| c[i][j].iter().any(|&x| x))
            .count()
    };

    dbg!(f((0, -1, 3)));

    let mut max_cnt = 0;
    for i in 0..h as i32 {
        max_cnt = max_cnt.max(f((i, -1, 3)));
        max_cnt = max_cnt.max(f((i, w as i32, 1)));
    }
    for j in 0..w as i32 {
        max_cnt = max_cnt.max(f((-1, j, 2)));
        max_cnt = max_cnt.max(f((h as i32, j, 0)));
    }
    dbg!(max_cnt);
}
