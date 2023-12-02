use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("inputs/day2.txt").unwrap();
    let games = input.split("\n").collect::<Vec<_>>();
    let mut ans = 0;
    'outer: for game in games {
        //println!("{game}");
        let content = game.split(":").collect::<Vec<_>>()[1];
        //println!("{content}");
        let id = game.split(":").collect::<Vec<_>>()[0]
            .split(" ").collect::<Vec<_>>()[1].parse::<i32>().unwrap();
        let rounds = content.split(";").collect::<Vec<_>>();
        let mut map = HashMap::from([
            ("red", 0), ("blue", 0), ("green", 0)
        ]);
        for round in rounds {
            let draws = round.split(",").map(|d| d.trim()).collect::<Vec<_>>();
            for draw in draws {
                let split = draw.split(" ").collect::<Vec<_>>();
                let num = split[0].parse::<i32>().unwrap();
                let colour = split[1];
                if map[colour] < num {
                    map.insert(colour, num);
                }
                // map[colour].insert(num);
            }
            /*if map["red"] > 12 || map["green"] > 13 || map["blue"] > 14 {
                continue 'outer;
            }*/
        }
        ans += map["red"] * map["green"] * map["blue"];
    }
    println!("{ans}");
}

static EXAMPLE: &str = "\
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\
";