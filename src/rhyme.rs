#[derive(RustcDecodable, RustcEncodable)]
pub struct Rhyme {
    pub word: String,
    freq: i32,
    pub score: i32,
    flags: String,
    syllables: i32
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
