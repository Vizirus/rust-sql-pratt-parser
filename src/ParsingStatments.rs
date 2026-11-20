use std::fmt::{Display, Formatter};
use crate::statement::{BinaryOperator, Expression, UnaryOperator};

#[derive(Clone)]
pub (crate) struct AddOperator{
    pub (crate) precedence: i32
}
#[derive(Clone)]
pub (crate) struct SubstractOperator{
    pub (crate) precedence: i32,
}
#[derive(Clone)]

pub (crate) struct DivideOperator{
    pub (crate) precedence: i32,
}
#[derive(Clone)]
pub (crate) struct MultiplyOperator{
    pub (crate) precedence: i32,
}
#[derive(Clone)]
pub (crate) struct GreaterThanOrEqualOperator{
    pub (crate) precedence: i32,
}
#[derive(Clone)]
pub (crate) struct LessThanOrEqualOperator{
    pub (crate) precedence: i32,
}
#[derive(Clone)]
pub (crate) struct GreaterThanOperator{
    pub (crate) precedence: i32,
}
#[derive(Clone)]
pub (crate) struct LessThanOperator{
    pub (crate) precedence: i32,
}
pub (crate) struct EqualOperator{}
#[derive(Clone)]
pub (crate) struct NotEqualOperator{}
#[derive(Clone)]
pub (crate) struct RightParent{
    pub (crate) precedence: i32
}
#[derive(Clone)]
pub (crate) struct LeftParent{
    pub (crate) precedence: i32
}
#[derive(Clone)]
pub (crate) struct AscOp{
    pub (crate) precedence: i32
}
#[derive(Clone)]
pub (crate) struct DescOp{
    pub (crate) precedence: i32
}
#[derive(Clone)]
pub (crate) struct And{
    pub (crate) precedence: i32
}
#[derive(Clone)]
pub (crate) struct Or{
    pub (crate) precedence: i32
}
#[derive(Clone)]
//Number Implementation
pub (crate) struct Number{
    pub (crate) number: u64,
}
impl Number{
    pub (crate) fn new(mut self, number: u64){
        self.number = number;
    }
    pub (crate) fn prefixExpression(self) -> Expression{
        return Expression::Number(self.number);
    }
}
impl Display for Number{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result{
        write!(f, "{}", self.number)
    }
}
//Identifier implementation
#[derive(Clone)]
pub (crate) struct Identifier{
    tag: String,
}
impl Identifier{
    pub (crate) fn new(Tag: String) -> Identifier{
        Self{tag:Tag}
    }
    pub (crate) fn prefixExpression(self) -> Expression{
        return Expression::Identifier(self.tag);
    }
}
impl Display for Identifier{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result{
        write!(f, "\n{}", self.tag)
    }
}
impl AddOperator{
    pub (crate) fn new() -> AddOperator{
        Self{precedence: 20}
    }
    pub (crate) fn infix_operator(&mut self, left: Box<Expression>, mut parser: crate::parser::Parser) -> Expression{
        return Expression::BinaryOperation {
            left_operand: left,
            operator: BinaryOperator::Plus,
            right_operand: Box::new(parser.buildExpression(self.precedence).unwrap()),
        }
    }
}
impl SubstractOperator{
    pub (crate) fn new() -> SubstractOperator{
        Self{precedence: 20}
    }
    pub (crate) fn infix_operator(&mut self, left: Box<Expression>, mut parser: crate::parser::Parser) -> Expression{
        return Expression::BinaryOperation {
            left_operand: left,
            operator: BinaryOperator::Minus,
            right_operand: Box::new(parser.buildExpression(self.precedence).unwrap()),
        }
    }
}

impl MultiplyOperator{
    pub (crate) fn new() -> MultiplyOperator{
        Self{precedence: 22}
    }
    pub (crate) fn infix_operator(&mut self, left: Box<Expression>, mut parser: crate::parser::Parser) -> Expression{
        return Expression::BinaryOperation {
            left_operand: left,
            operator: BinaryOperator::Multiply,
            right_operand: Box::new(parser.buildExpression(self.precedence).unwrap()),
        }
    }
}

