use nom::{
    branch::alt,
    bytes::complete::{tag, take, take_until},
    character::complete::line_ending,
    combinator::eof,
    multi::{many1, separated_list0},
    sequence::{terminated, tuple},
    IResult,
};

struct Input<'a> {
    insts: &'a str,
    moves: Vec<(&'a str, &'a str, &'a str)>,
}

fn parse(s: &str) -> IResult<&str, Input> {
    let (s, insts) = terminated(take_until("\n"), many1(line_ending))(s)?;
    let (s, moves) = separated_list0(
        line_ending,
        tuple((
            terminated(take(3_usize), tag(" = (")),
            terminated(take(3_usize), tag(", ")),
            terminated(take(3_usize), tag(")")),
        )),
    )(s)?;
    let (s, _) = alt((line_ending, eof))(s)?;
    Ok((s, Input { insts, moves }))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[test]
fn day08() {
    let txt = aoc::get_input(8).unwrap();

    let Input { insts, moves } = parse(&txt).unwrap().1;
    let moves: ahash::AHashMap<&str, (&str, &str)> =
        moves.into_iter().map(|(idx, l, r)| (idx, (l, r))).collect();

    // part one
    // let starts: Vec<&str> = vec!["AAA"];

    // part two
    let starts: Vec<&str> = moves
        .keys()
        .filter(|t| t.as_bytes()[2] == b'A')
        .cloned()
        .collect();

    let mut tot = 1;
    for mut curr in starts {
        let mut step = 0;
        let mut inst = insts.chars().cycle();
        while curr.as_bytes()[2] != b'Z' {
            step += 1;
            curr = match inst.next().unwrap() {
                'L' => moves[curr].0,
                'R' => moves[curr].1,
                _ => unreachable!(),
            };
        }
        tot = lcm(tot, step);
    }
    dbg!(tot);
}
