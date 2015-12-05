mod rhyme;
use rhyme::*;

extern crate rustc_serialize;
use rustc_serialize::json::{self, Json};
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("heart.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();
    
    let json = Json::from_str(&data).unwrap();

    let arr = json.as_array().unwrap();

    let mut rhyme_list : Vec<Rhyme> = Vec::new();
    for item in arr {
        // println!("item: {}", item.to_string());
        let decoded : Rhyme = json::decode(&item.to_string()).unwrap();
        println!("Look ma, I decoded a {}", decoded.word);
        rhyme_list.push(decoded);
    }

    rhyme_list.retain( |x| x.score == 300);
    println!("The filtered rhyme list: {}", rhyme_list.len());
}
