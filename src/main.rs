use std::io::prelude::*;
mod rhyme;
use rhyme::*;

extern crate regex;
use regex::Regex;
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
    let url = format!("http://rhymebrain.com/talk?function=getRhymes&word={}", word);
    let resp = http::handle().get(url).exec().unwrap(); 

    let data = str::from_utf8(resp.get_body()).unwrap();
    let json = Json::from_str(&data).unwrap();

    let rhymes = json.as_array().unwrap().iter()
        .map(|item| json::decode::<Rhyme>(&item.to_string()).unwrap())
        .filter(|rhyme| rhyme.score == 300)
        .collect::<Vec<Rhyme>>();

    println!("The filtered rhyme list: {}", rhymes.len());

    let mut strings = pull_strings_from_dir();

    let contains_rhyme =  |word : &String| -> bool {
        for rhyme in &rhymes {
            let s: &str = &word;
            let r: &str = &rhyme.word;
            let rstring = format!("\\b{}\\b", r);
            let rstr : &str = &rstring;
            let regex = Regex::new(&rstring).unwrap();
            return regex.is_match(&s);
        };
        return false;
    };

    strings.retain(&contains_rhyme);

    for string in &strings {
        print!("Filtered string: {} \n", string);
    };
    print!("There are this many filtered strings: {}", strings.len())


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
