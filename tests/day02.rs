use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{eof, map_res};
use nom::multi::{many0, separated_list0};
use nom::sequence::{delimited, terminated};
use nom::IResult;

#[derive(Debug)]
struct Count {
    r: usize,
    g: usize,
    b: usize,
}

#[derive(Debug)]
struct Game {
    id: usize,
    cnts: Vec<Count>,
}

fn color(s: &str) -> IResult<&str, (usize, &str)> {
    let (s, cnt) = terminated(map_res(digit1, str::parse), space1)(s)?;
    let (s, color) = alt((tag("red"), tag("green"), tag("blue")))(s)?;
    Ok((s, (cnt, color)))
}

fn count(s: &str) -> IResult<&str, Count> {
    let (s, colors) = separated_list0(tag(", "), color)(s)?;
    let mut count = Count { r: 0, g: 0, b: 0 };
    for (cnt, color) in colors {
        if let Some(value) = match color {
            "red" => Some(&mut count.r),
            "green" => Some(&mut count.g),
            "blue" => Some(&mut count.b),
            _ => None,
        } {
            *value = cnt;
        }
    }
    Ok((s, count))
}

fn game(s: &str) -> IResult<&str, Game> {
    let (s, id) = delimited(tag("Game "), map_res(digit1, str::parse), tag(": "))(s)?;
    let (s, cnts) = terminated(separated_list0(tag("; "), count), alt((line_ending, eof)))(s)?;
    Ok((s, Game { id, cnts }))
}

fn games(s: &str) -> IResult<&str, Vec<Game>> {
    terminated(many0(game), eof)(s)
}

#[test]
fn day02() {
    let txt = aoc::get_input(2).unwrap();
    let (_, games) = games(&txt).unwrap();
    let mut res1 = 0;
    let mut res2 = 0;
    for Game { id, cnts } in &games {
        if cnts.iter().all(|c| c.r <= 12 && c.g <= 13 && c.b <= 14) {
            res1 += id;
        }
        let max = cnts.iter().fold(Count { r: 0, g: 0, b: 0 }, |a, b| Count {
            r: a.r.max(b.r),
            g: a.g.max(b.g),
            b: a.b.max(b.b),
        });
        res2 += max.r * max.g * max.b;
    }
    dbg!(res1);
    dbg!(res2);
}
