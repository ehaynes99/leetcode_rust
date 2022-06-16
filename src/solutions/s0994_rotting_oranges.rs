pub struct Solution;

const EMPTY: i32 = 0;
const FRESH: i32 = 1;
const ROTTEN: i32 = 2;

impl Solution {
    pub fn oranges_rotting(grid: Vec<Vec<i32>>) -> i32 {
        let width = grid[0].len();

        let mut grid = grid.into_iter().flatten().collect::<Vec<_>>();
        let adjacencies = Self::get_adjacent(&grid, width);
        let mut fresh_count = grid.iter().filter(|value| **value == FRESH).count();

        let mut cycles = 0;

        while fresh_count > 0 {
            cycles += 1;
            let mut new_rotten = Vec::new();
            for index in 0..grid.len() {
                if grid[index] == FRESH && adjacencies[index].iter().any(|i| grid[*i] == ROTTEN) {
                    new_rotten.push(index);
                    fresh_count -= 1;
                }
            }
            if new_rotten.is_empty() {
                return -1;
            }
            for i in new_rotten {
                grid[i] = ROTTEN;
            }
        }

        cycles
    }

    fn get_adjacent(grid: &[i32], width: usize) -> Vec<Vec<usize>> {
        let mut result = Vec::new();

        for index in 0..grid.len() {
            let mut adjacent = Vec::new();

            if grid[index] == FRESH {
                let x = index % width;
                let y = index / width;

                if x > 0 {
                    adjacent.push(index - 1);
                }
                if y > 0 {
                    adjacent.push(index - width);
                }
                if x < width - 1 {
                    adjacent.push(index + 1);
                }
                if index + width < grid.len() {
                    adjacent.push(index + width);
                }
            }

            result.push(adjacent.into_iter().filter(|i| grid[*i] != EMPTY).collect());
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_grid(values: &[&[i32]]) -> Vec<Vec<i32>> {
        values.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    pub fn test1() {
        let grid = to_grid(&[
            &[2, 1, 1], //
            &[1, 1, 0],
            &[0, 1, 1],
        ]);
        assert_eq!(4, Solution::oranges_rotting(grid));
    }

    #[test]
    pub fn test2() {
        let grid = to_grid(&[
            &[2, 1, 1], //
            &[0, 1, 1],
            &[1, 0, 1],
        ]);
        assert_eq!(-1, Solution::oranges_rotting(grid));
    }

    #[test]
    pub fn test3() {
        let grid = to_grid(&[
            &[0, 2], //
        ]);
        assert_eq!(0, Solution::oranges_rotting(grid));
    }
}
