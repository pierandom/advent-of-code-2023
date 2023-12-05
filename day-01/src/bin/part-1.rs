fn main() {
    let input = include_str!("input-1.txt");
    let output = solve(input);
    dbg!(output);
}

fn process_line(line: &str) -> u32 {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first_digit = digits.next().unwrap();
    10 * first_digit + digits.last().unwrap_or(first_digit)
}

fn solve(input: &str) -> u32 {
    input.lines().map(process_line).sum::<u32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet";
        let output = solve(input);
        assert_eq!(output, 142);
    }
}
