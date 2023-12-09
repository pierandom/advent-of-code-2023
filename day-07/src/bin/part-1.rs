use std::{collections::HashMap, ops::AddAssign, str::FromStr};

fn main() {
    let input = include_str!("input-1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let mut plays: Vec<Play> = input
        .lines()
        .map(|line| line.parse::<Play>().unwrap())
        .collect();
    plays.sort();
    plays
        .iter()
        .enumerate()
        .map(|(i, play)| (i + 1) * play.bid)
        .sum()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Copy, Clone, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
            "J" => Ok(Card::Jack),
            "Q" => Ok(Card::Queen),
            "K" => Ok(Card::King),
            "A" => Ok(Card::Ace),
            _ => panic!("Unknown card"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug)]
struct Play {
    hand: Vec<Card>,
    bid: usize,
}

impl Play {
    fn get_hand_type(&self) -> HandType {
        let counter = self
            .hand
            .iter()
            .fold(HashMap::<&Card, u8>::new(), |mut acc, card| {
                acc.entry(card).or_insert(0).add_assign(1);
                acc
            });
        let mut most_common: Vec<_> = counter.values().cloned().collect();
        most_common.sort_by(|a, b| b.cmp(a));
        if most_common[0] == 5 {
            HandType::FiveOfAKind
        } else if most_common[0] == 4 {
            HandType::FourOfAKind
        } else if most_common[0] == 3 && most_common[1] == 2 {
            HandType::FullHouse
        } else if most_common[0] == 3 {
            HandType::ThreeOfAKind
        } else if most_common[0] == 2 && most_common[1] == 2 {
            HandType::TwoPair
        } else if most_common[0] == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }
}

impl FromStr for Play {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand, bid) = s.split_once(' ').unwrap();
        Ok(Play {
            hand: hand
                .chars()
                .map(|c| c.to_string().parse::<Card>().unwrap())
                .collect(),
            bid: bid.parse().unwrap(),
        })
    }
}

impl PartialEq for Play {
    fn eq(&self, other: &Self) -> bool {
        self.hand == other.hand
    }
}

impl Eq for Play {}

impl PartialOrd for Play {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.get_hand_type() == other.get_hand_type() {
            Some(Ord::cmp(&self.hand, &other.hand))
        } else {
            Some(Ord::cmp(&self.get_hand_type(), &other.get_hand_type()))
        }
    }
}

impl Ord for Play {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        PartialOrd::partial_cmp(&self, &other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        let output = solve(input);
        assert_eq!(output, 6440);
    }
}
