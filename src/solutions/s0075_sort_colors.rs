pub struct Solution;

impl Solution {
    pub fn sort_colors(nums: &mut [i32]) {
        // nums all in range 0..3
        let mut counts = [0; 3];
        nums.iter().for_each(|num| {
            counts[*num as usize] += 1;
        });
        let mut curr = 0usize;
        (0..counts.len()).for_each(|num| {
            while counts[num] > 0 {
                nums[curr] = num as i32;
                curr += 1;
                counts[num] -= 1;
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut nums = vec![2, 0, 2, 1, 1, 0];
        let expected = vec![0, 0, 1, 1, 2, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(expected, nums);
    }

    #[test]
    fn test2() {
        let mut nums = vec![2, 0, 1];
        let expected = vec![0, 1, 2];
        Solution::sort_colors(&mut nums);
        assert_eq!(expected, nums);
    }
}
