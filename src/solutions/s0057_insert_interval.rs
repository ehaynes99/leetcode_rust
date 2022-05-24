pub struct Solution;

impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut new_interval = new_interval;


        let mut insert_index = 0;
        for interval in intervals.iter() {
            if interval[1] < new_interval[0] {
                insert_index += 1;
            } else {
                break;
            }
        }

        let mut tail_index = insert_index;
        for interval in &intervals[insert_index..] {
            if interval[0] < new_interval[0] {
                new_interval[0] = interval[0];
            }
            if interval[0] <= new_interval[1] {
                if interval[1] >= new_interval[1] {
                    new_interval[1] = interval[1];
                }
            } else {
                break;
            }
            tail_index += 1;
        }

        let mut intervals = intervals;
        intervals.splice(insert_index..tail_index, [new_interval]);

        return intervals;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let intervals = vec![vec![1, 3], vec![6, 9]];
        let new_interval = vec![2, 5];
        let expected = vec![vec![1, 5], vec![6, 9]];
        assert_eq!(expected, Solution::insert(intervals, new_interval));
    }

    #[test]
    fn test2() {
        let intervals = vec![
            vec![1, 2],
            vec![3, 5],
            vec![6, 7],
            vec![8, 10],
            vec![12, 16],
        ];
        let new_interval = vec![4, 8];
        let expected = vec![vec![1, 2], vec![3, 10], vec![12, 16]];
        assert_eq!(expected, Solution::insert(intervals, new_interval));
    }

    #[test]
    fn test3() {
        let intervals = vec![vec![3, 4], vec![5, 6]];
        let new_interval = vec![1, 2];
        let expected = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(expected, Solution::insert(intervals, new_interval));
    }

    #[test]
    fn test4() {
        let intervals = vec![vec![1, 2], vec![5, 6]];
        let new_interval = vec![3, 4];
        let expected = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(expected, Solution::insert(intervals, new_interval));
    }

    #[test]
    fn test5() {
        let intervals = vec![vec![1, 2], vec![3, 4]];
        let new_interval = vec![5, 6];
        let expected = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        assert_eq!(expected, Solution::insert(intervals, new_interval));
    }

    #[test]
    fn test6() {
        let intervals = vec![vec![1, 3], vec![4, 6], vec![7, 9]];
        let new_interval = vec![2, 8];
        let expected = vec![vec![1, 9]];
        assert_eq!(expected, Solution::insert(intervals, new_interval));
    }

    #[test]
    fn test7() {
        let intervals = vec![vec![1, 5]];
        let new_interval = vec![2, 3];
        let expected = vec![vec![1, 5]];
        assert_eq!(expected, Solution::insert(intervals, new_interval));
    }
}
