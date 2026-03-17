// Given a 0-indexed n x n integer matrix grid, return the number of pairs (ri, cj) such that row ri and column cj are equal.

// A row and column pair is considered equal if they contain the same elements in the same order (i.e., an equal array).
use std::collections::HashMap;

pub fn equal_row_and_column_pairs(grid: &Vec<Vec<i64>>) -> usize {
    let n = grid.len();
    let mut row_counts = HashMap::new();

    for row in grid {
        *row_counts.entry(row).or_insert(0) += 1;
    }

    let mut count = 0;

    for c in 0..n {
        for (row_vec, &occurences) in &row_counts {
            let mut is_equal = true;
            for r in 0..n {
                if grid[r][c] != row_vec[r] {
                    is_equal = false;
                    break;
                }
            }
            if is_equal {
                count += occurences;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::equal_row_and_column_pairs;

    #[test]
    fn test_fequal_row_and_column_pairs(){
        let grid = vec![vec![3,2,1],vec![1,7,6],vec![2,7,7]];
        let result = equal_row_and_column_pairs(&grid);
        assert_eq!(result, 1);
    }
}