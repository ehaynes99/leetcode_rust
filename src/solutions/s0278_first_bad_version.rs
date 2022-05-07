pub struct Solution {
    first_bad: i32,
}

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut min = 0;
        let mut max = n;

        while min <= max {
            // (min + max) / 2 overflows
            let mid = min + (max - min) / 2;

            if self.isBadVersion(mid) {
                max = mid - 1;
            } else {
                min = mid + 1;
            }
        }

        min
    }

    // this method is implemented by leetcode with this specific casing
    #[allow(non_snake_case)]
    fn isBadVersion(&self, version: i32) -> bool {
        version >= self.first_bad
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let n = 5;
        let first_bad = 4;
        let solution = Solution { first_bad };
        assert_eq!(first_bad, solution.first_bad_version(n));
    }

    #[test]
    fn test2() {
        let n = 1;
        let first_bad = 1;
        let solution = Solution { first_bad };
        assert_eq!(first_bad, solution.first_bad_version(n));
    }

    #[test]
    fn test3() {
        let n = 2126753390;
        let first_bad = 1702766719;
        let solution = Solution { first_bad };
        assert_eq!(first_bad, solution.first_bad_version(n));
    }
}
