use ahash::AHashMap;
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

type Cache = AHashMap<((usize, usize), (usize, usize)), usize>;

fn dfs(s: &[u8], n: &[usize], sg: (usize, usize), ng: (usize, usize), cache: &mut Cache) -> usize {
    if let Some(&cached) = cache.get(&(sg, ng)) {
        return cached;
    }
    let cnt = if ng.0 == ng.1 {
        if s[sg.0..sg.1].iter().all(|&c| c != b'#') {
            1
        } else {
            0
        }
    } else {
        let mut cnt = 0;
        for i in sg.0..sg.1 {
            let j = (ng.0 + ng.1) / 2;
            let x = n[j];
            let (nl, nr) = ((ng.0, j), (j + 1, ng.1));
            // match "^#+" at the start
            if i == 0 && j == 0 && x <= sg.1 && s[..x].iter().all(|&c| c != b'.') {
                let (sl, sr) = ((sg.0, 0), (i + x, sg.1));
                cnt += dfs(s, n, sl, nl, cache) * dfs(s, n, sr, nr, cache);
            }
            // match ".#+"
            if s[i] != b'#' && i + 1 + x <= sg.1 && s[i + 1..i + 1 + x].iter().all(|&c| c != b'.') {
                let (sl, sr) = ((sg.0, i), (i + 1 + x, sg.1));
                cnt += dfs(s, n, sl, nl, cache) * dfs(s, n, sr, nr, cache);
            }
        }
        cnt
    };
    cache.insert((sg, ng), cnt);
    cnt
}

#[test]
fn day12() {
    let txt = aoc::get_input(12).unwrap();

    let mut res1 = 0;
    let Input { rows } = parse(&txt).unwrap().1;
    for (ss, ns) in &rows {
        let mut cache: Cache = Cache::new();
        let cnt = dfs(ss.as_bytes(), ns, (0, ss.len()), (0, ns.len()), &mut cache);
        res1 += cnt;
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
        let ns = std::iter::once(ns)
            .cycle()
            .take(5)
            .flatten()
            .cloned()
            .collect_vec();
        let mut cache: Cache = Cache::new();
        let cnt = dfs(ss.as_bytes(), &ns, (0, ss.len()), (0, ns.len()), &mut cache);
        res2 += cnt;
    }
    dbg!(res2);
}
