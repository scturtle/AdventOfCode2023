use nom::{bytes::complete::take_till, character::complete::one_of, IResult};

fn parse(s: &str) -> IResult<&str, (&str, char, Option<usize>)> {
    let (s, lbl) = take_till(|c| c == '=' || c == '-')(s)?;
    let (s, op) = one_of("-=")(s)?;
    let lens = if s.is_empty() { None } else { s.parse().ok() };
    Ok((s, (lbl, op, lens)))
}

fn hash(s: impl Iterator<Item = u8>) -> usize {
    s.fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

#[test]
fn day15() {
    let txt = aoc::get_input(15).unwrap();

    let instrs: Vec<&str> = txt.trim().split(',').collect();
    let res1: usize = instrs.iter().map(|t| hash(t.bytes())).sum();
    dbg!(res1);

    let mut boxs: Vec<Vec<(&str, usize)>> = vec![vec![]; 256];
    for instr in &instrs {
        let (lbl, op, lens) = parse(instr).unwrap().1;
        let bx = boxs.get_mut(hash(lbl.bytes())).unwrap();
        if op == '-' {
            if let Some(idx) = bx.iter().position(|t| t.0 == lbl) {
                bx.remove(idx);
            }
        } else if op == '=' {
            let lens = lens.unwrap();
            if let Some(idx) = bx.iter().position(|t| t.0 == lbl) {
                bx[idx] = (lbl, lens);
            } else {
                bx.push((lbl, lens));
            }
        }
    }
    let res2 = boxs
        .iter()
        .enumerate()
        .map(|(i, bx)| {
            bx.iter()
                .enumerate()
                .map(|(j, (_, lens))| (i + 1) * (j + 1) * lens)
                .sum::<usize>()
        })
        .sum::<usize>();
    dbg!(res2);
}
