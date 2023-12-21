use ahash::AHashMap;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, digit1, line_ending, one_of},
    combinator::{eof, map, map_res},
    multi::{many1, separated_list1 as sep1},
    sequence::{preceded as pre, terminated as term, tuple},
    IResult,
};

fn int<T: std::str::FromStr>(s: &str) -> IResult<&str, T> {
    map_res(digit1, str::parse::<T>)(s)
}

#[derive(Debug)]
enum Ruler<'a> {
    LessThan(char, usize, &'a str),
    LargerThan(char, usize, &'a str),
    Goto(&'a str),
}

fn ruler(s: &str) -> IResult<&str, Ruler> {
    let less_or_larger = map(
        tuple((one_of("xmas"), one_of("<>"), int::<usize>, tag(":"), alpha1)),
        |(cat, op, val, _, lbl)| {
            if op == '<' {
                Ruler::LessThan(cat, val, lbl)
            } else {
                Ruler::LargerThan(cat, val, lbl)
            }
        },
    );
    let goto = map(alpha1, Ruler::Goto);
    alt((less_or_larger, goto))(s)
}

#[derive(Debug)]
struct Workflow<'a> {
    lbl: &'a str,
    rulers: Vec<Ruler<'a>>,
}

fn workflow(s: &str) -> IResult<&str, Workflow> {
    let (s, lbl) = term(alpha1, tag("{"))(s)?;
    let (s, rulers) = sep1(tag(","), ruler)(s)?;
    let (s, _) = tag("}")(s)?;
    Ok((s, Workflow { lbl, rulers }))
}

#[derive(Debug, Clone, Copy)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

fn part(s: &str) -> IResult<&str, Part> {
    let usize_ = int::<usize>;
    let (s, (x, m, a, s_)) = tuple((
        pre(tag("{x="), usize_),
        pre(tag(",m="), usize_),
        pre(tag(",a="), usize_),
        pre(tag(",s="), usize_),
    ))(s)?;
    let (s, _) = tag("}")(s)?;
    Ok((s, Part { x, m, a, s: s_ }))
}

fn parse(s: &str) -> IResult<&str, (Vec<Workflow>, Vec<Part>)> {
    let (s, workflows) = sep1(line_ending, workflow)(s)?;
    let (s, _) = many1(line_ending)(s)?;
    let (s, parts) = sep1(line_ending, part)(s)?;
    let (s, _) = alt((line_ending, eof))(s)?;
    Ok((s, (workflows, parts)))
}

fn get(cat: char, part: Part) -> usize {
    let Part { x, m, a, s } = part;
    match cat {
        'x' => x,
        'm' => m,
        'a' => a,
        's' => s,
        _ => unreachable!(),
    }
}

fn go<'a>(part: Part, w: &'a Workflow) -> &'a str {
    for r in &w.rulers {
        match *r {
            Ruler::LessThan(cat, val, lbl) => {
                if get(cat, part) < val {
                    return lbl;
                }
            }
            Ruler::LargerThan(cat, val, lbl) => {
                if get(cat, part) > val {
                    return lbl;
                }
            }
            Ruler::Goto(lbl) => {
                return lbl;
            }
        }
    }
    unreachable!()
}

fn modify(mut part: Part, cat: char, val: usize) -> Part {
    match cat {
        'x' => part.x = val,
        'm' => part.m = val,
        'a' => part.a = val,
        's' => part.s = val,
        _ => unreachable!(),
    }
    part
}

fn dfs(
    mut min: Part,
    mut max: Part,
    cur: &str,
    workflows: &AHashMap<&str, Workflow>,
    res: &mut usize,
) {
    if min.x > max.x || min.m > max.m || min.a > max.a || min.s > max.s {
        return;
    }
    if cur == "A" || cur == "R" {
        if cur == "A" {
            *res += (max.x - min.x + 1)
                * (max.m - min.m + 1)
                * (max.a - min.a + 1)
                * (max.s - min.s + 1);
        }
        return;
    }
    let w = workflows.get(cur).unwrap();
    for ruler in &w.rulers {
        match *ruler {
            Ruler::LessThan(cat, val, lbl) => {
                let tmp = modify(max, cat, val - 1);
                dfs(min, tmp, lbl, workflows, res);
                min = modify(min, cat, val);
            }
            Ruler::LargerThan(cat, val, lbl) => {
                let tmp = modify(min, cat, val + 1);
                dfs(tmp, max, lbl, workflows, res);
                max = modify(max, cat, val);
            }
            Ruler::Goto(lbl) => {
                dfs(min, max, lbl, workflows, res);
            }
        }
    }
}

#[test]
fn day19() {
    let txt = aoc::get_input(19).unwrap();
    let (workflows, parts) = parse(&txt).unwrap().1;
    let workflows: AHashMap<&str, _> = workflows.into_iter().map(|w| (w.lbl, w)).collect();

    let mut res1 = 0;
    for part in parts {
        let mut cur = "in";
        while cur != "A" && cur != "R" {
            cur = go(part, workflows.get(cur).unwrap());
        }
        if cur == "A" {
            res1 += part.x + part.m + part.a + part.s;
        }
    }
    dbg!(res1);

    let min = Part {
        x: 1,
        m: 1,
        a: 1,
        s: 1,
    };
    let max = Part {
        x: 4000,
        m: 4000,
        a: 4000,
        s: 4000,
    };
    let mut res2 = 0;
    dfs(min, max, "in", &workflows, &mut res2);
    dbg!(res2);
}
