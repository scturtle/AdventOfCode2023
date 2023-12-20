use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    combinator::{eof, map, opt},
    multi::separated_list1 as sep1,
    sequence::tuple,
    IResult,
};
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone)]
enum Module<'a> {
    Broadcaster {
        lbl: &'a str,
        to: Vec<&'a str>,
    },
    FlipFlop {
        lbl: &'a str,
        to: Vec<&'a str>,
        state: bool,
    },
    Conjunction {
        lbl: &'a str,
        to: Vec<&'a str>,
        state: BTreeMap<&'a str, bool>,
    },
}

fn module(s: &str) -> IResult<&str, Module> {
    map(
        tuple((
            opt(one_of("%&")),
            alpha1,
            tag(" -> "),
            sep1(tag(", "), alpha1),
        )),
        |(sym, lbl, _, to)| match sym {
            None => Module::Broadcaster {
                lbl: "broadcaster",
                to,
            },
            Some('%') => Module::FlipFlop {
                lbl,
                to,
                state: false,
            },
            Some('&') => Module::Conjunction {
                lbl,
                to,
                state: BTreeMap::new(),
            },
            _ => unreachable!(),
        },
    )(s)
}

fn parse(s: &str) -> IResult<&str, Vec<Module>> {
    let (s, modules) = sep1(line_ending, module)(s)?;
    let (s, _) = alt((line_ending, eof))(s)?;
    Ok((s, modules))
}

#[test]
fn day20() {
    let txt = aoc::get_input(20).unwrap();
    let modules = parse(&txt).unwrap().1;
    let mut modules: BTreeMap<_, _> = modules
        .into_iter()
        .map(|m| match m {
            Module::Broadcaster { lbl, .. } => (lbl, m),
            Module::FlipFlop { lbl, .. } => (lbl, m),
            Module::Conjunction { lbl, .. } => (lbl, m),
        })
        .collect();

    let mut pre = BTreeMap::<&str, Vec<&str>>::new();
    for (lbl, m) in &modules {
        let to = match m {
            Module::Broadcaster { to, .. } => to,
            Module::FlipFlop { to, .. } => to,
            Module::Conjunction { to, .. } => to,
        };
        for &to in to {
            pre.entry(to).or_default().push(lbl);
        }
    }
    for (lbl, m) in &mut modules {
        if let Module::Conjunction { state, .. } = m {
            for pre in pre.get(lbl).unwrap() {
                state.insert(pre, false);
            }
        }
    }

    // for part two
    let mut periods = BTreeMap::<&str, usize>::new();

    let mut low_cnt = 0;
    let mut high_cnt = 0;
    for iter in 0.. {
        let mut q = VecDeque::new();
        q.push_back(("button", "broadcaster", false));
        while let Some((from, lbl, pulse)) = q.pop_front() {
            if pulse {
                high_cnt += 1;
            } else {
                low_cnt += 1;
            }
            if let Some(m) = modules.get_mut(lbl) {
                match m {
                    Module::Broadcaster { lbl, to } => {
                        for to in to {
                            q.push_back((lbl, to, pulse));
                        }
                    }
                    Module::FlipFlop { lbl, to, state } => {
                        if !pulse {
                            *state = !*state;
                            let pulse = *state;
                            for to in to {
                                q.push_back((lbl, to, pulse));
                            }
                        }
                    }
                    Module::Conjunction { lbl, to, state } => {
                        // part two
                        if to == &["rx"] && pulse && !periods.contains_key(from) {
                            periods.insert(from, iter + 1);
                            if periods.len() == state.len() {
                                dbg!(periods.values().product::<usize>());
                                return;
                            }
                        }
                        state.insert(from, pulse);
                        let pulse = state.values().all_equal_value() != Ok(&true);
                        for to in to {
                            q.push_back((lbl, to, pulse));
                        }
                    }
                }
            }
        }
        // part one
        if iter + 1 == 1000 {
            dbg!(low_cnt * high_cnt);
        }
    }
}
