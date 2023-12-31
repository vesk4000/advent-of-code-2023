use std::fs;
use std::ops::Bound::Excluded;
use regex::Regex;

fn main() {
    let input = fs::read_to_string("inputs/day9.txt").unwrap();
    let lines = input.trim().split("\n");
    let re = Regex::new(r"-?\d+").unwrap();
    let mut ans = 0;
    let mut ans_pt2 = 0;
    for line in lines {
        let nums = re.find_iter(line).map(|m| m.as_str().parse::<i32>().unwrap()).collect::<Vec<_>>();
        let sol = solve(&nums);
        ans += sol.1;
        ans_pt2 += sol.0;
    }
    println!("{ans} {ans_pt2}");
}

fn solve(v: &[i32]) -> (i32, i32) {
    if v.iter().all(|x| *x == 0) { return (0, 0); }

    let mut vv = Vec::<i32>::new();
    for i in 0..v.len() - 1 {
        vv.push(v[i + 1] - v[i]);
    }

    let sol = solve(&vv);
    (v[0] - sol.0, v[v.len() - 1] + sol.1)
}

static EXAMPLE: &str = "
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";