impl DivideOperator{
    pub (crate) fn new() -> DivideOperator{
        Self{precedence: 22}
    }
    pub (crate) fn infix_operator(&mut self, left: Box<Expression>,mut parser: crate::parser::Parser) -> Expression{
        return Expression::BinaryOperation {
            left_operand: left,
            operator: BinaryOperator::Divide,
            right_operand: Box::new(parser.buildExpression(self.precedence).unwrap()),
        }
    }
}
impl GreaterThanOrEqualOperator{
    pub (crate) fn new() -> GreaterThanOrEqualOperator{
        Self{precedence: 15}
    }
    pub (crate) fn infix_operator(&mut self, left: Box<Expression>, mut parser: crate::parser::Parser) -> Expression{
        return Expression::BinaryOperation {
            left_operand: left,
            operator: BinaryOperator::GreaterThanOrEqual,
            right_operand: Box::new(parser.buildExpression(self.precedence).unwrap()),
        }
    }
}
impl LessThanOrEqualOperator{
    pub (crate) fn new() -> LessThanOrEqualOperator{
        Self{precedence: 15}
    }
    pub (crate) fn infix_operator(&mut self, left: Box<Expression>, mut parser: crate::parser::Parser) -> Expression{
        return Expression::BinaryOperation {
            left_operand: left,
            operator: BinaryOperator::LessThanOrEqual,
            right_operand: Box::new(parser.buildExpression(self.precedence).unwrap()),
        }
    }
}
impl GreaterThanOperator{
    pub (crate) fn new() -> GreaterThanOperator{
        Self{precedence: 15}
    }
    pub (crate) fn infix_operator(&mut self, left: Box<Expression>, mut parser: crate::parser::Parser) -> Expression{
        return Expression::BinaryOperation {
            left_operand: left,
            operator: BinaryOperator::GreaterThan,
            right_operand: Box::new(parser.buildExpression(self.precedence).unwrap()),
        }
    }
}
impl LessThanOperator{
    pub (crate) fn new() -> LessThanOperator{
        Self{precedence: 15}
    }
    pub (crate) fn infix_operator(&mut self, left: Box<Expression>, mut parser: crate::parser::Parser) -> Expression{
        return Expression::BinaryOperation {
            left_operand: left,
            operator: BinaryOperator::LessThan,
            right_operand: Box::new(parser.buildExpression(self.precedence).unwrap()),
        }
    }
}
impl RightParent{
    pub (crate) fn new() -> RightParent{
        Self{
            precedence: 0
        }
    }
}
impl LeftParent{
    pub (crate) fn new() -> LeftParent{
        Self{precedence: 0}
    }
}
impl AscOp{
    pub (crate) fn new() -> AscOp{
        Self{precedence: 5}
    }
    pub (crate) fn infix_operator(&mut self, left: Box<Expression>, mut parser: crate::parser::Parser) -> Expression{
        Expression::UnaryOperation {
            operator: UnaryOperator::Desc,
            operand: left,
        }
    }
}
impl DescOp{
    pub (crate) fn new() -> DescOp{
        Self{precedence: 5}
    }
    pub (crate) fn infix_operator(&mut self, left: Box<Expression>, mut parser: crate::parser::Parser) -> Expression{
        Expression::UnaryOperation {
            operator: UnaryOperator::Desc,
            operand: left,
        }
    }
}
impl And{
    pub (crate) fn new() -> And{
        Self{precedence: 10}
    }
    pub (crate) fn infix_operator(&mut self, left: Box<Expression>,mut parser: crate::parser::Parser) -> Expression{
        return Expression::BinaryOperation {
            left_operand: left,
            operator: BinaryOperator::And,
            right_operand: Box::new(parser.buildExpression(self.precedence).unwrap()),
        }
    }
}
impl Or{
    pub (crate) fn new() -> Or{
        Self{precedence: 5}
    }
    pub (crate) fn infix_operator(&mut self, left: Box<Expression>,mut parser: crate::parser::Parser) -> Expression{
        return Expression::BinaryOperation {
            left_operand: left,
            operator: BinaryOperator::Or,
            right_operand: Box::new(parser.buildExpression(self.precedence).unwrap()),
        }
    }
}