use std::{
    collections::{HashMap, HashSet},
    ops::AddAssign,
};

fn main() {
    let input = include_str!("input-1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    input
        .lines()
        .map(count_matches)
        .enumerate()
        .fold(HashMap::new(), |mut acc, (i, count)| {
            acc.entry(i).or_insert(0).add_assign(1);
            // scratchcards number i available.
            let v = *acc.get(&i).unwrap();

            for j in i + 1..i + 1 + (count as usize) {
                acc.entry(j).or_insert(0).add_assign(v);
            }
            acc
        })
        .values()
        .sum()
}

fn count_matches(line: &str) -> u32 {
    let (_, numbers) = line.split_once(": ").unwrap();
    let (winning_numbers, my_numbers) = numbers.trim().split_once(" | ").unwrap();
    let winning_numbers: HashSet<u32> = winning_numbers
        .split(' ')
        .filter_map(|n| n.trim().parse().ok())
        .collect();
    let my_numbers: HashSet<u32> = my_numbers
        .split(' ')
        .filter_map(|n| n.trim().parse().ok())
        .collect();
    let count = my_numbers.intersection(&winning_numbers).count() as u32;
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
        Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
        Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
        Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
        Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
        Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let output = solve(input);
        assert_eq!(output, 30);
    }
}
