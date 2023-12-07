use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{eof, map_res};
use nom::multi::{many1, separated_list1};
use nom::sequence::{pair, terminated};
use nom::IResult;

#[derive(Debug)]
struct Range {
    dst: usize,
    src: usize,
    len: usize,
}

#[derive(Debug)]
struct Input {
    seeds: Vec<usize>,
    maps: Vec<Vec<Range>>,
}

fn range(s: &str) -> IResult<&str, Range> {
    let (s, v) = separated_list1(space1, map_res(digit1, str::parse))(s)?;
    let (s, _) = alt((line_ending, eof))(s)?;
    let (dst, src, len) = v.into_iter().collect_tuple().unwrap();
    Ok((s, Range { dst, src, len }))
}

fn map(s: &str) -> IResult<&str, Vec<Range>> {
    let (s, _) = pair(take_until("map:"), tag("map:\n"))(s)?;
    many1(range)(s)
}

fn input(s: &str) -> IResult<&str, Input> {
    let (s, _) = tag("seeds: ")(s)?;
    let (s, seeds) = separated_list1(space1, map_res(digit1, str::parse))(s)?;
    let (s, maps) = terminated(many1(map), eof)(s)?;
    Ok((s, Input { seeds, maps }))
}

fn merge(mut segs: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    segs.sort();
    let mut stack = vec![segs[0]];
    for t in segs.into_iter().skip(1) {
        let last = stack.last_mut().unwrap();
        if last.0 <= t.0 && t.0 <= last.0 + last.1 {
            last.1 = last.1.max(t.0 + t.1 - last.0);
        } else {
            stack.push(t);
        }
    }
    stack
}

#[test]
fn day05() {
    let txt = aoc::get_input(5).unwrap();
    let Input { seeds, maps } = input(&txt).unwrap().1;

    // part one
    let mut res1 = usize::MAX;
    for &seed in &seeds {
        let mut cur = seed;
        for map in &maps {
            for r in map {
                if cur >= r.src && cur < r.src + r.len {
                    cur = (cur - r.src) + r.dst;
                    break;
                }
            }
        }
        res1 = res1.min(cur);
    }
    dbg!(res1);

    // part two
    let segs = seeds.chunks_exact(2).map(|t| (t[0], t[1])).collect_vec();
    let mut segs = merge(segs);
    for map in &maps {
        let mut next_segs = vec![];
        let mut segs_iter = segs.iter();
        let mut cur = *segs_iter.next().unwrap();
        loop {
            let mut matched = false;
            for r in map {
                if cur.0 >= r.src && cur.0 < r.src + r.len {
                    let len = cur.1.min(r.src + r.len - cur.0);
                    next_segs.push((cur.0 - r.src + r.dst, len));
                    cur.0 += len;
                    cur.1 -= len;
                    matched = true;
                    break;
                }
            }
            // no rule matched, identity mapping
            if !matched {
                next_segs.push(cur);
                cur.0 += cur.1;
                cur.1 = 0;
            }
            // matched but no exhausted, match again
            if cur.1 != 0 {
                continue;
            }
            // this segment is done, do next segument
            let Some(seg) = segs_iter.next() else { break };
            cur = *seg;
        }
        segs = merge(next_segs);
    }
    dbg!(segs.first().unwrap().0);
}
