pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let num_courses = num_courses as usize;
        let reqs = {
            let mut reqs = vec![vec![]; num_courses as usize];
            for edge in prerequisites {
                reqs[edge[0] as usize].push(edge[1] as usize);
            }
            reqs
        };
        let mut cache = vec![false; num_courses];

        for course in 0..num_courses {
            let mut path = HashSet::new();
            if !Self::check(course, &reqs, &mut path, &mut cache) {
                return false;
            }
        }
        true
    }

    fn check(
        course: usize,
        reqs: &Vec<Vec<usize>>,
        path: &mut HashSet<usize>,
        cache: &mut Vec<bool>,
    ) -> bool {
        if cache[course] {
            return true;
        }
        if !path.insert(course) {
            return false;
        }
        for req in &reqs[course] {
            if !Self::check(*req, reqs, path, cache) {
                return false;
            }
        }
        path.remove(&course);
        cache[course] = true;
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn as_vecs(arr: &[[i32; 2]]) -> Vec<Vec<i32>> {
        arr.iter().map(|pair| pair.to_vec()).collect()
    }

    #[test]
    fn test1() {
        let num_courses = 2;
        let prerequisites = as_vecs(&[[1, 0]]);
        assert!(Solution::can_finish(num_courses, prerequisites));
    }

    #[test]
    fn test2() {
        let num_courses = 2;
        let prerequisites = as_vecs(&[[1, 0], [0, 1]]);
        assert!(!Solution::can_finish(num_courses, prerequisites));
    }

    #[test]
    fn test3() {
        let num_courses = 5;
        let prerequisites = as_vecs(&[[1, 4], [2, 4], [3, 1], [3, 2]]);
        assert!(Solution::can_finish(num_courses, prerequisites));
    }

    #[test]
    fn test4() {
        let num_courses = 100;
        #[rustfmt::skip]
        let prerequisites = as_vecs(&[
            [1,  0 ], [2,  0 ], [2,  1 ], [3,  1 ], [3 , 2 ], [4,  2 ], [4,  3 ], [5,  3 ],
            [5,  4 ], [6,  4 ], [6,  5 ], [7,  5 ], [7,  6 ], [8,  6 ], [8 , 7 ], [9,  7 ],
            [9,  8 ], [10,  8], [10, 9 ], [11, 9 ], [11, 10], [12, 10], [12, 11], [13, 11],
            [13, 12], [14, 12], [14, 13], [15, 13], [15, 14], [16, 14], [16, 15], [17, 15],
            [17, 16], [18, 16], [18, 17], [19, 17], [19, 18], [20, 18], [20, 19], [21, 19],
            [21, 20], [22, 20], [22, 21], [23, 21], [23, 22], [24, 22], [24, 23], [25, 23],
            [25, 24], [26, 24], [26, 25], [27, 25], [27, 26], [28, 26], [28, 27], [29, 27],
            [29, 28], [30, 28], [30, 29], [31, 29], [31, 30], [32, 30], [32, 31], [33, 31],
            [33, 32], [34, 32], [34, 33], [35, 33], [35, 34], [36, 34], [36, 35], [37, 35],
            [37, 36], [38, 36], [38, 37], [39, 37], [39, 38], [40, 38], [40, 39], [41, 39],
            [41, 40], [42, 40], [42, 41], [43, 41], [43, 42], [44, 42], [44, 43], [45, 43],
            [45, 44], [46, 44], [46, 45], [47, 45], [47, 46], [48, 46], [48, 47], [49, 47],
            [49, 48], [50, 48], [50, 49], [51, 49], [51, 50], [52, 50], [52, 51], [53, 51],
            [53, 52], [54, 52], [54, 53], [55, 53], [55, 54], [56, 54], [56, 55], [57, 55],
            [57, 56], [58, 56], [58, 57], [59, 57], [59, 58], [60, 58], [60, 59], [61, 59],
            [61, 60], [62, 60], [62, 61], [63, 61], [63, 62], [64, 62], [64, 63], [65, 63],
            [65, 64], [66, 64], [66, 65], [67, 65], [67, 66], [68, 66], [68, 67], [69, 67],
            [69, 68], [70, 68], [70, 69], [71, 69], [71, 70], [72, 70], [72, 71], [73, 71],
            [73, 72], [74, 72], [74, 73], [75, 73], [75, 74], [76, 74], [76, 75], [77, 75],
            [77, 76], [78, 76], [78, 77], [79, 77], [79, 78], [80, 78], [80, 79], [81, 79],
            [81, 80], [82, 80], [82, 81], [83, 81], [83, 82], [84, 82], [84, 83], [85, 83],
            [85, 84], [86, 84], [86, 85], [87, 85], [87, 86], [88, 86], [88, 87], [89, 87],
            [89, 88], [90, 88], [90, 89], [91, 89], [91, 90], [92, 90], [92, 91], [93, 91],
            [93, 92], [94, 92], [94, 93], [95, 93], [95, 94], [96, 94], [96, 95], [97, 95],
            [97, 96], [98, 96], [98, 97], [99, 97],
        ]);
        assert!(Solution::can_finish(num_courses, prerequisites));
    }
}
