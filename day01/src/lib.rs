use spin_sdk::http::{IntoResponse, Request, Params, Response, Router};
use spin_sdk::{http_component};
use std::str::FromStr;
use anyhow::Result;

fn read_file(path: &str) -> String {
    let input = std::fs::read_to_string(path).expect("Error reading input.txt");
    input
}

fn split_input(input: &str) -> Vec<&str> {
    input.split("\n").collect()
}

#[derive(Debug)]
struct Number<T>(T);

static NUMS: [(&str, u32); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

impl FromStr for Number<u32> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pairs = Vec::new();
        for (n, i) in NUMS.iter() {
            let x = s.match_indices(n).collect::<Vec<(usize, &str)>>();
            for (j, _) in x.iter() {
                pairs.push((*j, *i));
            }
        }
        for (i, c) in s.chars().enumerate() {
            if c.is_numeric() {
                pairs.push((i, c.to_digit(10).unwrap()));
            }
        }
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        let sum = (pairs[0].1 * 10) + pairs[pairs.len() - 1].1;
        Ok(Number(sum))
    }
}

fn get_pt1(_req: Request, _params: Params) -> Result<impl IntoResponse>  {
    let input = read_file("input_files/input_d1.txt");
    let mut result = Vec::new();
    for i in split_input(&input) {
        let r: Vec<u32> = i
            .chars()
            .filter(|c| c.is_numeric())
            .flat_map(|f| f.to_digit(10))
            .collect();
        let sum = (r[0] * 10) + r[r.len() - 1];
        result.push(sum);
    }
    let res = result.iter().sum::<u32>();
    Ok(Response::new(200, res.to_string()))
  
}

fn get_pt2(_req: Request, _params: Params) -> Result<impl IntoResponse>   {
    let input = read_file("input_files/input_d2.txt");
    let lns = split_input(input.as_str());
    let l: Vec<Number<u32>> = lns.iter().map(|ll| Number::from_str(ll).unwrap()).collect();
    let res = l.iter().map(|x| x.0).sum::<u32>();
    println!("PART 2 ::: {res:?} :::");
    Ok(Response::new(200, res.to_string()))
}

/// A simple Spin HTTP component.
#[http_component]
fn handle_day01(req: Request) -> impl IntoResponse {

    let mut router = Router::new();
    router.get("/day1/pt1", get_pt1);
    router.get("/day1/pt2", get_pt2);
    router.any("/api/*", not_found);
    router.handle(req)
}
   

fn not_found(_: Request, _: Params) -> Result<impl IntoResponse> {
    Ok(Response::new(200, "not found".to_string()))
}
