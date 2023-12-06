use std::collections::HashMap;

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
    _ = it.next();
    let turns = it.next().unwrap().split(';');
    let mut turn_maps = vec![];
    for turn in turns {
        let mut color_map = HashMap::new();
        for (color, number) in turn.split(',').map(parse_color_number) {
            color_map.insert(color, number);
        }
        turn_maps.push(color_map);
    }
    let min_red = turn_maps.iter().filter_map(|m| m.get("red")).max().unwrap();
    let min_green = turn_maps
        .iter()
        .filter_map(|m| m.get("green"))
        .max()
        .unwrap();
    let min_blue = turn_maps
        .iter()
        .filter_map(|m| m.get("blue"))
        .max()
        .unwrap();
    dbg!(min_red, min_green, min_blue);
    min_red * min_green * min_blue
}

fn parse_color_number(color_number: &str) -> (&str, u32) {
    let mut it = color_number.trim().split(' ');
    let number = it.next().and_then(|s| s.parse::<u32>().ok()).unwrap();
    let color = it.next().unwrap();
    (color, number)
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
        assert_eq!(output, 2286);
    }
}
