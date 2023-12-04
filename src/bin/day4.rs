use std::collections::{HashSet, VecDeque};
use std::fs;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/day4.txt")?;
    let lines = input.trim().split("\n");
    let re = Regex::new(r"\d+")?;
    let mut card_wins = Vec::<usize>::new();
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut q = VecDeque::<usize>::new();
    for line in lines {
        let (card, cont) = line.split_once(":").unwrap();
        let (win, have) = cont.split_once("|").unwrap();
        let winning_nums = re.find_iter(win)
            .map(|m| m.as_str()).collect::<HashSet<_>>();
        let num_wins = re.find_iter(have)
            .filter(|m| winning_nums.contains(m.as_str())).count();
        card_wins.push(num_wins);
        q.push_back(re.find_iter(card).next().unwrap().as_str().parse::<usize>()? - 1);
        ans1 += if num_wins == 0 {0} else { 2_i32.pow((num_wins - 1) as u32) };
    }
    while !q.is_empty() {
        ans2 += 1;
        let card = q.pop_front().unwrap();
        for i in card + 1..=std::cmp::min(card + card_wins[card], card_wins.len() - 1) {
            q.push_back(i);
        }
    }
    println!("{ans1}");
    println!("{ans2}");
    Ok(())
}

static EXAMPLE: &str = "\
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";