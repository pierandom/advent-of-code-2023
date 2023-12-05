fn main() {
    let input = include_str!("input-2.txt");
    let output = solve(input);
    dbg!(output);
}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|i| {
        let reduced_line = &line[i..];
        if reduced_line.starts_with("one") {
            Some(1)
        } else if reduced_line.starts_with("two") {
            Some(2)
        } else if reduced_line.starts_with("three") {
            Some(3)
        } else if reduced_line.starts_with("four") {
            Some(4)
        } else if reduced_line.starts_with("five") {
            Some(5)
        } else if reduced_line.starts_with("six") {
            Some(6)
        } else if reduced_line.starts_with("seven") {
            Some(7)
        } else if reduced_line.starts_with("eight") {
            Some(8)
        } else if reduced_line.starts_with("nine") {
            Some(9)
        } else {
            reduced_line.chars().next().and_then(|c| c.to_digit(10))
        }
    });
    let first_digit = it.next().unwrap();
    10 * first_digit + it.last().unwrap_or(first_digit)
}

fn solve(input: &str) -> u32 {
    input.lines().map(process_line).sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen";
        let output = solve(input);
        assert_eq!(output, 281);
    }
}
