use std::any::Any;
use std::fmt::{Display, Formatter};
use std::ops::Add;

mod statement;
mod token;
mod parser;
mod tokenizer;
fn main() {
    let mut tokenizer_struct = tokenizer::Tokenizer{tokens_collection: Vec::new()};

    tokenizer_struct.tokenize_string(String::from("SeLeCT one, two FROM Users ; "));
    for i in tokenizer_struct.tokens_collection.iter(){
        println!("{}",i);
    }
}