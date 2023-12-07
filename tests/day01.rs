use itertools::Itertools;

#[test]
fn day01() {
    let txt = aoc::get_input(1).unwrap();
    let pats: Vec<(usize, String)> = (0..=9)
        .map(|d| d.to_string())
        .enumerate()
        .chain(
            "one two three four five six seven eight nine"
                .split(' ')
                .map(|s| s.to_string())
                .enumerate()
                .map(|(i, s)| (i + 1, s)),
        )
        .collect();
    let mut res: usize = 0;
    for line in txt.trim().lines() {
        let (_, first) = pats
            .iter()
            .filter_map(|(i, pat)| line.find(pat).map(|j| (j, i)))
            .sorted()
            .next()
            .unwrap();
        let (_, last) = pats
            .iter()
            .filter_map(|(i, pat)| line.rfind(pat).map(|j| (j, i)))
            .sorted()
            .next_back()
            .unwrap();
        res += first * 10 + last;
    }
    dbg!(res);
}
