use std::fs;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/day6.txt")?;
    let (time_line, dist_line) = input.trim().split_once("\n").unwrap();
    let re = Regex::new(r"\d+")?;
    let times = re.find_iter(time_line).map(|m| m.as_str().parse::<u32>().unwrap());
    let dists = re.find_iter(dist_line).map(|m| m.as_str().parse::<u32>().unwrap());
    let mut ans = 1;
    for (time, best_dist) in times.zip(dists) {
        let mut num = 0;
        for i in 0..=time {
            if i * (time - i) > best_dist {
                num += 1;
            }
        }
        ans *= num;
    }
    println!("{ans}");
    Ok(())
}