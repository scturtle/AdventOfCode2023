use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{digit1, line_ending, space1};
use nom::combinator::{eof, map_res};
use nom::multi::many1;
use nom::sequence::{pair, preceded, terminated};
use nom::IResult;

#[derive(Debug)]
struct Card {
    win: Vec<usize>,
    mine: Vec<usize>,
}

fn card(s: &str) -> IResult<&str, Card> {
    let (s, _) = pair(take_until(":"), tag(":"))(s)?;
    let (s, win) = many1(preceded(space1, map_res(digit1, str::parse)))(s)?;
    let (s, _) = pair(space1, tag("|"))(s)?;
    let (s, mine) = many1(preceded(space1, map_res(digit1, str::parse)))(s)?;
    let (s, _) = alt((line_ending, eof))(s)?;
    Ok((s, Card { win, mine }))
}

fn cards(s: &str) -> IResult<&str, Vec<Card>> {
    terminated(many1(card), eof)(s)
}

#[test]
fn day04() {
    let txt = aoc::get_input(4).unwrap();
    let (_, cards) = cards(&txt).unwrap();
    let mut res1 = 0;
    let mut copies = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let cnt = card.mine.iter().filter(|n| card.win.contains(n)).count();
        // for part 1
        if cnt > 0 {
            res1 += 2_usize.pow(cnt as u32 - 1)
        }
        // for part 2
        for j in i + 1..(i + cnt + 1).min(cards.len()) {
            copies[j] += copies[i];
        }
    }
    dbg!(res1);
    dbg!(copies.iter().sum::<usize>());
}
