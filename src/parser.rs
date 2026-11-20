use std::cmp::PartialEq;
use std::fmt::{format, Debug, Display, Formatter};
use std::iter::{Copied, Peekable};
use std::panic::resume_unwind;
use crate::Add;
use std::ptr::null;
use std::thread::{current, yield_now};
use std::vec::IntoIter;
use crate::statement::{BinaryOperator, Constraint, DBType, Expression, Statement, TableColumn, UnaryOperator};
use crate::{statement, token};
use crate::parser::SQLStatementState::{Final};
use crate::token::{Keyword, Token};
use crate::ParsingStatments;
use crate::ParsingStatments::{AddOperator, SubstractOperator, DivideOperator, MultiplyOperator, Number, Identifier, GreaterThanOperator, LessThanOperator, GreaterThanOrEqualOperator, LessThanOrEqualOperator, LeftParent, RightParent, AscOp, DescOp, And, Or};
use crate::*;
use crate::statement::Constraint::{NotNull, PrimaryKey};
use crate::statement::Statement::CreateTable;
use crate::token::Keyword::Check;
use super::*;
enum SQLStatementState{ //States for state machine
    Default,
    Select,
    From,
    Where,
    OrderBy,
    Append,
    Final,
    Create,
    Table,
    TableColumn,
    AppendCreate
}

//Operators

