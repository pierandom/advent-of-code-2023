use std::collections::HashSet;

fn main() {
    let input = include_str!("input-1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> u32 {
    let (_, numbers) = line.split_once(": ").unwrap();
    let (winning_numbers, my_numbers) = numbers.split_once(" | ").unwrap();
    // empty strings come up when you split on whitespace because 
    //of single digit numbers -> filter_map instead of filter
    let winning_numbers: HashSet<u32> = winning_numbers
        .split(' ')
        .filter_map(|n| n.parse().ok())
        .collect();
    let my_numbers: HashSet<u32> = my_numbers
        .split(' ')
        .filter_map(|n| n.parse().ok())
        .collect();
    let count = my_numbers.intersection(&winning_numbers).count() as u32;
    if count == 0 {
        0
    } else {
        (2 as u32).pow(count - 1)
    }
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
        assert_eq!(output, 13);
    }
}
