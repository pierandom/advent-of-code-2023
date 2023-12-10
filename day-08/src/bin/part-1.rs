use std::collections::HashMap;

fn main() {
    let input = include_str!("input-1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let (instructions, map) = input.split_once("\n\n").unwrap();
    let map = parse_map(map);
    let mut steps = 0;
    let mut state = "AAA".to_string();
    for instruction in instructions.chars().cycle() {
        steps += 1;
        let (left, right) = map.get(&state.to_string()).unwrap();
        if instruction == 'L' {
            state = left.clone();
        } else {
            state = right.clone();
        }
        if state == "ZZZ" {
            break;
        }
    }
    steps
}

fn parse_map(input: &str) -> HashMap<String, (String, String)> {
    input.lines().map(parse_line).fold(HashMap::new(), |mut map, (key, value)| {
        map.insert(key, value);
        map
    })
}

fn parse_line(input: &str) -> (String, (String, String)) {
    let (key, value) = input.split_once(" = ").unwrap();
    let (left, right) = value.split_once(", ").unwrap();
    let left = left.strip_prefix('(').unwrap();
    let right = right.strip_suffix(')').unwrap();
    (key.to_string(), (left.to_string(), right.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        let output = solve(input);
        assert_eq!(output, 2);
    }
}