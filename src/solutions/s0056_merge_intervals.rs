pub struct Solution;

impl Solution {
    pub fn merge(mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        intervals.sort_by(|a, b| b[0].cmp(&a[0]));

        let mut result = Vec::with_capacity(intervals.len());
        result.push(intervals.pop().unwrap());

        while !intervals.is_empty() {
            let interval = intervals.pop().unwrap();

            let last = result.last_mut().unwrap();

            if interval[0] <= last[1] {
                if interval[1] > last[1] {
                    last[1] = interval[1];
                }
            } else {
                result.push(interval);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let intervals = vec![
            vec![1, 3], //
            vec![2, 6],
            vec![8, 10],
            vec![15, 18],
        ];
        let expected = vec![
            vec![1, 6], //
            vec![8, 10],
            vec![15, 18],
        ];
        assert_eq!(expected, Solution::merge(intervals));
    }

    #[test]
    fn test2() {
        let intervals = vec![
            vec![1, 4], //
            vec![4, 5],
        ];
        let expected = vec![vec![1, 5]];
        assert_eq!(expected, Solution::merge(intervals));
    }

    #[test]
    fn test3() {
        let intervals = vec![
            vec![1, 4], //
            vec![0, 4],
        ];
        let expected = vec![vec![0, 4]];
        assert_eq!(expected, Solution::merge(intervals));
    }
}
