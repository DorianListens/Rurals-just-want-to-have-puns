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
    let json = get_json_array(&word);

    let rhymes = json.iter()
        .filter_map(decode_rhyme)
        .filter(|rhyme| rhyme.score == 300)
        .collect::<Vec<Rhyme>>();

    let phrases = collect_phrases();

    let puns:Vec<Pun> = phrases.iter()
        .filter_map(|phrase| return make_pun(&rhymes, &word, phrase))
        .collect();

    for pun in &puns {
        pun.print();
    }
}

fn get_json_array(word:&String) -> Vec<Json> {
    if let Some(json) = fetch_json(word) {
        if let Some(json_array) = json.as_array() {
            return json_array.to_owned();
        }
    }
    return vec![];
}

fn fetch_json(word:&String) -> Option<Json> {
    let url = format!("http://rhymebrain.com/talk?function=getRhymes&word={}", word);
    if let Ok(resp) = http::handle().get(url).exec() {
        if let Ok(data) = str::from_utf8(resp.get_body()) {
            if let Ok(json) = Json::from_str(&data) {
                return Some(json)
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

fn collect_phrases() -> Vec<String> {
    let path = Path::new("./phrases/");
    let mut strings_to_return : Vec<String> = Vec::new();
    if let Ok(file_names) = fs::read_dir(path) {
        for file_name in file_names {
            if let Ok(file_name) = file_name {
                if let Ok(file) = File::open(file_name.path()) {
                    let strings = collect_strings_from_file(file);
                    for string in strings {
                        strings_to_return.push(string);
                    }
                }
            }
        }
    }
    return strings_to_return;
}

fn collect_strings_from_file(mut file:File) -> Vec<String> {
    let mut s = String::new();
    let result = file.read_to_string(&mut s);
    return s.split("\n").map(|s| s.to_string()).collect::<Vec<String>>();
}

fn make_pun(rhymes: &Vec<Rhyme>, word: &String, string:&String) -> Option<Pun> {
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

