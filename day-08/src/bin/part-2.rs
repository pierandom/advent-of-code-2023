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
    let mut states = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .collect::<Vec<_>>();
    for instruction in instructions.chars().cycle() {
        steps += 1;
        if instruction == 'L' {
            for state in states.iter_mut() {
                let (left, _) = map.get(&state.clone()).unwrap();
                *state = left;
            }
        } else {
            for state in states.iter_mut() {
                let (_, right) = map.get(&state.clone()).unwrap();
                *state = right;
            }
        }
        if states.iter().all(|state| state.ends_with('Z')) {
            break;
        }
    }
    steps
}

fn parse_map(input: &str) -> HashMap<String, (String, String)> {
    input
        .lines()
        .map(parse_line)
        .fold(HashMap::new(), |mut map, (key, value)| {
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
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        let output = solve(input);
        assert_eq!(output, 6);
    }
}
