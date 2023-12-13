use itertools::Itertools;

#[test]
fn day13() {
    let txt = aoc::get_input(13).unwrap();

    let ms = txt
        .split("\n\n")
        .map(|m| m.lines().map(|l| l.trim().as_bytes()).collect_vec())
        .collect_vec();

    let mut res1 = 0;
    for m in &ms {
        let (h, w) = (m.len(), m[0].len());
        for j in 1..w {
            let n = j.min(w - j);
            if (0..n).all(|jj| (0..h).all(|i| m[i][j - 1 - jj] == m[i][j + jj])) {
                res1 += j;
            }
        }
        for i in 1..h {
            let n = i.min(h - i);
            if (0..n).all(|ii| (0..w).all(|j| m[i - 1 - ii][j] == m[i + ii][j])) {
                res1 += i * 100;
            }
        }
    }
    dbg!(res1);

    let mut res2 = 0;
    for m in &ms {
        let (h, w) = (m.len(), m[0].len());
        for j in 1..w {
            if 1 == (0..j.min(w - j))
                .map(|jj| {
                    (0..h)
                        .map(|i| (m[i][j - 1 - jj] != m[i][j + jj]) as usize)
                        .sum::<usize>()
                })
                .sum::<usize>()
            {
                res2 += j;
            }
        }
        for i in 1..h {
            if 1 == (0..i.min(h - i))
                .map(|ii| {
                    (0..w)
                        .map(|j| (m[i - 1 - ii][j] != m[i + ii][j]) as usize)
                        .sum::<usize>()
                })
                .sum::<usize>()
            {
                res2 += i * 100;
            }
        }
    }
    dbg!(res2);
}
