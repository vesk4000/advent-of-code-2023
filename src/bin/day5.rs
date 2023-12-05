use std::fs;
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = fs::read_to_string("inputs/day5.txt")?;
    let input_parts = input.trim().split(":").collect::<Vec<_>>();
    let re = Regex::new(r"\d+")?;
    let seeds = re.find_iter(input_parts[1]).map(|m| m.as_str().parse::<u64>().unwrap()).collect::<Vec<_>>();
    //println!("{seeds}");
    let mut transforms = [Vec::<(u64, u64, u64)>::new(),
        Vec::<(u64, u64, u64)>::new(), Vec::<(u64, u64, u64)>::new(), Vec::<(u64, u64, u64)>::new()
    ,Vec::<(u64, u64, u64)>::new(), Vec::<(u64, u64, u64)>::new(), Vec::<(u64, u64, u64)>::new()];
    for (i, input_part) in input_parts.iter().enumerate() {
        //println!("{input_part}");
        let lines = input_part.split("\n").collect::<Vec<_>>();
        if(i == 0 || i == 1) {
            continue;
        }
        for line in lines {

            let nums = re.find_iter(line).map(|m| m.as_str().parse::<u64>().unwrap()).collect::<Vec<_>>();
            if(nums.is_empty()) {
                continue;
            }
            transforms[i - 2].push((nums[0], nums[1], nums[2]));
        }
    }
    let mut ans: u64 = 1000000000;
    'ss: for seed in seeds {
        //println!("{seed}");
        let mut s = seed;

        'tt: for transform in &transforms {
            //println!("{seed} {s}");
            'rr: for r in transform {
                if(s >= r.1 && s < r.1 + r.2) {
                    s = r.0 + s - r.1;
                    break 'rr;
                }
            }
        }
        //println!("{seed} {s}");
        if(ans > s) {
            ans = s;
        }
    }
    println!("{ans}");

    Ok(())
}

static EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";