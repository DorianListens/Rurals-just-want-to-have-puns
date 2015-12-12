use std::io::prelude::*;
mod rhyme;
use rhyme::*;

extern crate regex;
use regex::Regex;
use regex::NoExpand;
extern crate rustc_serialize;
use rustc_serialize::json::{self, Json};
extern crate curl;
use curl::http;
use std::str;
use std::env;
use std::fs::{self, File};
use std::path::Path;

fn main() {
    let word = env::args().nth(1).unwrap_or("heart".to_string());
    println!("the word is really: {}", word);
    let json = fetch_json(&word);

    let rhymes = json.as_array().unwrap_or(&vec![]).iter()
        .filter_map(decode_rhyme)
        .filter(|rhyme| rhyme.score == 300)
        .collect::<Vec<Rhyme>>();

    let strings = pull_strings_from_dir();

    let puns:Vec<Pun> = strings.iter()
        .filter_map( |string| return make_puns(&rhymes, &word, string))
        .collect();

    for pun in &puns {
        println!("{} (pun of {})", &pun.pun, &pun.original);
    }
}

fn make_puns(rhymes: &Vec<Rhyme>, word: &String, string:&String) -> Option<Pun> {
    for rhyme in rhymes {
       let rstring = format!("\\b{}\\b", &rhyme.word);
       if let Ok(regex) = Regex::new(&rstring) {
           if regex.is_match(&string) {
               let replaced = regex.replace(&string, NoExpand(&word));
               return Some(Pun::new(&string, &replaced));
           }
       }
    }
    return None;
}

fn decode_rhyme(item:&Json) -> Option<Rhyme> {
    if let Ok(rhyme) = json::decode::<Rhyme>(&item.to_string()) {
        return Some(rhyme)
    }
    return None
}

fn fetch_json(word:&String) -> Json {
    let url = format!("http://rhymebrain.com/talk?function=getRhymes&word={}", word);
    let resp = http::handle().get(url).exec().unwrap(); 

    let data = str::from_utf8(resp.get_body()).unwrap();
    let json = Json::from_str(&data).unwrap();
    return json;
}

fn pull_strings_from_dir() -> Vec<String> {
    let path = Path::new("./phrases/");
    let files = fs::read_dir(path).unwrap();

    let mut strings : Vec<String> = Vec::new();
    for file in files {
        let mut f = File::open(file.unwrap().path()).unwrap();
        let mut s = String::new();
        f.read_to_string(&mut s);
        let str = s.split("\n").collect::<Vec<&str>>();
        for st in str {
            strings.push(st.to_string());
        }
    }

    return strings;
}
