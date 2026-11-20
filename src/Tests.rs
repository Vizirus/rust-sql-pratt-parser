use crate::*;
#[cfg(test)]

mod all_tests
{
    use crate::token::{Keyword, Token};
    use crate::*;
    use crate::parser::Parser;
    use crate::tokenizer::Tokenizer;
    use super::*;
    #[test]
    /*fn compare_basic_tokenizer()
    {
        let mut tokenizer_struct = tokenizer::Tokenizer{tokens_collection: Vec::new()};
        tokenizer_struct.tokenize_string(String::from("SELECT one, two FROM users WHERE three>-4; "));
        let mut ProperVec = Vec::new();
        ProperVec.push(token::Token::Keyword(Keyword::Select));
        ProperVec.push(token::Token::Identifier(String::from("one")));
        ProperVec.push(token::Token::Comma);
        ProperVec.push(token::Token::Identifier(String::from("two")));
        ProperVec.push(token::Token::Keyword(Keyword::From));
        ProperVec.push(token::Token::Identifier(String::from("users")));
        ProperVec.push(token::Token::Keyword(Keyword::Where));
        ProperVec.push(token::Token::Identifier(String::from("three")));
        ProperVec.push(token::Token::GreaterThan);
        ProperVec.push(token::Token::Minus);
        ProperVec.push(token::Token::Number(4));
        ProperVec.push(token::Token::Semicolon);
        ProperVec.push(token::Token::Eof);
        for i in 0..ProperVec.len(){
            assert_eq!(tokenizer_struct.tokens_collection[i], ProperVec[i]);
        }
    }*/
    #[test]
    fn checkPrattParser(){
        let mut tokenCollection = Vec::new();
        tokenCollection.push(Token::LeftParentheses);
        tokenCollection.push(token::Token::Identifier("One".to_string()));
        tokenCollection.push(token::Token::Plus);
        tokenCollection.push(token::Token::Number(34));
        tokenCollection.push(token::Token::RightParentheses);
        tokenCollection.push(token::Token::Star);
        tokenCollection.push(token::Token::Identifier("Two".to_string()));
        tokenCollection.push(token::Token::Semicolon);
        let mut parser = Parser::new();
        //parser.tokenize(tokenCollection);
        //let mut result = parser.parseExpression().unwrap();
        //println!("{:?}", result);
    }
    #[test]
    fn TestCreateTable(){
        let query = "SELECT Test FROM test WHERE Age>(20+2)*5; ";
        let mut tokenizer_struct = tokenizer::Tokenizer{tokens_collection: Vec::new()};
        tokenizer_struct.tokenize_string(String::from(query));
        /*for i in tokenizer_struct.tokens_collection{
            println!("{:?}", i);
        }*/
        match ParseSQL(tokenizer_struct.tokens_collection){
            Ok(statements) => {
                println!("Parsed statements: {:?}", statements);
            }
            Err(e) =>{println!("Error occured while parsing SQL: {}", e)}

        };
        /*let mut parser = Parser::new();
        //parser.tokenize(tokenCollection);
        let mut result = parser.parseExpression().unwrap();
        println!("{:?}", result);*/
    }
}