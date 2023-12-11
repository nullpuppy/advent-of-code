use std::collections::HashSet;

/// Use PartNumber to track numbers found in input, and where we found them, and how many
/// characters the number uses.
/// We derive PartialEq, Eq and Hash so we can filter into a HashSet
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PartNumber {
    pub num: usize,
    pub row: usize, // line number was found on.
    pub col: usize, // column the part number starts on
    pub len: usize, // how long the part number is
}

/// Use PartSymbol to keep track of every symbol found in the input, and where we found it.
#[derive(Debug, Clone)]
pub struct PartSymbol {
    pub symbol: char,
    pub row: usize, // line number the part was found on.
    pub col: usize, // column the part was found on
}

impl PartNumber {
    /// Check if the symbol requested is adjacent to the current part number.
    /// To do this, we look if the symbol's row and col lie within the perimeter of
    /// `cells` that surround the part number
    pub fn symbol_is_adjacent(&self, symbol: &PartSymbol) -> bool {
        symbol.row >= self.row.saturating_sub(1)
            && symbol.row <= self.row.saturating_add(1)
            && symbol.col >= self.col.saturating_sub(1)
            && symbol.col <= self.col.saturating_add(self.len)
    }
}

impl PartSymbol {
    pub fn adjacent_part_numbers(&self, part_numbers: &Vec<PartNumber>) -> HashSet<PartNumber> {
        part_numbers
            .iter()
            .filter_map(|part_num| {
                if part_num.symbol_is_adjacent(&self) {
                    Some(part_num.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

pub fn parse_input(input: impl Iterator<Item = String>) -> (Vec<PartSymbol>, Vec<PartNumber>) {
    let mut numbers: Vec<PartNumber> = vec![];
    let mut symbols: Vec<PartSymbol> = vec![];

    for (row, line) in input.enumerate() {
        let mut num = String::new();

        for (col, ch) in line.chars().enumerate() {
            match ch {
                ch if ch.is_digit(10) => {
                    num.push(ch);
                }
                _ => {
                    match ch {
                        ch if ch != '.' => symbols.push(PartSymbol {
                            symbol: ch,
                            row,
                            col,
                        }),
                        _ => {}
                    }
                    if num.len() > 0 {
                        // If we hit a non-digit, and have a number, save the number
                        // and reset our temp
                        numbers.push(PartNumber {
                            num: num.parse::<usize>().unwrap(),
                            row,
                            col: col - num.len(),
                            len: num.len(),
                        });
                        num = String::new();
                    }
                }
            }
        }

        // We might end the above loop with a number (number ends at EOL, not a non-digit char)
        if num.len() > 0 {
            numbers.push(PartNumber {
                num: num.parse::<usize>().unwrap(),
                row,
                col: line.len() - num.len(),
                len: num.len(),
            });
        }
    }

    (symbols, numbers)
}
