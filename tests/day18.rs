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

type Line = ((i64, i64), (i64, i64));

fn test_hori_line(hori_line: Line, vert_lines: &[Line]) -> bool {
    let (st, ed) = hori_line;
    let xs = vert_lines
        .iter()
        .filter_map(|&(a, b)| {
            if a == st || a == ed {
                Some(b.0)
            } else if b == st || b == ed {
                Some(a.0)
            } else {
                None
            }
        })
        .collect_vec();
    assert!(xs.len() == 2);
    let x = st.0;
    (xs[0] < x && x < xs[1]) || (xs[1] < x && x < xs[0])
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

    let mut cur = (0, 0);
    let mut xs = vec![0];
    let mut hori_lines = vec![];
    let mut vert_lines = vec![];
    for &(dir, step) in &moves {
        let (dx, dy) = dir_to_dxdy(dir);
        let nxt = (cur.0 + dx * step, cur.1 + dy * step);
        xs.push(nxt.0);
        if cur.0 == nxt.0 {
            hori_lines.push(if cur.1 < nxt.1 {
                (cur, nxt)
            } else {
                (nxt, cur)
            });
        }
        if cur.1 == nxt.1 {
            vert_lines.push(if cur.0 < nxt.0 {
                (cur, nxt)
            } else {
                (nxt, cur)
            });
        }
        cur = nxt;
    }

    let xs: Vec<i64> = xs.into_iter().sorted().unique().collect();

    let cnt_line = |x: i64| -> i64 {
        let mut pts = vec![];
        for &(st, ed) in &hori_lines {
            if st.0 == x {
                let change = test_hori_line((st, ed), &vert_lines);
                pts.push((st.1, ed.1, change));
            }
        }
        for &(st, ed) in &vert_lines {
            if st.0 < x && x < ed.0 {
                pts.push((st.1, ed.1, true));
            }
        }
        pts.sort();
        let mut cnt = 0;
        let mut inner = false;
        for (i, &(x, y, change)) in pts.iter().enumerate() {
            cnt += y - x + 1;
            if inner {
                cnt += x - pts[i - 1].1 - 1;
            }
            if change {
                inner = !inner;
            }
        }
        cnt
    };
    let cnt_rect = |x0: i64, x1: i64| -> i64 {
        let mut pts = vec![];
        for &(st, ed) in &vert_lines {
            if st.0 <= x0 && x1 <= ed.0 {
                pts.push(st.1);
            }
        }
        pts.sort();
        let mut cnt = 0;
        for cs in pts.chunks_exact(2) {
            cnt += cs[1] - cs[0] + 1;
        }
        cnt * (x1 - x0 - 1)
    };

    let mut tot = 0;
    tot += cnt_line(xs[0]);
    for i in 1..xs.len() {
        tot += cnt_rect(xs[i - 1], xs[i]);
        tot += cnt_line(xs[i]);
    }
    dbg!(tot);
}
