pub struct Solution;

const OFFSET: u8 = 'a' as u8;

impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let get_index = |c: char| (c as u8 - OFFSET) as usize;
        let mut counts = [0u32; 26];
        for index in magazine.chars().map(&get_index) {
            counts[index] += 1;
        }

        for index in ransom_note.chars().map(&get_index) {
            if counts[index] < 1 {
                return false;
            }
            counts[index] -= 1;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let ransom_note = "a".to_string();
        let magazine = "b".to_string();
        assert!(!Solution::can_construct(ransom_note, magazine));
    }

    #[test]
    fn test2() {
        let ransom_note = "aa".to_string();
        let magazine = "ab".to_string();
        assert!(!Solution::can_construct(ransom_note, magazine));
    }

    #[test]
    fn test3() {
        let ransom_note = "aa".to_string();
        let magazine = "aab".to_string();
        assert!(Solution::can_construct(ransom_note, magazine));
    }

    #[test]
    fn test4() {
        let ransom_note = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
        let magazine = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
        assert!(Solution::can_construct(ransom_note, magazine));
    }
}
