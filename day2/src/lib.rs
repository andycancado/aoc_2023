use anyhow::Result;
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;
use std::str::FromStr;

const MAX_RED: i32 = 12;
const MAX_GREEN: i32 = 13;
const MAX_BLUE: i32 = 14;

fn read_file(path: &str) -> String {
    let input = std::fs::read_to_string(path).expect("Error reading input.txt");
    input
}

fn split_input(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

#[derive(Debug, Default)]
struct GameDay2 {
    game_number: i32,
    red: i32,
    blue: i32,
    green: i32,
}

impl GameDay2 {
    fn validate(&self) -> bool {
        self.red > MAX_RED || self.green > MAX_GREEN || self.blue > MAX_BLUE
    }
}

impl FromStr for GameDay2 {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut game = GameDay2::default();
        let split = s.split(":").collect::<Vec<&str>>();
        game.game_number = split
            .first()
            .unwrap()
            .to_string()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()?;
        let p = split.last().unwrap().to_string().trim().to_string();

        for pp in p.split(";").collect::<Vec<&str>>().iter() {
            // println!("::: {:?}", pp.trim());
            let split = pp.trim().split(",").collect::<Vec<&str>>();

            for s in split.iter() {
                let mut split = s.trim().split_ascii_whitespace();
                let num = split.next().unwrap().parse::<i32>()?;
                let value = split.next().unwrap().to_string();
                match value.as_str() {
                    "red" => game.red = std::cmp::max(game.red, num),
                    "blue" => game.blue = std::cmp::max(game.blue, num),
                    "green" => game.green = std::cmp::max(game.green, num),
                    _ => (),
                }
            }
        }
        Ok(game)
    }
}

fn get_pt1(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    let f_txt = read_file("input_files/input_d2_2.txt");
    let f_split = split_input(&f_txt);
    let res: i32 = f_split
        .iter()
        .map(|g| GameDay2::from_str(g).unwrap())
        .filter(|g| g.validate())
        .map(|g| g.game_number)
        .sum();
    Ok(Response::new(200, res.to_string()))
}

fn get_pt2(_req: Request, _params: Params) -> Result<impl IntoResponse> {
    let f_txt = read_file("input_files/input_d2_2.txt");
    let f_split = split_input(&f_txt);
    let res: i32 = f_split
        .iter()
        .map(|g| GameDay2::from_str(g).unwrap())
        .map(|g| g.blue * g.red * g.green)
        .sum();
    Ok(Response::new(200, res.to_string()))
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_day02(req: Request) -> impl IntoResponse {
    let mut router = Router::new();
    router.get("/day2/pt1", get_pt1);
    router.get("/day2/pt2", get_pt2);
    router.any("/day2/*", not_found);
    router.handle(req)
}

fn not_found(_: Request, _: Params) -> Result<impl IntoResponse> {
    Ok(Response::new(200, "not found".to_string()))
}
