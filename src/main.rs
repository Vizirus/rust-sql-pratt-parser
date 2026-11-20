use std::any::Any;
use std::fmt::{Display, Formatter};
use std::io::stdin;
use std::ops::Add;
use crate::parser::{ParseSQL, Parser};

mod statement;
mod token;
mod parser;
mod tokenizer;
mod ParsingStatments;
mod Tests;

fn main() {
    let mut tokenizer_struct = tokenizer::Tokenizer{tokens_collection: Vec::new()};
    println!("====== Hello! Welcome to the basic SQL Parser, would you like to start? (y/n) ( ======");
    let mut result = String::new();
    stdin().read_line(&mut result).expect("Did not enter a correct string");
    match result.as_str().trim() {
        "y" => {
            loop{
                println!("===== Before starting. The short description is available. Whould you like to see it?===== \n===== (Write 'y' or simply write an expression to mis this part. If you would like to exit, write 'e') ===== ");
                let mut insideResult = String::new();
                stdin().read_line(&mut insideResult).expect("Did not enter a correct string");
                match insideResult.as_str().trim() {
                    "y" => {
                        println!("===== The short description of application ======");
                        println!("\nThis application uses tokenizer and pratt parser techniques. \nThe keywords (like Select, Create, Asc, Desc, And...) can be written in any syntax (application is syntax free for keywords). \nThe application can convert limited types of queries!");
                        println!("Application can convert Select statement with necessary From and optional Where and Order By keywords.");
                        println!("IMPORTANT NOTE! At this moment application cant conver statements at Where, which includes AND and OR operators (everything after AND won`t be converted).");
                        println!("Also, expression are limited (can convert only, <, >, <=, >=, +, -. *, /). For entering some comparisons (For exaple, Age>15+2) you should write them into braces Age>(15+2)");
                        println!("Order By keyword statement has same restrictions for AND and OR as Where statement!");
                        println!("Create table has basic functionality (no complex tables) with slight restrictions for Check statement (Check involves expressions, so it has same limitations with Where keyword in Select\n");
                    },
                    "e" => {
                        break;
                    }
                    _ => {
                        tokenizer_struct.tokens_collection.clear();

                        tokenizer_struct.tokenize_string(String::from(insideResult.as_str().trim().clone()));
                        match ParseSQL(tokenizer_struct.tokens_collection.clone()){
                            Ok(statements) => {
                                for statement in statements {
                                    println!("{}", format!("{}", statement));

                                }
                            }
                            Err(e) =>{println!("Error occurred while parsing SQL: {}", e)}

                        };
                    }
                }
            }
        }
        "n" => {
            println!("====== Thank you for using Basic SQL Parser tool! ======")
        }
        _ => return Err("The entered symbol is wrong!").unwrap()
    }
}