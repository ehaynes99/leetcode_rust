pub struct Solution;

type Cell = (usize, usize);

struct Grid<T> {
    cells: Vec<Vec<T>>,
    width: usize,
    height: usize,
}

impl<T> Grid<T> {
    fn new(cells: Vec<Vec<T>>) -> Self {
        let height = cells.len();
        let width = cells[0].len();
        Self {
            cells,
            width,
            height,
        }
    }

    fn get(&self, cell: Cell) -> &T {
        let (x, y) = cell;
        &self.cells[y][x]
    }

    fn neighbors(&self, cell: Cell) -> Vec<Cell> {
        let (x, y) = cell;

        let mut result = Vec::new();
        if x > 0 {
            result.push((x - 1, y));
        };
        if x < self.width - 1 {
            result.push((x + 1, y));
        };
        if y > 0 {
            result.push((x, y - 1));
        };
        if y < self.height - 1 {
            result.push((x, y + 1));
        };

        result
    }
}

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let grid = Grid::new(grid);

        let mut visited = vec![vec![false; grid.width]; grid.height];
        let mut num_islands = 0;

        (0..grid.height).for_each(|y| {
            (0..grid.width).for_each(|x| {
                let cell = (x, y);
                if !visited[y][x] && grid.get(cell) == &'1' {
                    visited[y][x] = true;
                    num_islands += 1;
                    let mut stack = vec![cell];

                    while !stack.is_empty() {
                        let cell = stack.pop().unwrap();
                        for neighbor in grid.neighbors(cell) {
                            let (x, y) = neighbor;
                            if !visited[y][x] && grid.get(neighbor) == &'1' {
                                visited[y][x] = true;
                                stack.push(neighbor);
                            }
                        }
                    }
                }
            });
        });

        num_islands
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn to_grid(values: &[&[char]]) -> Vec<Vec<char>> {
        values.iter().map(|row| row.to_vec()).collect()
    }

    #[test]
    fn test1() {
        let grid = to_grid(&[
            &['1', '1', '1', '1', '0'],
            &['1', '1', '0', '1', '0'],
            &['1', '1', '0', '0', '0'],
            &['0', '0', '0', '0', '0'],
        ]);
        assert_eq!(1, Solution::num_islands(grid));
    }

    #[test]
    fn test2() {
        let grid = to_grid(&[
            &['1', '1', '0', '0', '0'],
            &['1', '1', '0', '0', '0'],
            &['0', '0', '1', '0', '0'],
            &['0', '0', '0', '1', '1'],
        ]);
        assert_eq!(3, Solution::num_islands(grid));
    }

    #[test]
    fn test3() {
        let grid = to_grid(&[
            &['1', '0', '1', '0', '1'],
            &['0', '1', '0', '1', '0'],
            &['1', '0', '1', '0', '1'],
            &['0', '1', '0', '1', '0'],
            &['1', '0', '1', '0', '1'],
        ]);
        assert_eq!(13, Solution::num_islands(grid));
    }

    #[test]
    fn test4() {
        let grid = to_grid(&[
            &['1', '1', '1', '1', '1', '0', '1', '1', '1', '1'],
            &['1', '0', '1', '0', '1', '1', '1', '1', '1', '1'],
            &['0', '1', '1', '1', '0', '1', '1', '1', '1', '1'],
            &['1', '1', '0', '1', '1', '0', '0', '0', '0', '1'],
            &['1', '0', '1', '0', '1', '0', '0', '1', '0', '1'],
            &['1', '0', '0', '1', '1', '1', '0', '1', '0', '0'],
            &['0', '0', '1', '0', '0', '1', '1', '1', '1', '0'],
            &['1', '0', '1', '1', '1', '0', '0', '1', '1', '1'],
            &['1', '1', '1', '1', '1', '1', '1', '1', '0', '1'],
            &['1', '0', '1', '1', '1', '1', '1', '1', '1', '0'],
        ]);
        assert_eq!(2, Solution::num_islands(grid));
    }

    #[test]
    fn test5() {
        let grid = to_grid(&[
            &['1', '1', '1', '1', '1', '0', '1', '1', '1', '1'],
            &['0', '1', '1', '0', '1', '1', '1', '0', '1', '1'],
            &['1', '0', '1', '0', '1', '1', '0', '1', '0', '1'],
            &['1', '0', '1', '1', '0', '1', '1', '1', '1', '1'],
            &['1', '1', '0', '0', '1', '1', '1', '1', '1', '1'],
            &['1', '1', '0', '1', '1', '1', '1', '1', '1', '1'],
            &['1', '1', '1', '1', '1', '1', '1', '1', '0', '1'],
            &['0', '1', '1', '0', '1', '1', '1', '1', '1', '0'],
            &['1', '1', '0', '1', '1', '0', '1', '1', '1', '1'],
            &['0', '1', '1', '1', '1', '1', '0', '1', '1', '1'],
        ]);
        assert_eq!(1, Solution::num_islands(grid));
    }
}
