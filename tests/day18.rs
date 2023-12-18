use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{digit1, line_ending, one_of},
    combinator::{eof, map_res},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

fn parse(s: &str) -> IResult<&str, Vec<(char, usize, &str)>> {
    let (s, input) = separated_list1(
        line_ending,
        tuple((
            terminated(one_of("UDLR"), tag(" ")),
            terminated(map_res(digit1, str::parse), tag(" (#")),
            terminated(take(6usize), tag(")")),
        )),
    )(s)?;
    let (s, _) = alt((line_ending, eof))(s)?;
    Ok((s, input))
}

use itertools::Itertools;

fn dir_to_dxdy(dir: char) -> (i64, i64) {
    match dir {
        'U' => (-1, 0),
        'D' => (1, 0),
        'L' => (0, -1),
        'R' => (0, 1),
        _ => unreachable!(),
    }
}

fn digit_to_dir(d: u8) -> char {
    match d - b'0' {
        0 => 'R',
        1 => 'D',
        2 => 'L',
        3 => 'U',
        _ => unreachable!(),
    }
}

#[test]
fn day18() {
    let txt = aoc::get_input(18).unwrap();
    let input = parse(&txt).unwrap().1;

    // for part one
    // let moves = input
    //     .iter()
    //     .map(|&(dir, step, _)| (dir, step as i64))
    //     .collect_vec();

    // for part two
    let moves = input
        .iter()
        .map(|(_, _, s)| {
            (
                digit_to_dir(s.as_bytes()[5]),
                i64::from_str_radix(&s[..5], 16).unwrap(),
            )
        })
        .collect_vec();

    // https://www.reddit.com/r/adventofcode/comments/18l0qtr/comment/kdvrqv8
    // A: inner area (lack of border), i: inner point number, b: border point number
    // Pick’s theorem: A = i + b / 2 - 1
    // shoelace formula: A = 1/2 * ∑ (xi - xi_1) * (yi - yi_1)
    // We want sum of the one meter rect ('#') around all points.
    // So what we really need is i + b, that is inner point number plus border point number.
    // Plus b / 2 + 1 to both side of Pick's theorem is:
    // A + b / 2 + 1 = i + b
    // We can get the inner aera A from shoelace formula,
    // and the border point number count through walking.

    let mut cur = (0, 0);
    let mut aera2 = 0;
    let mut b = 1;
    for &(dir, step) in &moves {
        let (dx, dy) = dir_to_dxdy(dir);
        let nxt = (cur.0 + dx * step, cur.1 + dy * step);
        aera2 += (nxt.0 - cur.0) * (nxt.1 + cur.1);
        b += step;
        cur = nxt;
    }
    dbg!(aera2 / 2 + b / 2 + 1);
}
