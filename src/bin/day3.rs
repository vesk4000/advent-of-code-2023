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
    let mut ans2 = 0;
    let mut v = HashMap::<(usize, usize), (usize, i32)>::new();
    for (i, line) in lines.iter().enumerate() {
        for _match in re.find_iter(line) {
            'curr_num: for y in i.checked_sub(1).unwrap_or(0)..=min(lines.len() - 1, i + 1) {
                for x in _match.start().checked_sub(1).unwrap_or(0)..=min(line.len() - 1, _match.end()) {
                    if(symbols.contains(&lines[y].chars().nth(x).unwrap())) {
                        //println!("{}", _match.as_str());
                        if(lines[y].chars().nth(x).unwrap() == '*') {
                            if(!v.contains_key(&(x, y))) {
                                v.insert((x, y), (0, 1));
                            }
                            let (count, sum) = v[&(x, y)];
                            v.insert((x, y), (count + 1, sum * _match.as_str().parse::<i32>()?));
                        }
                        //ans += _match.as_str().parse::<i32>()?;
                        //break 'curr_num;
                    }
                }
            }
        }
    }

    println!("{ans}");
    println!("{}", v.values().map(|vv| if vv.0 == 2 {vv.1} else {0}).sum::<i32>());
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