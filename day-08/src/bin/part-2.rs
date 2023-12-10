use std::collections::HashMap;

fn main() {
    let input = include_str!("input-1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let (instructions, map) = input.split_once("\n\n").unwrap();
    let map = parse_map(map);
    let mut states = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();
    let distances = states.iter_mut().map(|state| {
        instructions.chars().cycle().position(|c| {
            if c == 'L' {
                *state = map[state].0.clone();
            } else {
                *state = map[state].1.clone();
            }
            state.ends_with('Z')
        }).unwrap() + 1
    }).collect::<Vec<_>>();
    lcm(&distances)
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

fn lcm(nums: &[usize]) -> usize {
    nums.iter().fold(1, |acc, x| acc * x / gcd(acc, *x))
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
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
