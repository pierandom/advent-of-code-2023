fn main() {
    let input = include_str!("input-1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let (times, distances) = parse_input(input);
    times.iter().zip(distances.iter()).map(|(t, d)| {
        if let Some((a, b)) = solve_equation(*t, *d) {
            b - a + 1
        } else {
            0
        }
    }).product()
}


fn parse_input(input: &str) -> (Vec<usize>, Vec<usize>) {
    let mut it = input.lines();
    let times: Vec<usize> = parse_vec(it.next().unwrap(), "Time:");
    let distances: Vec<usize> = parse_vec(it.next().unwrap(), "Distance:");
    (times, distances)
}

fn parse_vec(input: &str, prefix: &str) -> Vec<usize> {
    input
        .trim()
        .strip_prefix(prefix)
        .unwrap()
        .trim()
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

// y = x**2 - t*x + r

fn solve_equation(t: usize, d: usize) -> Option<(usize, usize)> {
    let det = (t*t - 4*d) as f32;
    if det < 0.0 {
        return None;
    } else if det == 0.0 {
        return Some((t/2, t/2));
    } else {
        let a = (t as f32 - f32::sqrt(det))/2.0;
        let b = (t as f32 + f32::sqrt(det))/2.0;
        let a = if a.ceil() == a {
            a.ceil() as usize + 1
        } else {
            a.ceil() as usize
        };
        let b = if b.floor() == b {
            b.floor() as usize - 1
        } else {
            b.floor() as usize
        };
        Some((a, b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "Time:      7  15   30
            Distance:  9  40  200";
        let output = solve(input);
        assert_eq!(output, 288);
    }
}
