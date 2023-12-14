use itertools::{iproduct, Itertools};

#[test]
fn day14() {
    let txt = aoc::get_input(14).unwrap();
    let m: Vec<&[u8]> = txt.trim().lines().map(|l| l.as_bytes()).collect_vec();
    let (h, w) = (m.len(), m[0].len());
    let mut os = iproduct!(0..h, 0..w)
        .filter(|&(i, j)| m[i][j] == b'O')
        .map(|(i, j)| (i as i32, j as i32))
        .collect_vec();
    let mut hist = vec![];
    let mut pat = (None, None, None);
    for cycle in 0.. {
        for dir in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
            os.sort_by_key(|&(i, j)| (i * -dir.0, j * -dir.1));
            let mut new_os = std::collections::BTreeSet::new();
            for &(mut i, mut j) in &os {
                loop {
                    let (ni, nj) = (i + dir.0, j + dir.1);
                    if ni < 0 || nj < 0 || ni == h as i32 || nj == w as i32 {
                        break;
                    }
                    if m[ni as usize][nj as usize] == b'#' || new_os.contains(&(ni, nj)) {
                        break;
                    }
                    (i, j) = (ni, nj);
                }
                new_os.insert((i, j));
            }
            os = new_os.into_iter().collect();
        }
        for (i, hos) in hist.iter().enumerate() {
            if &os == hos {
                if pat.0.is_none() {
                    pat.0 = Some(i);
                    pat.1 = Some(cycle);
                } else if Some(i) == pat.0 {
                    pat.2 = Some(cycle);
                }
                break;
            }
        }
        if pat.2.is_some() {
            break;
        }
        hist.push(os.clone());
    }

    let index =
        (1000000000 - 1 - pat.1.unwrap()) % (pat.2.unwrap() - pat.1.unwrap()) + pat.0.unwrap();
    dbg!(hist[index].iter().map(|(i, _)| h as i32 - i).sum::<i32>());
}
