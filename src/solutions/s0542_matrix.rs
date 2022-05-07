use std::collections::VecDeque;

pub struct Solution;

type Cell = (usize, usize);

impl Solution {
    pub fn update_matrix(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let height = matrix.len();
        let width = matrix[0].len();
        let mut matrix = matrix;

        let get_peers = |cell: Cell| {
            let (x, y) = cell;

            let mut peers = Vec::new();
            if x > 0 {
                peers.push((x - 1, y))
            }
            if x + 1 < width {
                peers.push((x + 1, y));
            }
            if y > 0 {
                peers.push((x, y - 1));
            }
            if y + 1 < height {
                peers.push((x, y + 1));
            }

            peers
        };

        let mut cells = VecDeque::new();

        for y in 0..height {
            for x in 0..width {
                if matrix[y][x] == 0 {
                    cells.push_back((x, y));
                } else {
                    matrix[y][x] = i32::MAX;
                }
            }
        }

        while let Some(cell) = cells.pop_front() {
            let (x, y) = cell;
            let distance = matrix[y][x] + 1;
            for peer in get_peers(cell) {
                let (x, y) = peer;
                if matrix[y][x] == i32::MAX {
                    matrix[y][x] = distance;
                    cells.push_back(peer);
                }
            }
        }

        matrix
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmp() {
        let matrix = vec![
            vec![0, 1], //
            vec![1, 1],
        ];
        assert_eq!(
            Solution::update_matrix(matrix),
            vec![
                vec![0, 1], //
                vec![1, 2],
            ]
        )
    }

    #[test]
    fn test1() {
        let matrix = vec![
            vec![0, 0, 0], //
            vec![0, 1, 0],
            vec![0, 0, 0],
        ];
        assert_eq!(
            Solution::update_matrix(matrix),
            vec![
                vec![0, 0, 0], //
                vec![0, 1, 0],
                vec![0, 0, 0],
            ]
        )
    }

    #[test]
    fn test2() {
        let matrix = vec![
            vec![0, 0, 0], //
            vec![0, 1, 0],
            vec![1, 1, 1],
        ];
        assert_eq!(
            Solution::update_matrix(matrix),
            vec![
                vec![0, 0, 0], //
                vec![0, 1, 0],
                vec![1, 2, 1],
            ]
        );
    }

    #[test]
    fn test3() {
        let matrix = vec![
            vec![0, 0, 0, 0], //
            vec![0, 1, 0, 1],
            vec![1, 1, 1, 1],
            vec![1, 1, 1, 1],
        ];
        assert_eq!(
            Solution::update_matrix(matrix),
            vec![
                vec![0, 0, 0, 0], //
                vec![0, 1, 0, 1],
                vec![1, 2, 1, 2],
                vec![2, 3, 2, 3],
            ]
        );
    }

    #[test]
    fn test4() {
        let matrix = vec![
            vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 0],
            vec![1, 1, 1, 0, 0, 0, 1, 0, 0, 1],
            vec![0, 1, 1, 1, 0, 1, 1, 0, 1, 1],
            vec![1, 1, 1, 1, 0, 1, 1, 1, 1, 0],
            vec![1, 0, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 0, 0, 1, 0, 1, 1],
            vec![1, 1, 0, 1, 0, 0, 1, 1, 1, 1],
            vec![1, 1, 0, 1, 0, 0, 1, 0, 0, 0],
            vec![0, 0, 1, 0, 1, 0, 1, 1, 1, 0],
            vec![1, 1, 1, 1, 0, 1, 1, 0, 1, 1],
        ];
        // let tmp = vec![
        //     vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 0],
        //     vec![1, 2, 1, 0, 0, 0, 1, 0, 0, 1],
        //     vec![0, 1, 2, 1, 0, 1, 1, 0, 1, 1],
        //     vec![1, 1, 2, 1, 0, 1, 2, 1, 1, 0],
        //     vec![1, 0, 1, 2, 1, 1, 2, 1, 2, 1],
        //     vec![3, 1, 1, 1, 0, 0, 1, 0, 1, 2],
        //     vec![2, 1, 0, 1, 0, 0, 1, 1, 1, 1],
        //     vec![1, 1, 0, 1, 0, 0, 1, 0, 0, 0],
        //     vec![0, 0, 1, 0, 1, 0, 1, 1, 1, 0],
        //     vec![1, 1, 2, 1, 0, 1, 1, 0, 1, 1],
        // ];
        // let expected = vec![
        //     vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 0],
        //     vec![1, 2, 1, 0, 0, 0, 1, 0, 0, 1],
        //     vec![0, 1, 2, 1, 0, 1, 1, 0, 1, 1],
        //     vec![1, 1, 2, 1, 0, 1, 2, 1, 1, 0],
        //     vec![1, 0, 1, 2, 1, 1, 2, 1, 2, 1],
        //     vec![2, 1, 1, 1, 0, 0, 1, 0, 1, 2],
        //     vec![2, 1, 0, 1, 0, 0, 1, 1, 1, 1],
        //     vec![1, 1, 0, 1, 0, 0, 1, 0, 0, 0],
        //     vec![0, 0, 1, 0, 1, 0, 1, 1, 1, 0],
        //     vec![1, 1, 2, 1, 0, 1, 1, 0, 1, 1]
        // ];
        assert_eq!(
            Solution::update_matrix(matrix),
            vec![
                vec![0, 1, 1, 0, 1, 0, 1, 0, 1, 0],
                vec![1, 2, 1, 0, 0, 0, 1, 0, 0, 1],
                vec![0, 1, 2, 1, 0, 1, 1, 0, 1, 1],
                vec![1, 1, 2, 1, 0, 1, 2, 1, 1, 0],
                vec![1, 0, 1, 2, 1, 1, 2, 1, 2, 1],
                vec![2, 1, 1, 1, 0, 0, 1, 0, 1, 2],
                vec![2, 1, 0, 1, 0, 0, 1, 1, 1, 1],
                vec![1, 1, 0, 1, 0, 0, 1, 0, 0, 0],
                vec![0, 0, 1, 0, 1, 0, 1, 1, 1, 0],
                vec![1, 1, 2, 1, 0, 1, 1, 0, 1, 1]
            ]
        );
    }

