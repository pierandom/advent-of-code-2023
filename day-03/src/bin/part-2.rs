fn main() {
    let input = include_str!("input-1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '*' {
                result += get_gear_number(&grid, i, j);
            }
        }
    }
    result
}

fn option_to_number(option: Option<usize>) -> usize {
    match option {
        Some(_) => 1,
        None => 0,
    }
}

fn get_gear_number(grid: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let left = parse_left(grid, i, j);
    let right = parse_right(grid, i, j);
    let (above_left, above_right) = parse_above(grid, i, j);
    let (below_left, below_right) = parse_below(grid, i, j);
    let num_parts = option_to_number(left)
        + option_to_number(right)
        + option_to_number(above_left)
        + option_to_number(above_right)
        + option_to_number(below_left)
        + option_to_number(below_right);
    if num_parts == 2 {
        left.unwrap_or(1)
            * right.unwrap_or(1)
            * above_left.unwrap_or(1)
            * above_right.unwrap_or(1)
            * below_left.unwrap_or(1)
            * below_right.unwrap_or(1)
    } else {
        0
    }
}

fn parse_left(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Option<usize> {
    let mut k = j;
    while k > 0 && grid[i][k - 1].is_ascii_digit() {
        k -= 1;
    }
    grid[i][k..j].iter().collect::<String>().parse().ok()
}

fn parse_right(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Option<usize> {
    let mut k = j + 1;
    while k < grid[i].len() && grid[i][k].is_ascii_digit() {
        k += 1;
    }
    grid[i][j + 1..k].iter().collect::<String>().parse().ok()
}

fn parse_middle(grid: &Vec<Vec<char>>, i: usize, j: usize) -> Option<usize> {
    let mut h = j;
    while h > 0 && grid[i][h - 1].is_ascii_digit() {
        h -= 1;
    }
    let mut k = j + 1;
    while k < grid[i].len() && grid[i][k].is_ascii_digit() {
        k += 1;
    }
    grid[i][h..k].iter().collect::<String>().parse().ok()
}

fn parse_above(grid: &Vec<Vec<char>>, i: usize, j: usize) -> (Option<usize>, Option<usize>) {
    if i == 0 {
        return (None, None);
    }
    if grid[i - 1][j] == '.' {
        (parse_left(grid, i - 1, j), parse_right(grid, i - 1, j))
    } else {
        (parse_middle(grid, i - 1, j), None)
    }
}

fn parse_below(grid: &Vec<Vec<char>>, i: usize, j: usize) -> (Option<usize>, Option<usize>) {
    if i + 1 == grid.len() {
        return (None, None);
    }
    if grid[i + 1][j] == '.' {
        (parse_left(grid, i + 1, j), parse_right(grid, i + 1, j))
    } else {
        (parse_middle(grid, i + 1, j), None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = "467..114..
            ...*......
            ..35..633.
            ......#...
            617*......
            .....+.58.
            ..592.....
            ......755.
            ...$.*....
            .664.598..";
        let output = solve(input);
        assert_eq!(output, 467835);
    }
}
