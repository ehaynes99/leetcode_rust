pub struct Solution;

impl Solution {
    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        Self::permute_slice(nums, 0, &mut result);
        result
    }

    fn permute_slice(nums: Vec<i32>, index: usize, result: &mut Vec<Vec<i32>>) {
        if nums.len() - index == 1 {
            result.push(nums);
        } else {
            for alt_index in index..nums.len() {
                let mut vec = nums.to_vec();
                vec.swap(alt_index, index);
                Self::permute_slice(vec, index + 1, result);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![1, 2, 3];
        let expected = vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 2, 1],
            vec![3, 1, 2],
        ];
        assert_eq!(expected, Solution::permute(nums));
    }

    #[test]
    fn test2() {
        let nums = vec![0, 1];
        let expected = vec![vec![0, 1], vec![1, 0]];
        assert_eq!(expected, Solution::permute(nums));
    }

    #[test]
    fn test3() {
        let nums = vec![1];
        let expected = vec![vec![1]];
        assert_eq!(expected, Solution::permute(nums));
    }
}
