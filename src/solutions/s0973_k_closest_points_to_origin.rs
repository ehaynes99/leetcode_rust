pub struct Solution;

impl Solution {
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let mut points = points;

        points.sort_by(|a, b| {
            Self::distance(a).partial_cmp(&Self::distance(b)).unwrap()
        });

        points.truncate(k as usize);

        points
    }

    fn distance(point: &Vec<i32>) -> f32 {
        let sum_of_squares = point[0].pow(2) + point[1].pow(2);
        (sum_of_squares.abs() as f32).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let expected = vec![vec![-2, 2]];
        assert_eq!(expected, Solution::k_closest(points, k))
    }

    #[test]
    fn test2() {
        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;
        let expected = vec![vec![3, 3], vec![-2, 4]];
        assert_eq!(expected, Solution::k_closest(points, k))
    }
}
