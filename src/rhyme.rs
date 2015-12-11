#[derive(RustcDecodable, RustcEncodable)]
pub struct Rhyme {
    pub word: String,
    freq: i32,
    pub score: i32,
    flags: String,
    syllables: i32
}

pub struct Pun {
    pub original: String,
    pub pun: String
}

impl Pun {
    pub fn new(original:&str, pun: &str) -> Pun {
        Pun {
            original: original.to_string(),
            pun: pun.to_string()
        }
    }
}

