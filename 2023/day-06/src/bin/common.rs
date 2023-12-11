pub fn parse_line(line: String) -> Vec<usize> {
    line.split_once(":")
        .unwrap()
        .1
        .split_whitespace()
        .map(|n| n.parse::<usize>().unwrap())
        .collect()
}

pub fn concat_vec(v: Vec<usize>) -> usize {
    v.iter()
        .fold("".to_string(), |acc, v| acc + &v.to_string())
        .parse::<usize>()
        .unwrap()
}
