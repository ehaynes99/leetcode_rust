use std::collections::HashMap;

type Entry = (i32, String);

pub struct TimeMap {
    values: HashMap<String, Vec<Entry>>,
}

impl TimeMap {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: String, timestamp: i32) {
        let entry = self.values.entry(key).or_default();
        (*entry).push((timestamp, value));
    }

    pub fn get(&self, key: String, timestamp: i32) -> String {
        if let Some(values) = self.values.get(&key) {
            let index = values.partition_point(|(ts, _)| *ts <= timestamp);
            if index > 0 {
                return values[index - 1].1.clone();
            }
        }
        String::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let mut time_map = TimeMap::new();
        time_map.set("foo".to_string(), "bar".to_string(), 1);
        assert_eq!("bar".to_string(), time_map.get("foo".to_string(), 1));
        assert_eq!("bar".to_string(), time_map.get("foo".to_string(), 3));
        time_map.set("foo".to_string(), "bar2".to_string(), 4);
        assert_eq!("bar2".to_string(), time_map.get("foo".to_string(), 4));
        assert_eq!("bar2".to_string(), time_map.get("foo".to_string(), 5));
    }
}
