fn main() {
    let input = include_str!("input-1.txt");
    let output = solve(input);
    dbg!(output);
}

fn solve(input: &str) -> usize {
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();
    let mut result = 0;
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if !grid[i][j].is_ascii_digit() && grid[i][j] != '.' {
                result += sum_neighbor_numbers(&mut grid, i, j);
            }
        }
    }
    result
}

fn sum_neighbor_numbers(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut result = 0;
    result += sum_left(grid, i, j);
    result += sum_right(grid, i, j);
    result += sum_above(grid, i, j);
    result += sum_below(grid, i, j);
    result
}

fn sum_left(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut k = j;
    while k > 0 && grid[i][k - 1].is_ascii_digit() {
        k -= 1;
    }
    let result = grid[i][k..j]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap_or_default();
    for h in k..j {
        grid[i][h] = '.';
    }
    result
}

fn sum_right(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut k = j + 1;
    while k < grid[i].len() && grid[i][k].is_ascii_digit() {
        k += 1;
    }
    let result = grid[i][j + 1..k]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap_or_default();
    for h in j + 1..k {
        grid[i][h] = '.';
    }
    result
}

fn sum_through(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let mut h = j;
    while h > 0 && grid[i][h - 1].is_ascii_digit() {
        h -= 1;
    }
    let mut k = j + 1;
    while k < grid[i].len() && grid[i][k].is_ascii_digit() {
        k += 1;
    }
    let result = grid[i][h..k]
        .iter()
        .collect::<String>()
        .parse()
        .unwrap_or_default();
    for m in h..k {
        grid[i][m] = '.';
    }
    result
}

fn sum_above(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> usize {
    if i == 0 {
        return 0;
    }
    let mut result = 0;
    if grid[i - 1][j] == '.' {
        result += sum_left(grid, i - 1, j);
        result += sum_right(grid, i - 1, j);
    } else {
        result += sum_through(grid, i - 1, j);
    }
    result
}

fn sum_below(grid: &mut Vec<Vec<char>>, i: usize, j: usize) -> usize {
    if i + 1 == grid.len() {
        return 0;
    }
    let mut result = 0;
    if grid[i + 1][j] == '.' {
        result += sum_left(grid, i + 1, j);
        result += sum_right(grid, i + 1, j);
    } else {
        result += sum_through(grid, i + 1, j);
    }
    result
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
        assert_eq!(output, 4361);
    }
}
