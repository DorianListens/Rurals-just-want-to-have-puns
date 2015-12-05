mod rhyme;
use rhyme::*;

extern crate rustc_serialize;
use rustc_serialize::json::{self, Json};
extern crate curl;
use curl::http;
use std::str;
use std::env;

fn main() {
    let word = env::args().nth(1).unwrap_or("heart".to_string());
    println!("the word is really: {}", word);
    let url = format!("http://rhymebrain.com/talk?function=getRhymes&word={}", word);
    let resp = http::handle()
        .get(url)
        .exec().unwrap();

    let data = str::from_utf8(resp.get_body()).unwrap();
    let json = Json::from_str(&data).unwrap();

    let mut rhyme_list : Vec<Rhyme> = json.as_array().unwrap().iter()
        .map(|item| json::decode::<Rhyme>(&item.to_string()).unwrap())
        .filter(|item| item.score == 300)
        .collect();

    println!("The filtered rhyme list: {}", rhyme_list.len());

    for item in rhyme_list {
        println!("the word is: {}", item.word);
    }
}
