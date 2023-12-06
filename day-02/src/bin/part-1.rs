const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let input = include_str!("input-1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> u32 {
    input.lines().map(process_line).sum()
}

fn process_line(line: &str) -> u32 {
    let mut it = line.trim().split(':');
    let game_id = it
        .next()
        .and_then(|s| s.strip_prefix("Game "))
        .and_then(|s| s.parse::<u32>().ok()).unwrap();
    let mut turns = it.next().unwrap().split(';');
    if turns.all(is_turn_valid) {
        game_id
    } else {
        0
    }
}

fn is_turn_valid(turn: &str) -> bool {
    turn.split(',').all(is_color_number_valid)
}

fn is_color_number_valid(color_number: &str) -> bool {
    let mut it = color_number.trim().split(' ');
    let number = it.next().and_then(|s| s.parse::<u32>().ok()).unwrap();
    let color = it.next().unwrap();
    match color {
        "red" => number <= MAX_RED,
        "green" => number <= MAX_GREEN,
        "blue" => number <= MAX_BLUE,
        _ => unreachable!("Invalid color: {}", color),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        let output = solve(input);
        assert_eq!(output, 8);
    }
}