    /*

        Input
    [[0,1,1,0,1,0,1,0,1,0],[1,1,1,0,0,0,1,0,0,1],[0,1,1,1,0,1,1,0,1,1],[1,1,1,1,0,1,1,1,1,0],[1,0,1,1,1,1,1,1,1,1],[1,1,1,1,0,0,1,0,1,1],[1,1,0,1,0,0,1,1,1,1],[1,1,0,1,0,0,1,0,0,0],[0,0,1,0,1,0,1,1,1,0],[1,1,1,1,0,1,1,0,1,1]]
    Output
    [[0,1,1,0,1,0,1,0,1,0],[1,2,1,0,0,0,1,0,0,1],[0,1,2,1,0,1,1,0,1,1],[1,1,2,1,0,1,2,1,1,0],[1,0,1,2,1,1,2,1,2,1],[3,1,1,1,0,0,1,0,1,2],[2,1,0,1,0,0,1,1,1,1],[1,1,0,1,0,0,1,0,0,0],[0,0,1,0,1,0,1,1,1,0],[1,1,2,1,0,1,1,0,1,1]]
    Expected
    [[0,1,1,0,1,0,1,0,1,0],[1,2,1,0,0,0,1,0,0,1],[0,1,2,1,0,1,1,0,1,1],[1,1,2,1,0,1,2,1,1,0],[1,0,1,2,1,1,2,1,2,1],[2,1,1,1,0,0,1,0,1,2],[2,1,0,1,0,0,1,1,1,1],[1,1,0,1,0,0,1,0,0,0],[0,0,1,0,1,0,1,1,1,0],[1,1,2,1,0,1,1,0,1,1]]
    */
}
