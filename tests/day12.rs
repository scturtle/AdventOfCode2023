use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{digit1, line_ending},
    combinator::{eof, map_res},
    multi::{separated_list0, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

struct Input<'a> {
    rows: Vec<(&'a str, Vec<usize>)>,
}

fn parse(s: &str) -> IResult<&str, Input> {
    let (s, rows) = separated_list0(
        line_ending,
        tuple((
            terminated(take_until(" "), tag(" ")),
            separated_list1(tag(","), map_res(digit1, str::parse)),
        )),
    )(s)?;
    let (s, _) = alt((line_ending, eof))(s)?;
    Ok((s, Input { rows }))
}

fn dp(s: &[u8], n: &[usize]) -> usize {
    let mut cnt = vec![vec![0usize; n.len() + 1]; s.len() + 1];
    // s[..i] w/o "#"
    for i in 0..=s.len() {
        if s[..i].iter().all(|&c| c != b'#') {
            cnt[i][0] = 1;
        }
    }
    for i in 1..=s.len() {
        for j in 1..=n.len() {
            let x = n[j - 1];
            let mut cur = 0;
            for m in 0..i {
                // s[0..m]: cnt[m][j - 1]
                // s[m..m + 1 + x] is ".#+"
                if m + 1 + x <= i && s[m] != b'#' && s[m + 1..m + 1 + x].iter().all(|&c| c != b'.')
                // s[m + 1 + x..i] w/o "#"
                    && s[m + 1 + x..i].iter().all(|&c| c != b'#')
                {
                    cur += cnt[m][j - 1];
                }
            }
            cnt[i][j] = cur;
        }
    }
    cnt[s.len()][n.len()]
}

#[test]
fn day12() {
    let txt = aoc::get_input(12).unwrap();

    let mut res1 = 0;
    let Input { rows } = parse(&txt).unwrap().1;
    for (ss, ns) in &rows {
        // prepend a "." before to match ".*+" for each item
        let ss = ".".to_string() + ss;
        res1 += dp(ss.as_bytes(), ns);
    }
    dbg!(res1);

    let mut res2 = 0;
    let Input { rows } = parse(&txt).unwrap().1;
    for (ss, ns) in rows.iter() {
        let ss = std::iter::once(ss)
            .cycle()
            .take(5)
            .cloned()
            .collect_vec()
            .join("?");
        // prepend a "." before to match ".*+" for each item
        let ss = ".".to_string() + &ss;
        let ns = std::iter::once(ns)
            .cycle()
            .take(5)
            .flatten()
            .cloned()
            .collect_vec();
        res2 += dp(ss.as_bytes(), &ns);
    }
    dbg!(res2);
}
