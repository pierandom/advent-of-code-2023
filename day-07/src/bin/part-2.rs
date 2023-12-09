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
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl FromStr for Card {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "J" => Ok(Card::Joker),
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eight),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::Ten),
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
                if card != &Card::Joker {
                    acc.entry(card).or_insert(0).add_assign(1);
                }
                acc
            });
        let mut most_common: Vec<_> = counter.values().cloned().collect();
        most_common.sort_by(|a, b| b.cmp(a));
        let joker_count = self.count_jokers() as u8;
        let first_most_common = most_common.get(0).unwrap_or(&0);
        let second_most_common = most_common.get(1).unwrap_or(&0);
        if first_most_common + joker_count == 5 {
            HandType::FiveOfAKind
        } else if first_most_common + joker_count == 4 {
            HandType::FourOfAKind
        } else if first_most_common + second_most_common + joker_count == 5 {
            HandType::FullHouse
        } else if first_most_common + joker_count == 3 {
            HandType::ThreeOfAKind
        } else if first_most_common + second_most_common + joker_count == 4 {
            HandType::TwoPair
        } else if first_most_common + joker_count == 2 {
            HandType::OnePair
        } else {
            HandType::HighCard
        }
    }

    fn count_jokers(&self) -> usize {
        self.hand.iter().filter(|&&c| c == Card::Joker).count()
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
        assert_eq!(output, 5905);

        let input = include_str!("input-1.txt");
        let output = solve(input);
        assert_eq!(output, 251824095);
    }
}
