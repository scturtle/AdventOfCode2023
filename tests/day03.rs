use itertools::Itertools;
use std::collections::{BTreeMap, BTreeSet};

fn around(i: usize, j: usize, h: usize, w: usize) -> impl Iterator<Item = (usize, usize)> {
    let i_iter = [i.max(1) - 1, i, (i + 1).min(h - 1)].into_iter().unique();
    let j_iter = [j.max(1) - 1, j, (j + 1).min(w - 1)].into_iter().unique();
    i_iter
        .cartesian_product(j_iter)
        .filter(move |p| *p != (i, j))
}

#[test]
fn day03() {
    let txt = aoc::get_input(3).unwrap();
    let m = txt
        .trim()
        .lines()
        .map(|l| l.trim().as_bytes())
        .collect_vec();
    let (h, w) = (m.len(), m[0].len());
    let is_sym = |c: u8| c != b'.' && !c.is_ascii_digit();
    let mut res1 = 0;
    let mut gear_ratio: BTreeMap<_, Vec<usize>> = BTreeMap::new();
    let mut cur_gear = BTreeSet::new();
    for i in 0..h {
        let mut cur = 0;
        let mut adj = false;
        cur_gear.clear();
        for j in 0..w {
            let c = m[i][j];
            if c.is_ascii_digit() {
                cur = cur * 10 + (c - b'0') as usize;
                for (ii, jj) in around(i, j, h, w) {
                    if is_sym(m[ii][jj]) {
                        adj = true;
                        if m[ii][jj] == b'*' {
                            cur_gear.insert((ii, jj));
                        }
                    }
                }
            } else {
                if adj {
                    res1 += cur;
                    for &gear in &cur_gear {
                        gear_ratio.entry(gear).or_default().push(cur);
                    }
                }
                cur = 0;
                adj = false;
                cur_gear.clear();
            }
        }
        if adj {
            res1 += cur;
            for &gear in &cur_gear {
                gear_ratio.entry(gear).or_default().push(cur);
            }
        }
    }
    dbg!(res1);
    let mut res2 = 0;
    for (_, nums) in gear_ratio {
        if nums.len() == 2 {
            res2 += nums[0] * nums[1];
        }
    }
    dbg!(res2);
}
