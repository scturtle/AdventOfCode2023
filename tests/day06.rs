use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{eof, map_res};
use nom::multi::many1;
use nom::sequence::{delimited, preceded};
use nom::IResult;

fn parse(s: &str) -> IResult<&str, (Vec<usize>, Vec<usize>)> {
    let (s, times) = delimited(
        tag("Time:"),
        many1(preceded(space1, map_res(digit1, str::parse))),
        line_ending,
    )(s)?;
    let (s, dists) = delimited(
        tag("Distance:"),
        many1(preceded(space1, map_res(digit1, str::parse))),
        alt((line_ending, eof)),
    )(s)?;
    Ok((s, (times, dists)))
}

#[test]
fn day06() {
    let txt = aoc::get_input(6).unwrap();
    let (times, dists) = parse(&txt).unwrap().1;
    let mut res1 = 1;
    for (&time, &dist) in times.iter().zip(dists.iter()) {
        let is_ok = |speed: &usize| speed * (time - speed) > dist;
        let cnt = (1..time).filter(is_ok).count();
        res1 *= cnt;
    }
    dbg!(res1);

    let (time, dist) = txt
        .trim()
        .lines()
        .map(|line| {
            line.chars()
                .filter(char::is_ascii_digit)
                .collect::<String>()
                .parse::<usize>()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    let is_ok = |speed| speed * (time - speed) > dist;
    let (mut l, mut r) = (0, time / 2);
    while l + 1 != r {
        let m = (l + r) / 2;
        *(if !is_ok(m) { &mut l } else { &mut r }) = m;
    }
    let tmp = r;
    let (mut l, mut r) = (time / 2, time);
    while l + 1 != r {
        let m = (l + r) / 2;
        *(if is_ok(m) { &mut l } else { &mut r }) = m;
    }
    let res2 = r - tmp;
    dbg!(res2);
}
