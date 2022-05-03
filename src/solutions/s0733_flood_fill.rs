use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn flood_fill(
        image: Vec<Vec<i32>>,
        row: i32,
        column: i32,
        new_color: i32,
    ) -> Vec<Vec<i32>> {
        let mut image = image;
        let start_cell = (column as usize, row as usize);
        let original_color = image[start_cell.1][start_cell.0];
        let width = image[0].len();
        let height = image.len();

        let mut visited = HashSet::new();
        visited.insert(start_cell);
        let mut cells = vec![start_cell];

        while !cells.is_empty() {
            let cell = cells.pop().unwrap();
            let x = cell.0;
            let y = cell.1;

            image[y][x] = new_color;

            let peers = {
                let mut result = Vec::new();
                if x > 0 {
                    result.push((x - 1, y));
                }
                if x + 1 < width {
                    result.push((x + 1, y));
                }
                if y > 0 {
                    result.push((x, y - 1));
                }
                if y + 1 < height {
                    result.push((x, y + 1));
                }

                result
            };

            for peer in peers {
                let peer_color = image[peer.1][peer.0];
                if peer_color == original_color && visited.insert(peer) {
                    cells.push(peer);
                }
            }
        }
        image
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let image = vec![
            vec![1, 1, 1], //
            vec![1, 1, 0],
            vec![1, 0, 1],
        ];
        let result = Solution::flood_fill(image, 1, 1, 2);
        let expected = vec![
            vec![2, 2, 2], //
            vec![2, 2, 0],
            vec![2, 0, 1],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test2() {
        let image = vec![
            vec![0, 0, 0], //
            vec![0, 0, 0],
        ];
        let result = Solution::flood_fill(image, 0, 0, 2);
        let expected = vec![
            vec![2, 2, 2], //
            vec![2, 2, 2],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test3() {
        let image = vec![
            vec![0,0,0], //
            vec![1,0,0],
        ];
        let result = Solution::flood_fill(image, 1, 0, 2);
        let expected = vec![
            vec![0,0,0], //
            vec![2,0,0]
        ];
        assert_eq!(result, expected);
    }
}
