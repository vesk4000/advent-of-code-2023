use std::fs;
use std::env;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let input = fs::read_to_string("inputs/day1.txt").unwrap();
    let lines = input.split('\n').collect::<Vec<_>>();

    let mut zero_bytes = [0; 2];
    '0'.encode_utf8(&mut zero_bytes);
    let zero_byte = zero_bytes[0];

    let re = Regex::new(concat!(r"(one)|(two)|(three)|(four)|(five)|(six)",
        "|(seven)|(eight)|(nine)","|([0-9])")).unwrap();
    let er = Regex::new(concat!(r"(eno)|(owt)|(eerht)|(ruof)|(evif)|(xis)",
    "|(neves)|(thgie)|(enin)","|([0-9])")).unwrap();let hash = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);
    let hash_rev = HashMap::<String, i32>::from([
        ("one".chars().rev().collect(), 1),
        ("two".chars().rev().collect::<String>(), 2),
        ("three".chars().rev().collect::<String>(), 3),
        ("four".chars().rev().collect::<String>(), 4),
        ("five".chars().rev().collect::<String>(), 5),
        ("six".chars().rev().collect::<String>(), 6),
        ("seven".chars().rev().collect::<String>(), 7),
        ("eight".chars().rev().collect::<String>(), 8),
        ("nine".chars().rev().collect::<String>(), 9),
        ("1".to_string(), 1),
        ("2".to_string(), 2),
        ("3".to_string(), 3),
        ("4".to_string(), 4),
        ("5".to_string(), 5),
        ("6".to_string(), 6),
        ("7".to_string(), 7),
        ("8".to_string(), 8),
        ("9".to_string(), 9),
    ]);

    let anz = re.captures(lines[0]).unwrap();

    let mut ans = 0;
    for line in lines {
        if line.is_empty() {
            break;
        }

        let mut curr: i32 = 0;

        //println!("{}", line);

        let mut charz = vec![];
        for (_, [one]) in re.captures_iter(line).map(|c| c.extract()) {
            //println!("{}", hash[one]);
            charz.push(hash[one]);

        }

        let mut cha = vec![];
        println!("{}",line.chars().rev().collect::<String>());
        for (_, [one]) in er.captures_iter(&mut line.chars().rev().collect::<String>()).map(|c| c.extract()) {
            //println!("{}", hash[one]);
            cha.push(hash_rev[one]);

        }

        if(charz[charz.len() - 1] != cha[0]) {
            println!("Cowabunga!");
        }

        let charzz = vec![charz[0], cha[0]];
        for c in charzz {
            curr += c;
            curr *= 10;
        }

        //println!("skrrrt!");



        /*let mut charz = vec![];
        for c in line.chars() {
            if c >= '0' && c <= '9' {
                charz.push(c);
            }
        }
        let charzz = vec![charz[0], charz[charz.len() - 1]];

        for c in charzz {
            let mut c_bytes = [0; 2];
            c.encode_utf8(&mut c_bytes);
            let c_byte = c_bytes[0];
            curr += i32::from(c_byte - zero_byte);
            curr *= 10;
        }*/
        ans += curr / 10;
        println!("{} {}", line, curr / 10);
    }
    println!("{}", ans);
}

static EXAMPLE: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

static EXAMPLE_HARD: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";