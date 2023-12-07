use itertools::Itertools;

#[derive(Debug)]
struct Card {
    kind: u8,
    cards: [char; 5],
    bid: usize,
}

impl Card {
    fn new(cards: [char; 5], bid: usize) -> Self {
        let kind = Self::infer(cards);
        Self { cards, kind, bid }
    }

    fn infer(cards: [char; 5]) -> u8 {
        let groups = cards
            .iter()
            .sorted()
            .group_by(|&c| c)
            .into_iter()
            .map(|(_, g)| g.count())
            .sorted()
            .rev()
            .collect_vec();
        match groups.as_slice() {
            [5] => 6,
            [4, 1] => 5,
            [3, 2] => 4,
            [3, 1, 1] => 3,
            [2, 2, 1] => 2,
            [2, 1, 1, 1] => 1,
            [1, 1, 1, 1, 1] => 0,
            _ => unreachable!(),
        }
    }

    fn rekind(&mut self) {
        self.kind = "23456789TJQKA"
            .chars()
            .map(|faker| {
                let mut cards2 = self.cards;
                for c in &mut cards2 {
                    if c == &'J' {
                        *c = faker;
                    }
                }
                Self::infer(cards2)
            })
            .max()
            .unwrap();
    }
}

fn cmp(this: &Card, other: &Card, order: &str) -> std::cmp::Ordering {
    match this.kind.cmp(&other.kind) {
        core::cmp::Ordering::Equal => {}
        ord => return ord,
    }
    let cards0 = this
        .cards
        .iter()
        .map(|c| order.find(*c).unwrap())
        .collect_vec();
    let cards1 = other
        .cards
        .iter()
        .map(|c| order.find(*c).unwrap())
        .collect_vec();
    cards0.cmp(&cards1)
}

fn parse(s: &str) -> Vec<Card> {
    s.trim()
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(' ').unwrap();
            let cards = cards.chars().collect_vec().try_into().unwrap();
            let bid = bid.parse().unwrap();
            Card::new(cards, bid)
        })
        .collect()
}

#[test]
fn day07() {
    let txt = aoc::get_input(7).unwrap();
    let mut cards = parse(&txt);
    // part one
    cards.sort_by(|c1, c2| cmp(c1, c2, "23456789TJQKA"));
    let res1: usize = cards.iter().enumerate().map(|(i, c)| (i + 1) * c.bid).sum();
    dbg!(res1);
    // part two
    for c in &mut cards {
        c.rekind();
    }
    cards.sort_by(|c1, c2| cmp(c1, c2, "J23456789TQKA"));
    let res2: usize = cards.iter().enumerate().map(|(i, c)| (i + 1) * c.bid).sum();
    dbg!(res2);
}
