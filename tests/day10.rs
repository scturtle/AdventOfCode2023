use itertools::{iproduct, Itertools};

#[test]
fn day10() {
    let txt = aoc::get_input(10).unwrap();

    let m = txt
        .trim()
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect_vec();
    let (h, w) = (m.len(), m[0].len());

    let dir: ahash::AHashMap<u8, [(i32, i32); 2]> = [
        (b'F', [(1, 0), (0, 1)]),
        (b'L', [(-1, 0), (0, 1)]),
        (b'J', [(-1, 0), (0, -1)]),
        (b'7', [(1, 0), (0, -1)]),
        (b'-', [(0, -1), (0, 1)]),
        (b'|', [(-1, 0), (1, 0)]),
    ]
    .into_iter()
    .collect();

    // infer start position
    let s = iproduct!(0..h, 0..w)
        .find(|&(i, j)| m[i][j] == b'S')
        .unwrap();
    let s = (s.0 as i32, s.1 as i32);

    // infer start symbol
    let s_sym = dir
        .keys()
        .cloned()
        .find(|k| {
            dir[k].iter().all(|(ni, nj)| {
                let (i, j) = s;
                if i + ni < 0 || i + ni == h as i32 || j + nj < 0 || j + nj == w as i32 {
                    return false;
                }
                let n_sym = m[(i + ni) as usize][(j + nj) as usize];
                if n_sym == b'.' {
                    return false;
                }
                dir[&n_sym]
                    .iter()
                    .any(|(nni, nnj)| ni + nni == 0 && nj + nnj == 0)
            })
        })
        .unwrap();

    // part one
    let mut res1 = 0;
    let mut q = std::collections::VecDeque::new();
    let mut saw = ahash::AHashSet::new();
    q.push_back((s, 0));
    saw.insert(s);
    while let Some(u) = q.pop_front() {
        let ((i, j), dis) = u;
        res1 = res1.max(dis);
        let sym = if u.0 == s {
            s_sym
        } else {
            m[i as usize][j as usize]
        };
        for (ni, nj) in dir[&sym] {
            let (ni, nj) = (i + ni, j + nj);
            if saw.insert((ni, nj)) {
                q.push_back(((ni, nj), dis + 1));
            }
        }
    }
    dbg!(res1);

    // part two
    let mut res2 = 0;
    #[allow(clippy::needless_range_loop)]
    for i in 0..h {
        let mut cnt = 0;
        let mut last = None;
        for j in 0..w {
            if saw.contains(&(i as i32, j as i32)) {
                let c = if m[i][j] == b'S' { s_sym } else { m[i][j] };
                if c != b'-' {
                    // NOTE: we count "|" / "F-*J" / "L-*7" as one vertical line but not "F-*7" and "L-*J"
                    if c == b'|'
                        || (last == Some(b'F') && c == b'J')
                        || (last == Some(b'L') && c == b'7')
                    {
                        cnt += 1;
                    }
                    last = Some(c);
                }
            } else if cnt & 1 == 1 {
                res2 += 1;
            }
        }
    }
    dbg!(res2);
}
