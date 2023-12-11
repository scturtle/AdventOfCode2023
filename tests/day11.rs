use ahash::AHashSet;
use itertools::{iproduct, Itertools};

#[test]
fn day11() {
    let txt = aoc::get_input(11).unwrap();
    let m: Vec<&[u8]> = txt.trim().lines().map(|l| l.as_bytes()).collect_vec();
    let (h, w) = (m.len(), m[0].len());
    let xhs: AHashSet<usize> = (0..h)
        .filter(|&i| m[i].iter().all(|&b| b == b'.'))
        .collect();
    let xws: AHashSet<usize> = (0..w)
        .filter(|&j| (0..h).all(|i| m[i][j] == b'.'))
        .collect();
    let stars: Vec<(usize, usize)> = iproduct!(0..h, 0..w)
        .filter(|&(i, j)| m[i][j] == b'#')
        .collect();
    let mut res = 0;
    for i in 0..stars.len() - 1 {
        for j in i + 1..stars.len() {
            let s0 = stars[i];
            let s1 = stars[j];
            let (i0, i1) = (s0.0.min(s1.0), s0.0.max(s1.0));
            let (j0, j1) = (s0.1.min(s1.1), s0.1.max(s1.1));
            res += i1 - i0 + (1000000 - 1) * (i0..i1).filter(|i| xhs.contains(i)).count();
            res += j1 - j0 + (1000000 - 1) * (j0..j1).filter(|j| xws.contains(j)).count();
        }
    }
    dbg!(res);
}
