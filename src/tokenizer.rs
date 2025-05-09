
use crate::token;
use crate::token::Keyword;

pub(crate) struct Tokenizer{
    pub(crate) tokens_collection: Vec<token::Token>,
}
//SeLeCT user
impl Tokenizer{
    pub (crate) fn split_query(query: String) -> Vec<String> {
        let mut splitted_tokens = Vec::new();
        let mut statement = String::new();
        let mut query_splitted_iter = query.chars().peekable();
        let mut conditionalSt = String::new();
        while let Some(tokenForLoop) = query_splitted_iter.next() //process of forming of the proper vector for tokinization
        {
            match (tokenForLoop)
            {
                '+' => splitted_tokens.push( String::from("+")),
                '-' => splitted_tokens.push( String::from("-")),
                '*' => splitted_tokens.push( String::from("*")),
                '/' => splitted_tokens.push( String::from("/")),
                '(' => splitted_tokens.push( String::from("(")),
                ')' => splitted_tokens.push( String::from(")")),
                '>' => {
                    conditionalSt.push('>');
                    if(!statement.is_empty())
                    {
                        splitted_tokens.push(statement.clone());
                        statement = String::new();
                    }
                },
                '<' => {
                    conditionalSt.push('<');
                    if(!statement.is_empty())
                    {
                        splitted_tokens.push(statement.clone());
                        statement = String::new();
                    }
                },
                '=' => {
                    conditionalSt.push('=');
                    if(!statement.is_empty())
                    {
                        splitted_tokens.push(statement.clone());
                        statement = String::new();
                    }
                },
                '!' => {
                    conditionalSt.push('!');
                    if(!statement.is_empty())
                    {
                        splitted_tokens.push(statement.clone());
                        statement = String::new();
                    }
                },
                _ => {
                    if(tokenForLoop == ' ' || tokenForLoop == ',' || tokenForLoop == ';')
                    {
                        if(!statement.is_empty())
                        {
                            splitted_tokens.push(statement.clone());
                            statement = String::new();
                        }
                        match tokenForLoop {
                            ',' => splitted_tokens.push( String::from(",")),
                            ';' => splitted_tokens.push( String::from(";")),
                            _ => ()
                        }

                        continue;
                    }
                    else
                    {
                        statement.push(tokenForLoop);
                        if(!conditionalSt.is_empty())
                        {
                            splitted_tokens.push(conditionalSt.clone());
                            conditionalSt = String::new();
                        }
                    }
                }
            }
        }
        return splitted_tokens;
    }
    pub (crate) fn tokenize_string(&mut self, query: String){

        let mut splitted_token_iter;
        let mut shouldBeStated = false;

        splitted_token_iter = Tokenizer::split_query(query).into_iter().peekable();
        while let Some(tokenCollectionItem) = splitted_token_iter.next()
        {
            match tokenCollectionItem.to_ascii_lowercase().as_str(){
                //Default keywords
                "select" => {
                    self.tokens_collection.push(token::Token::Keyword(Keyword::Select));
                    shouldBeStated = true;
                },
                "create" => {
                    self.tokens_collection.push(token::Token::Keyword(Keyword::Create));
                    shouldBeStated = true;
                },
                "table"=> {
                    self.tokens_collection.push(token::Token::Keyword(Keyword::Table));
                    shouldBeStated = true;
                },
                "where" => {
                    self.tokens_collection.push(token::Token::Keyword(Keyword::Where));
                    shouldBeStated = true;
                },
                "order" => self.tokens_collection.push(token::Token::Keyword(Keyword::Order)),
                "by" => {
                    self.tokens_collection.push(token::Token::Keyword(Keyword::By));
                    shouldBeStated = true;
                },
                "asc"=> self.tokens_collection.push(token::Token::Keyword(Keyword::Asc)),
                "desc" => self.tokens_collection.push(token::Token::Keyword(Keyword::Desc)),
                "from"=> {
                    self.tokens_collection.push(token::Token::Keyword(Keyword::From));
                    shouldBeStated = true;
                },
                "and" => {
                    self.tokens_collection.push(token::Token::Keyword(Keyword::And));
                    shouldBeStated = true;
                },
                "or"=> {
                    self.tokens_collection.push(token::Token::Keyword(Keyword::Or));
                    shouldBeStated = true;
                },
                "not" => {
                    self.tokens_collection.push(token::Token::Keyword(Keyword::Not));
                    shouldBeStated = true;
                },
                "true"=> self.tokens_collection.push(token::Token::Keyword(Keyword::True)),
                "false" => self.tokens_collection.push(token::Token::Keyword(Keyword::False)),
                "primary"=> self.tokens_collection.push(token::Token::Keyword(Keyword::Primary)),
                "key" => self.tokens_collection.push(token::Token::Keyword(Keyword::Key)),
                "check"=> self.tokens_collection.push(token::Token::Keyword(Keyword::Check)),
                "int" => self.tokens_collection.push(token::Token::Keyword(Keyword::Int)),
                "bool"=> self.tokens_collection.push(token::Token::Keyword(Keyword::Bool)),
                "varchar" => self.tokens_collection.push(token::Token::Keyword(Keyword::Varchar)),
                "null" => self.tokens_collection.push(token::Token::Keyword(Keyword::Null)),
                //Special signs
                ";" => self.tokens_collection.push(token::Token::Semicolon),
                "," => {
                    self.tokens_collection.push(token::Token::Comma);
                    shouldBeStated = true;
                },
                ">" => self.tokens_collection.push(token::Token::GreaterThan),
                "<" => self.tokens_collection.push(token::Token::LessThan),
                ">=" => self.tokens_collection.push(token::Token::GreaterThanOrEqual),
                "<=" => self.tokens_collection.push(token::Token::LessThanOrEqual),
                "==" => self.tokens_collection.push(token::Token::Equal),
                "!=" => self.tokens_collection.push(token::Token::NotEqual),
                "(" => self.tokens_collection.push(token::Token::RightParentheses),
                ")" => self.tokens_collection.push(token::Token::LeftParentheses),
                "+" => self.tokens_collection.push(token::Token::Plus),
                "-" => self.tokens_collection.push(token::Token::Minus),
                "*" => self.tokens_collection.push(token::Token::Star),
                "/" => self.tokens_collection.push(token::Token::Divide),
                _  => {
                    if(tokenCollectionItem.parse::<i32>().is_ok()){
                        self.tokens_collection.push(token::Token::Number(tokenCollectionItem.parse().unwrap()));
                        continue;
                    }
                    else if(tokenCollectionItem.contains("\"") || tokenCollectionItem.contains("\'")){
                        self.tokens_collection.push(token::Token::String(tokenCollectionItem.to_string()));
                        continue;
                    }
                    if(!shouldBeStated)
                    {
                        self.tokens_collection.push(token::Token::Invalid(String::from("Invalid token")));
                        continue;
                    }
                    else {
                        shouldBeStated = false;
                        self.tokens_collection.push(token::Token::Identifier(String::from(tokenCollectionItem)));
                        continue;
                    }
                }
            }
        }
        self.tokens_collection.push(token::Token::Eof);
    }
}
#[cfg(test)]
mod all_tests
{
    use crate::tokenizer;
    use super::*;
    #[test]
    fn compare_basic_tokenizer()
    {
        let mut tokenizer_struct = tokenizer::Tokenizer{tokens_collection: Vec::new()};
        tokenizer_struct.tokenize_string(String::from("SELECT one, two FROM users ; "));
        let mut ProperVec = Vec::new();
        ProperVec.push(token::Token::Keyword(Keyword::Select));
        ProperVec.push(token::Token::Identifier(String::from("one")));
        ProperVec.push(token::Token::Comma);
        ProperVec.push(token::Token::Identifier(String::from("two")));
        ProperVec.push(token::Token::Keyword(Keyword::From));
        ProperVec.push(token::Token::Identifier(String::from("users")));
        ProperVec.push(token::Token::Semicolon);
        ProperVec.push(token::Token::Eof);
        for i in 0..ProperVec.len(){
            assert_eq!(tokenizer_struct.tokens_collection[i], ProperVec[i]);
        }
    }
}