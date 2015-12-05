mod rhyme;
use rhyme::*;
fn main() {
    let rhyme = Rhyme::new("Heart", 100, 300, "bc", 1);
    println!("the word is: {}", rhyme.word);
}
