use std::fs;

#[derive(Debug)]
struct Draw {
    red: u32,
    green: u32,
    blue: u32,
}

#[derive(Debug)]
struct Game {
    game_number: u32,
    rounds: Vec<Draw>,
}

fn main() {
    const BAG_STATE: Draw = Draw {
        red: 12,
        green: 13,
        blue: 14,
    };
    let input = read_input();
    let mut games: Vec<Game> = vec![];
    for line in input.lines() {
        let mut rounds: Vec<Draw> = vec![];
        let game_number = line
            .split(":")
            .next()
            .unwrap()
            .split(" ")
            .nth(1)
            .unwrap()
            .parse::<u32>()
            .unwrap();
        // Iterate through each round
        for round in line.split(":").nth(1).unwrap().split(";") {
            let mut red = 0;
            let mut green = 0;
            let mut blue = 0;
            // iterate through each round
            for item in round.split(",") {
                let amount = item
                    .trim()
                    .split(" ")
                    .next()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();
                let color = item.trim().split(" ").nth(1).unwrap();
                if color == "green" {
                    green += amount;
                } else if color == "red" {
                    red += amount;
                } else if color == "blue" {
                    blue += amount;
                }
            }
            let draw = Draw {
                red: red,
                green: green,
                blue: blue,
            };
            rounds.push(draw);
        }
        let game = Game {
            game_number: game_number,
            rounds: rounds,
        };
        games.push(game);
    }
    let res = games.iter().map(|game| validate_game(game, &BAG_STATE));
    dbg!(res.sum::<u32>());
}

fn validate_draw(draw: &Draw, bag_state: &Draw) -> bool {
    draw.blue <= bag_state.blue && draw.red <= bag_state.red && draw.green <= bag_state.green
}

fn validate_game(game: &Game, bag_state: &Draw) -> u32 {
    if game
        .rounds
        .iter()
        .all(|draw| validate_draw(draw, bag_state))
    {
        return game.game_number;
    }
    0
}

fn read_input() -> String {
    return fs::read_to_string("./input.txt").expect("Could not read from input file!");
}
