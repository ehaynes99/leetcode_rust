pub struct Solution;

use std::cmp;
use std::collections::{hash_map::Entry, HashMap};

impl Solution {
    pub fn accounts_merge(mut accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut graph: Vec<Vec<usize>> = vec![vec![]; accounts.len()];
        let mut mail_map: HashMap<String, usize> = HashMap::new();

        for index in 0..accounts.len() {
            while accounts[index].len() > 1 {
                let email = accounts[index].pop().unwrap();
                match mail_map.entry(email) {
                    Entry::Occupied(entry) => {
                        let other = *entry.get();
                        graph[index].push(other);
                        graph[other].push(index);
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(index);
                    }
                }
            }
        }

        let components = Self::get_components(&graph);

        mail_map.drain().for_each(|(email, index)| {
            let index = components[index];
            accounts[index].push(email);
        });

        (0..accounts.len()).for_each(|index| {
            accounts[index][1..].sort_unstable();
        });
        accounts
            .into_iter()
            .filter(|account| account.len() > 1)
            .collect()
    }

    fn get_components(graph: &Vec<Vec<usize>>) -> Vec<usize> {
        let mut visited = vec![false; graph.len()];
        let mut components = (0..visited.len()).into_iter().collect::<Vec<_>>();

        for index in 0..graph.len() {
            if !visited[index] {
                let mut min = index;
                let mut stack = vec![index];
                let mut component = vec![];

                while !stack.is_empty() {
                    let curr = stack.pop().unwrap();
                    visited[curr] = true;
                    component.push(curr);
                    min = cmp::min(min, curr);

                    graph[curr].iter().for_each(|other| {
                        if !visited[*other] {
                            stack.push(*other);
                        }
                    })
                }
                component.into_iter().for_each(|i| components[i] = min);
            }
        }
        components
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util::test_utils::assert_eq_nested_unordered;

    fn to_strings(arr: Vec<Vec<&str>>) -> Vec<Vec<String>> {
        arr.into_iter()
            .map(|strings| strings.into_iter().map(|str| str.to_string()).collect())
            .collect()
    }

    #[test]
    fn test1() {
        let accounts = to_strings(vec![
            vec!["John", "johnsmith@mail.com", "john_newyork@mail.com"],
            vec!["John", "johnsmith@mail.com", "john00@mail.com"],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"],
        ]);

        let expected = to_strings(vec![
            vec![
                "John",
                "john00@mail.com",
                "john_newyork@mail.com",
                "johnsmith@mail.com",
            ],
            vec!["Mary", "mary@mail.com"],
            vec!["John", "johnnybravo@mail.com"],
        ]);
        assert_eq_nested_unordered(expected, Solution::accounts_merge(accounts));
    }
    #[test]
    fn test2() {
        let accounts = to_strings(vec![
            vec!["Gabe", "Gabe0@m.co", "Gabe3@m.co", "Gabe1@m.co"],
            vec!["Kevin", "Kevin3@m.co", "Kevin5@m.co", "Kevin0@m.co"],
            vec!["Ethan", "Ethan5@m.co", "Ethan4@m.co", "Ethan0@m.co"],
            vec!["Hanzo", "Hanzo3@m.co", "Hanzo1@m.co", "Hanzo0@m.co"],
            vec!["Fern", "Fern5@m.co", "Fern1@m.co", "Fern0@m.co"],
        ]);

        let expected = to_strings(vec![
            vec!["Ethan", "Ethan0@m.co", "Ethan4@m.co", "Ethan5@m.co"],
            vec!["Gabe", "Gabe0@m.co", "Gabe1@m.co", "Gabe3@m.co"],
            vec!["Hanzo", "Hanzo0@m.co", "Hanzo1@m.co", "Hanzo3@m.co"],
            vec!["Kevin", "Kevin0@m.co", "Kevin3@m.co", "Kevin5@m.co"],
            vec!["Fern", "Fern0@m.co", "Fern1@m.co", "Fern5@m.co"],
        ]);
        assert_eq_nested_unordered(expected, Solution::accounts_merge(accounts));
    }

    #[test]
    fn test3() {
        let accounts = to_strings(vec![
            vec!["David", "David0@m.co", "David1@m.co"],
            vec!["David", "David3@m.co", "David4@m.co"],
            vec!["David", "David4@m.co", "David5@m.co"],
            vec!["David", "David2@m.co", "David3@m.co"],
            vec!["David", "David1@m.co", "David2@m.co"],
        ]);
        let expected = to_strings(vec![vec![
            "David",
            "David0@m.co",
            "David1@m.co",
            "David2@m.co",
            "David3@m.co",
            "David4@m.co",
            "David5@m.co",
        ]]);
        assert_eq_nested_unordered(expected, Solution::accounts_merge(accounts));
    }
}
