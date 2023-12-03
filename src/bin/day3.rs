use std::cmp::{max, min};
use std::collections::{HashMap, HashSet};
use std::{fs, panic};
use regex::Regex;

#[allow(unused)]

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/day3.txt")?;
    let lines = input.trim().split("\n").collect::<Vec<_>>();
    let re = Regex::new(r"\d+")?;
    let symbols = HashSet::from(['$', '%', '/', '*', '&', '#', '+', '-', '@', '=']);
    let mut ans = 0;
    for (i, line) in lines.iter().enumerate() {
        for _match in re.find_iter(line) {
            'curr_num: for y in i.checked_sub(1).unwrap_or(0)..=min(lines.len() - 1, i + 1) {
                for x in _match.start().checked_sub(1).unwrap_or(0)..=min(line.len() - 1, _match.end()) {
                    if(symbols.contains(&lines[y].chars().nth(x).unwrap())) {
                        //println!("{}", _match.as_str());
                        ans += _match.as_str().parse::<i32>()?;
                        break 'curr_num;
                    }
                }
            }
        }
    }
    println!("{ans}");
    Ok(())
}

static EXAMPLE: &str = "
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";