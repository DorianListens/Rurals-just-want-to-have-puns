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

impl Rhyme {
   pub fn new(word:&str, freq:i32, score:i32, flags:&str, syllables: i32) -> Rhyme {
        Rhyme {
            word: word.to_string(),
            freq: freq,
            score: score,
            flags: flags.to_string(),
            syllables: syllables
        }
    }
}
