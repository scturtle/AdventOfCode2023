use itertools::Itertools;

#[test]
fn day09() {
    let txt = aoc::get_input(9).unwrap();
    let seqs: Vec<Vec<i64>> = txt
        .trim()
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|t| t.parse().unwrap())
                .collect_vec()
        })
        .collect_vec();

    let mut res1 = 0;
    let mut res2 = 0;
    for seq in seqs {
        let mut v = vec![seq];
        while v.last().unwrap().iter().any(|&t| t != 0) {
            v.push(
                v.last()
                    .unwrap()
                    .windows(2)
                    .map(|s| s[1] - s[0])
                    .collect_vec(),
            );
        }
        // part one
        let mut t = 0;
        for u in v.iter_mut().rev() {
            let last = *u.last().unwrap();
            u.push(last + t);
            t += last;
        }
        res1 += t;
        // part two
        let mut t = 0;
        for u in v.iter_mut().rev() {
            let first = *u.first().unwrap();
            u.insert(0, first - t);
            t = first - t;
        }
        res2 += t;
    }
    dbg!(res1);
    dbg!(res2);
}
