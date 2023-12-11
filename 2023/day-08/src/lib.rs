use regex::Regex;
use std::collections::HashMap;

enum PatternDirection {
    Left,
    Right,
}

pub struct DesertMaps {
    pattern: Vec<PatternDirection>,
    maps: HashMap<String, (String, String)>,
    start_nodes: Vec<String>,
    end_nodes: Vec<String>,
}

impl DesertMaps {
    pub fn from_input(mut input: impl Iterator<Item = String>) -> Self {
        let re =
            Regex::new(r"(?P<key>[A-Z0-9]{3}) = \((?P<left>[A-Z0-9]{3}), (?P<right>[A-Z0-9]{3})\)")
                .unwrap();
        let pattern = input.next().unwrap();
        let pattern: Vec<_> = pattern
            .chars()
            .map(|c| match c {
                'L' => PatternDirection::Left,
                'R' => PatternDirection::Right,
                _ => unreachable!(),
            })
            .collect();
        input.next(); // consume empty line.

        let mut start_nodes: Vec<String> = vec![];
        let mut end_nodes: Vec<String> = vec![];

        let mut maps: HashMap<String, (String, String)> = HashMap::new();
        input.for_each(|line| {
            let caps = re.captures(&line).unwrap();
            let key = &caps["key"];
            let (left, right) = (&caps["left"], &caps["right"]);

            if key.ends_with('A') {
                start_nodes.push(key.into());
            }
            if key.ends_with('Z') {
                end_nodes.push(key.into());
            }
            maps.insert(key.into(), (left.into(), right.into()));
        });
        Self {
            pattern,
            maps,
            start_nodes,
            end_nodes,
        }
    }

    pub fn start_nodes(&self) -> &Vec<String> {
        &self.start_nodes
    }

    pub fn end_nodes(&self) -> &Vec<String> {
        &self.end_nodes
    }

    pub fn step_count(&self, start_node: String, end_nodes: &[String]) -> usize {
        let mut step = 0;
        let mut count = 0;
        let mut current = start_node;

        loop {
            current = match self.pattern[step] {
                PatternDirection::Left => self.maps[&current].0.to_owned(),
                PatternDirection::Right => self.maps[&current].1.to_owned(),
            };
            count += 1;

            if end_nodes.contains(&current) {
                break;
            }

            if step + 1 >= self.pattern.len() {
                step = 0;
            } else {
                step += 1;
            }
        }

        count
    }
}