//
#[derive(Clone)]
enum TokenExpressions{
    Default,
    Add(AddOperator),
    Minus(SubstractOperator),
    Divide(DivideOperator),
    Multiply(MultiplyOperator),
    GreaterThan(GreaterThanOperator),
    LessThan(LessThanOperator),
    GreaterThanOrEqual(GreaterThanOrEqualOperator),
    LessThanOrEqual(LessThanOrEqualOperator),
    Number(Number),
    Identifier(Identifier),
    LeftParen(LeftParent),
    RightParen(RightParent),
    Ask(AscOp),
    Desc(DescOp),
    And(And),
    Or(Or),
}
impl Display for TokenExpressions{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
//Parser Implementation
#[derive(Clone)]
pub (crate) struct Parser{
    Tokens: Vec<TokenExpressions>,
    tokens: Peekable<IntoIter<TokenExpressions>>,
    pub (crate) current: Option<TokenExpressions>,
}
impl Parser {
    pub(crate) fn new() -> Self{
        Self{
            Tokens: Vec::new(),
            tokens: Vec::new().into_iter().peekable(),
            current: Some(TokenExpressions::Default),
        }
    }
    fn advanceToken(&mut self){
        self.current = self.tokens.next();
    }
    pub (crate) fn tokenize(&mut self, mut iter: &mut Peekable<IntoIter<Token>>){ // This function retokenizes tokens into new one, which are siutable for Pratt

        while let Some(token) = iter.next(){
            match token {
                Token::Number(number) => self.Tokens.push(TokenExpressions::Number(Number {number: number})),
                Token::Identifier(str) => self.Tokens.push(TokenExpressions::Identifier(Identifier::new(str))),
                Token::Plus => self.Tokens.push(TokenExpressions::Add(AddOperator::new())) ,
                Token::Minus => self.Tokens.push(TokenExpressions::Minus(SubstractOperator::new())),
                Token::Divide => self.Tokens.push(TokenExpressions::Divide(DivideOperator::new())),
                Token::Star => self.Tokens.push(TokenExpressions::Multiply(MultiplyOperator::new())),
                Token::GreaterThanOrEqual => self.Tokens.push(TokenExpressions::GreaterThanOrEqual(GreaterThanOrEqualOperator::new())),
                Token::LessThanOrEqual => self.Tokens.push(TokenExpressions::LessThanOrEqual(LessThanOrEqualOperator::new())),
                Token::GreaterThan => self.Tokens.push(TokenExpressions::GreaterThan(GreaterThanOperator::new())),
                Token::LessThan => self.Tokens.push(TokenExpressions::LessThan(LessThanOperator::new())),
                Token::LeftParentheses => self.Tokens.push(TokenExpressions::LeftParen(LeftParent::new())),
                Token::RightParentheses => self.Tokens.push(TokenExpressions::RightParen(RightParent::new())),
                Token::Keyword(Keyword::Asc) => self.Tokens.push((TokenExpressions::Ask(AscOp::new()))),
                Token::Keyword(Keyword::Desc) => self.Tokens.push((TokenExpressions::Desc(DescOp::new()))),
                Token::Keyword(Keyword::And) => self.Tokens.push(TokenExpressions::And(And::new())),
                Token::Keyword(Keyword::Or) => self.Tokens.push(TokenExpressions::Or(Or::new())),
                Token::Semicolon => { // Finish Tokenizing on Semicolon and Key word Order
                    self.tokens = std::mem::take(&mut self.Tokens).into_iter().peekable();
                    return;
                },
                Token::Keyword(Keyword::Order) => {
                    self.tokens = std::mem::take(&mut self.Tokens).into_iter().peekable();
                    return;
                },

                _ => break
            }
        }
        println!("The wrong token occured!");
    }
    pub (crate) fn get_precedence(token: Option<TokenExpressions>) -> i32 { // Get indexes
        match token {
            Some(TokenExpressions::Add(_)) => 20,
            Some(TokenExpressions::Minus(_)) => 20,
            Some(TokenExpressions::Multiply(_)) => 25,
            Some(TokenExpressions::Divide(_)) => 25,
            Some(TokenExpressions::And(_)) => 10,
            Some(TokenExpressions::Or(_)) => 5,
            Some(TokenExpressions::GreaterThanOrEqual(_)) | Some(TokenExpressions::GreaterThan(_)) | Some(TokenExpressions::LessThan(_)) | Some(TokenExpressions::LessThanOrEqual(_))  => 15,
            _ => 0,
        }
    }
    pub fn buildExpression(&mut self, precedence: i32) -> Option<Expression> {
        //let mut token = self.tokens.next();
        let mut token = self.current.clone(); // Fixing the token in memory
        self.advanceToken(); //Switching to the next token
        let mut left = match token { // Check if it`s correct token
            Some(TokenExpressions::Number(n)) => n.prefixExpression(),
            Some(TokenExpressions::Identifier(id))=> id.prefixExpression(),
            Some(TokenExpressions::Minus(_)) => {
                let expr = self.buildExpression(21)?;
                Expression::UnaryOperation {
                    operator: UnaryOperator::Minus,
                    operand: Box::new(expr),
                }
            },
            Some(TokenExpressions::LeftParen(op)) => {
                let expr = self.buildExpression(0).unwrap();
                match self.current {
                    Some(TokenExpressions::RightParen(_)) => {
                        self.advanceToken();

                    },

                    _ => { panic!("Expected closing parenthesis");}
                }
                expr
            },
            other => {
                panic!("{}", other?)
            },
        };

        while let peek = self.current.clone() { //Main loop, recoursevly diving deeply into tokens, to get a proper result
            let token_precedence = Parser::get_precedence(peek.clone());
            if token_precedence <= precedence{
                break;
            }
            token = self.current.clone();
            self.advanceToken();
            left = match token.clone() { //Executing infix_operation to form Binary expretion
                Some(TokenExpressions::Add(mut op)) => Some(op.infix_operator(Box::new(left), self.clone())).unwrap(),
                Some(TokenExpressions::Minus(mut op)) => Some(op.infix_operator(Box::new(left), self.clone())).unwrap(),
                Some(TokenExpressions::Multiply(mut op)) => Some(op.infix_operator(Box::new(left), self.clone())).unwrap(),
                Some(TokenExpressions::Divide(mut op)) => Some(op.infix_operator(Box::new(left), self.clone())).unwrap(),
                Some(TokenExpressions::GreaterThanOrEqual(mut op)) => Some(op.infix_operator(Box::new(left), self.clone())).unwrap(),
                Some(TokenExpressions::GreaterThan(mut op)) => Some(op.infix_operator(Box::new(left), self.clone())).unwrap(),
                Some(TokenExpressions::LessThanOrEqual(mut op))=> Some(op.infix_operator(Box::new(left), self.clone())).unwrap(),
                Some(TokenExpressions::LessThan(mut op)) => Some(op.infix_operator(Box::new(left), self.clone())).unwrap(),
                Some(TokenExpressions::And(mut op)) => Some(op.infix_operator(Box::new(left), self.clone())).unwrap(),
                Some(TokenExpressions::Or(mut op)) => Some(op.infix_operator(Box::new(left), self.clone())).unwrap(),
                Some(TokenExpressions::Ask(mut op)) => Some(op.infix_operator(Box::new(left), self.clone())).unwrap(),
                Some(TokenExpressions::Desc(mut op)) => Some(op.infix_operator(Box::new(left), self.clone())).unwrap(),
                _ => break,
            };
            self.current = self.tokens.peek().cloned();
        }
        Some(left)
    }
    pub (crate) fn parseExpression(&mut self) -> Option<Expression> {
        self.advanceToken();
        return self.buildExpression(0);
    }
}

enum tableColumnStage{
    Name,
    DBType,
    Constraint,
    Final
}
struct selectSignature {
    columns: Vec<Expression>,
    from: Option<Expression>,
    r#where: Option<Expression>,
    orderby: Vec<Expression>,
}
struct createSignature {
    table_name: String,
    column_list: Vec<TableColumn>,
}
pub (crate) fn ParseSQL(tokenVec: Vec<Token>) -> Result<Vec<Statement>, String> //Main SQL parser
{
    let mut resultVector = Vec::new();
    let mut tokens = tokenVec.into_iter().peekable();
    let mut currentToken = tokens.next();
    let mut signature = selectSignature {columns: Vec::new(), from: None, r#where: None, orderby: Vec::new()};
    let mut createSignature = createSignature {
        column_list: Vec::new(),
        table_name: String::new(),
    };
    let mut state = SQLStatementState::Default;
    loop {
        match state { // State machine, switching from state to state until finish
            SQLStatementState::Default => {
                match currentToken { // depending on the command, switching de branch of state machine
                    Some(Token::Keyword(Keyword::Select)) => {
                        state = SQLStatementState::Select;
                    }
                    Some(Token::Keyword(Keyword::Create)) =>{
                        state = SQLStatementState::Create;
                    }
                    _ => Err("Unexpected token in select".to_string())?,
                }
            }
            SQLStatementState::Select => { //If command is select, looking for all columns and add them to signature
                currentToken = tokens.next();
                match currentToken.clone() {
                    Some(Token::Identifier(statement)) => signature.columns.push(Expression::Identifier(statement.clone())),
                    Some(Token::Comma) => continue,
                    Some(Token::Keyword(Keyword::From)) => state = SQLStatementState::From,
                    _ => Err(format!("Unexpected token! Expected Identifier (column1, column2), but was {}", currentToken.clone().unwrap().to_string()))?,
                }
            },
            SQLStatementState::From => { //If command is From, looking for the table
                currentToken = tokens.next();
                match currentToken.clone() {
                    Some(Token::Identifier(statement)) => signature.from = Some(Expression::Identifier(statement.clone())),
                    Some(Token::Keyword(Keyword::Where)) => state = SQLStatementState::Where,
                    Some(Token::Semicolon) => state = SQLStatementState::Append,
                    _ => Err(format!("Unexpected token! Expected Identifier (tableName), but was {}", currentToken.clone().unwrap().to_string()))?
                }
            },
            SQLStatementState::Where => {// If command is Where, looking for all Expretions
                let mut prattParser = Parser::new();
                prattParser.tokenize(&mut tokens);
                let mut result = prattParser.parseExpression();
                signature.r#where = result;
                currentToken = tokens.next();
                match currentToken.clone() {
                    Some(Token::Eof) => state = SQLStatementState::Append,
                    Some(Token::Keyword(Keyword::By)) => state = SQLStatementState::OrderBy,
                    None => state = SQLStatementState::Append,
                    _ => return Err("Unexpected token! Expected EOF, Order By".to_string())?,
                }
            },
            SQLStatementState::OrderBy => { //If command is Order By, looking for all statements
                let mut prattParser = Parser::new();
                prattParser.tokenize(&mut tokens);
                let mut result = Vec::new();
                result.push(prattParser.parseExpression().unwrap());
                signature.orderby = result;
                currentToken = tokens.next();
                match currentToken.clone() {
                    Some(Token::Eof) => state = SQLStatementState::Append,
                    _ => return Err("Unexpected token! Expected EOF, Order By".to_string()).unwrap(),
                }
            },
            SQLStatementState::Create => {
                state = SQLStatementState::Table;
                currentToken = tokens.next();
            }
            SQLStatementState::Table => { //If command is create, looking fortable name
                currentToken = tokens.next();
                match currentToken.clone().unwrap() {
                    Token::Identifier(str) => {
                        createSignature.table_name = str.clone();
                        let peekedVal = tokens.peek();
                        match peekedVal.clone() {
                            Some(Token::LeftParentheses) => {
                                state = SQLStatementState::TableColumn;
                                currentToken = tokens.next();
                            }
                            _ => return Err("Unexprexted Token! The LeftParentheses were not found".to_string())?,
                        }
                    }
                    _ => return Err("Unexpected token! Expected Identifier".to_string())?,
                }
            }
            SQLStatementState::TableColumn => { // Working on DB type
                let mut columnStructure = TableColumn {
                    column_name: String::new(),
                    column_type: DBType::Int,
                    constraints: Vec::new(),
                };
                let mut stateC ;
                stateC = tableColumnStage::Name;
                loop{
                    currentToken = tokens.next();
                    match stateC{
                        tableColumnStage::Name => {
                            match currentToken.clone().unwrap() {
                                Token::Identifier(str) => {
                                    columnStructure.column_name = str.clone();
                                    stateC = tableColumnStage::DBType;
                                }
                                _ => return Err(format!("Expected Identifier, but was {}", currentToken.clone().unwrap().to_string()))?
                            }
                        }
                        tableColumnStage::DBType =>{
                            match currentToken.clone().unwrap() {
                                Token::Keyword(Keyword::Int) => {
                                    columnStructure.column_type = DBType::Int;
                                    let last = tokens.peek();
                                    createSignature.column_list.push(columnStructure.clone());
                                    match last.clone().unwrap() {
                                        Token::RightParentheses => {
                                            stateC = tableColumnStage::Final;
                                        }
                                        Token::Comma => {
                                            stateC = tableColumnStage::Name
                                        }
                                        Token::Keyword(Keyword::Primary) | Token::Keyword(Keyword::Check) | Token::Keyword(Keyword::Not) => {
                                            stateC = tableColumnStage::Constraint;
                                        }
                                        _ => return Err(format!("Expected Semicolon, but was {}", currentToken.clone().unwrap().to_string()))?
                                    }
                                },
                                Token::Keyword(Keyword::Varchar) => {
                                    let size = match tokens.next().unwrap() {
                                        Token::Number(num) => num,
                                        _ => return Err(format!("Expected Number, but was {}", currentToken.clone().unwrap().to_string()))?
                                    };
                                    columnStructure.column_type = DBType::Varchar(size);
                                    let last = tokens.next();
                                    createSignature.column_list.push(columnStructure.clone());
                                    match last.clone().unwrap() {
                                        Token::RightParentheses => {
                                            stateC = tableColumnStage::Final;
                                        }
                                        Token::Comma => {
                                            stateC = tableColumnStage::Name
                                        }
                                        _ => return Err(format!("Expected Semicolon, but was {}", currentToken.clone().unwrap().to_string()))?
                                    }
                                },
                                Token::Keyword(Keyword::Bool) => {
                                    columnStructure.column_type = DBType::Bool;
                                    let last = tokens.peek();
                                    createSignature.column_list.push(columnStructure.clone());
                                    match last.clone().unwrap() {
                                        Token::RightParentheses => {
                                            stateC = tableColumnStage::Final;
                                        }
                                        Token::Semicolon => {
                                            stateC = tableColumnStage::Name
                                        }
                                        _ => return Err(format!("Expected Semicolon, but was {}", currentToken.clone().unwrap().to_string()))?
                                    }
                                },
                                _ => return Err(format!("Expected Identifier, but was {}", currentToken.clone().unwrap().to_string()))?
                            }
                        }
                        tableColumnStage::Constraint => { // Adding the constraints
                            match currentToken.clone().unwrap() {
                                Token::Keyword(Keyword::Primary) => {
                                    match tokens.next().unwrap() {
                                        Token::Keyword(Keyword::Key) => {
                                            columnStructure.constraints.push(PrimaryKey);
                                            createSignature.column_list.push(columnStructure.clone());
                                            columnStructure.constraints = Vec::new();
                                            match tokens.next().unwrap(){
                                                Token::Comma =>{
                                                    stateC = tableColumnStage::Name;
                                                },
                                                Token::RightParentheses => {
                                                    stateC = tableColumnStage::Final;
                                                }
                                                _ => return Err(format!("Expected Comma, but was {}", currentToken.clone().unwrap().to_string()))?
                                            }
                                        }
                                        _ => return Err(format!("Expected Keyword but was {}", currentToken.clone().unwrap().to_string()))?
                                    }

                                },
                                Token::Keyword(Keyword::Not) => {
                                    match tokens.next().unwrap() {
                                        Token::Keyword(Keyword::Null) =>{
                                            columnStructure.constraints.push(NotNull);
                                            createSignature.column_list.push(columnStructure.clone());
                                            columnStructure.constraints = Vec::new();
                                            match tokens.next().unwrap(){
                                                Token::Comma =>{
                                                    stateC = tableColumnStage::Name;
                                                },
                                                Token::RightParentheses => {
                                                    stateC = tableColumnStage::Final;
                                                }
                                                _ => return Err(format!("Expected Comma, but was {}", currentToken.clone().unwrap().to_string()))?
                                            }
                                        }
                                        _ => return Err(format!("Expected Null after Noy, but was {}", currentToken.clone().unwrap().to_string()))?
                                    }
                                    columnStructure.constraints.push(PrimaryKey);
                                },
                                Token::Keyword(Keyword::Check) =>{
                                    currentToken = tokens.next();
                                    let mut smallVecTok: Vec<Token> = Vec::new();
                                    match currentToken.clone().unwrap() {
                                        Token::LeftParentheses =>{
                                            while let mut i = 0{
                                                i += 1;
                                                if(i >= 15){
                                                    break;
                                                }
                                                currentToken = tokens.next();
                                                match currentToken.clone().unwrap() {
                                                    Token::RightParentheses => {
                                                        break;
                                                    }
                                                    _ => smallVecTok.push(currentToken.unwrap())
                                                }

                                            }
                                            smallVecTok.push(Token::Semicolon);
                                            let mut prattParser = Parser::new();
                                            prattParser.tokenize(&mut smallVecTok.into_iter().peekable());
                                            columnStructure.constraints.push(Constraint::Check(prattParser.parseExpression().unwrap()));
                                            createSignature.column_list.push(columnStructure.clone());
                                            match tokens.next().unwrap(){
                                                Token::Comma =>{
                                                    stateC = tableColumnStage::Name;
                                                },
                                                Token::Semicolon => {
                                                    stateC = tableColumnStage::Final;
                                                },
                                                _ => return Err("Expected Comma, but was unknwo sign")?
                                            }
                                        }
                                        _ => return Err(format!("Expected LeftParentheses, but was {}", currentToken.clone().unwrap().to_string()))?
                                    }

                                }
                                _ => return Err(format!("Expected PrimaryKey or Not null or Check, but was {}", currentToken.clone().unwrap().to_string()))?
                            }
                        }
                        tableColumnStage::Final => {
                            state = SQLStatementState::AppendCreate;
                            break;
                        }
                    }
                }
            }
            SQLStatementState::AppendCreate =>{ //Append if Create was chosen
                resultVector.push(CreateTable {
                    column_list: createSignature.column_list.clone(),
                    table_name: createSignature.table_name.clone(),
                });
                match tokens.peek() {
                    Some(Token::Keyword(Keyword::Select)) => {
                        state = SQLStatementState::Default;
                        currentToken = tokens.next();
                    },
                    Some(Token::Keyword(Keyword::Create)) => {
                        state = SQLStatementState::Default;
                        currentToken = tokens.next();
                    },
                    _ => {
                        state = SQLStatementState::Final;
                    },
                }
            }
            SQLStatementState::Append => { // Append is select was chosen
                resultVector.push(Statement::Select {
                    columns: std::mem::take(&mut signature.columns),
                    from: std::mem::take(&mut signature.from).unwrap(),
                    r#where: std::mem::take(&mut signature.r#where),
                    orderby: std::mem::take(&mut signature.orderby)
                });
                match tokens.peek() {
                    Some(Token::Keyword(Keyword::Select)) => {
                        state = SQLStatementState::Default;
                        currentToken = tokens.next();
                    },
                    Some(Token::Keyword(Keyword::Create)) => {
                        state = SQLStatementState::Default;
                        currentToken = tokens.next();
                    },
                    _ => {
                        state = SQLStatementState::Final;
                    },
                }
            },
            SQLStatementState::Final => { //Finish the code
                return Ok(resultVector);
            },
        }
    }
}
