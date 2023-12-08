fn main() {
    let input = include_str!("input-1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let (time, distance) = parse_input(input);
    dbg!(time, distance);
    if let Some((a, b)) = solve_equation(time, distance) {
        dbg!(a, b);
        b - a + 1
    } else {
        0
    }
}

fn parse_input(input: &str) -> (usize, usize) {
    let mut it = input.lines();
    let time: usize = parse_num(it.next().unwrap());
    let distance: usize = parse_num(it.next().unwrap());
    (time, distance)
}

fn parse_num(input: &str) -> usize {
    input
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}

// y = x**2 - t*x + r

fn solve_equation(t: usize, d: usize) -> Option<(usize, usize)> {
    let det = (t * t - 4 * d) as f64;
    if det < 0.0 {
        return None;
    } else if det == 0.0 {
        return Some((t / 2, t / 2));
    } else {
        let a = (t as f64 - f64::sqrt(det)) / 2.0;
        let b = (t as f64 + f64::sqrt(det)) / 2.0;
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
        assert_eq!(output, 71503);
    }
}
