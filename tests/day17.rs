use ahash::{AHashMap, AHashSet};
use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

#[test]
fn day17() {
    let txt = aoc::get_input(17).unwrap();
    let m = txt
        .trim()
        .lines()
        .map(|l| l.bytes().map(|c| (c - b'0') as usize).collect_vec())
        .collect_vec();
    let (h, w) = (m.len() as i32, m[0].len() as i32);
    let dirs = [(-1, 0), (0, -1), (1, 0), (0, 1)];

    // part one
    // let (min_cont, max_cont) = (1, 3);
    // part two
    let (min_cont, max_cont) = (4, 10);

    // dijkstra
    let mut pq = BinaryHeap::new();
    let mut dist = AHashMap::new();
    let mut saw = AHashSet::new();
    pq.push((Reverse(0), (0, 0), (1, 2))); // cost => pos, (cont, dir)
    dist.insert(((0, 0), (1, 2)), 0); // (pos, (cont, dir)) => cost
    while let Some((_, u, ds)) = pq.pop() {
        let u_cost = dist[&(u, ds)];
        if u == (h - 1, w - 1) {
            dbg!(u_cost);
            break;
        }
        if !saw.insert((u, ds)) {
            continue;
        }
        let (dcnt, dir) = ds;
        #[allow(clippy::needless_range_loop)]
        'next_dir: for ndir in 0..dirs.len() {
            // no turn back
            if (dir + 4 - ndir) % 4 == 2 {
                continue;
            }
            // no more than max_cont
            if dcnt == max_cont && dir == ndir {
                continue;
            }
            let mut vdcnt = if dir == ndir { dcnt + 1 } else { 1 };
            let mut v = (u.0 + dirs[ndir].0, u.1 + dirs[ndir].1);
            if v.0 < 0 || v.1 < 0 || v.0 >= h || v.1 >= w {
                continue;
            }
            let mut cost = m[v.0 as usize][v.1 as usize];
            // no less than min_cont
            while vdcnt < min_cont {
                vdcnt += 1;
                v = (v.0 + dirs[ndir].0, v.1 + dirs[ndir].1);
                if v.0 < 0 || v.1 < 0 || v.0 >= h || v.1 >= w {
                    continue 'next_dir;
                }
                cost += m[v.0 as usize][v.1 as usize];
            }
            let vds = (vdcnt, ndir);
            let old_dist = *dist.get(&(v, vds)).unwrap_or(&usize::MAX);
            let new_dist = u_cost + cost;
            if old_dist > new_dist {
                dist.insert((v, vds), new_dist);
                pq.push((Reverse(new_dist), v, vds));
            }
        }
    }
}
