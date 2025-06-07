use pest_derive::Parser;
use pest::Parser;
use pest::iterators::Pair;
use std::fs;
use std::path::Path;

use crate::ast::{AstNode, Expression, Statement, BinOp};

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct EppParser;

pub fn parse_eppx_file(file_path: &Path) -> Result<Vec<AstNode>, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;
    parse_eppx_string(&content)
}

pub fn parse_eppx_string(input: &str) -> Result<Vec<AstNode>, String> {
    match EppParser::parse(Rule::program, input) {
        Ok(mut pairs) => {
            let mut ast_nodes = Vec::new();
            let program_pair = pairs.next().unwrap();
            for pair in program_pair.into_inner() {
                match pair.as_rule() {
                    Rule::statement => {
                        ast_nodes.push(build_ast_from_statement(pair)?);
                    }
                    _ => {}
                }
            }
            Ok(ast_nodes)
        }
        Err(e) => Err(format!("Parse failed: {}", e)),
    }
}

fn build_ast_from_statement(pair: Pair<Rule>) -> Result<AstNode, String> {
    let inner = pair.into_inner().next().ok_or_else(|| "Empty statement rule".to_string())?;
    match inner.as_rule() {
        Rule::print_statement => {
            let expr_pair = inner.into_inner().next().ok_or_else(|| "Print statement missing expression".to_string())?;
            let expr_node = build_ast_from_expression(expr_pair)?;
            Ok(AstNode::Statement(Statement::Print(Box::new(expr_node))))
        }
        Rule::assignment => {
            let mut inner_rules = inner.into_inner();
            let name = inner_rules.next().unwrap().as_str().to_string();
            let value_expr = build_ast_from_expression(inner_rules.next().unwrap())?;
            Ok(AstNode::Statement(Statement::Assignment {
                name,
                value: Box::new(value_expr),
            }))
        }
        _ => Err(format!("Unknown statement type: {:?}", inner.as_rule())),
    }
}

fn build_ast_from_expression(pair: Pair<Rule>) -> Result<Expression, String> {
    match pair.as_rule() {
        Rule::expression => { // Added to handle explicit expression rule
            // An expression rule directly contains an add_sub (or whatever the top precedence is)
            build_ast_from_expression(pair.into_inner().next().unwrap())
        }
        Rule::comparison => { // New handler for comparison rule
            let mut inner = pair.into_inner();
            let mut left = build_ast_from_expression(inner.next().unwrap())?;
            while let Some(op_pair) = inner.next() {
                let op_str = op_pair.as_str();
                let op = match op_str {
                    "==" => BinOp::Eq,
                    "!=" => BinOp::NotEq,
                    ">" => BinOp::Gt,
                    "<" => BinOp::Lt,
                    ">=" => BinOp::GtEq,
                    "<=" => BinOp::LtEq,
                    _ => return Err(format!("Unknown comparison_op: {}", op_str)),
                };
                let right_pair = inner.next().ok_or_else(|| format!("Missing right operand for {}", op_str))?;
                let right = build_ast_from_expression(right_pair)?;
                left = Expression::BinaryOperation {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            }
            Ok(left)
        }
        Rule::string_literal => {
            let full_str = pair.as_str();
            let content = full_str[1..full_str.len()-1].to_string();
            Ok(Expression::StringLiteral(content))
        }
        Rule::integer_literal => {
            let val = pair.as_str().parse::<i64>().map_err(|e| format!("Invalid integer: {}", e))?;
            Ok(Expression::IntegerLiteral(val))
        }
        Rule::identifier => {
            Ok(Expression::Identifier(pair.as_str().to_string()))
        }
        Rule::add_sub => { // Was Rule::arithmetic
            let mut inner = pair.into_inner();
            let mut left = build_ast_from_expression(inner.next().unwrap())?;
            while let Some(op_pair) = inner.next() {
                let op_str = op_pair.as_str();
                let op = match op_str {
                    "+" => BinOp::Add,
                    "-" => BinOp::Sub,
                    _ => return Err(format!("Unknown add_op: {}", op_str)),
                };
                let right_pair = inner.next().ok_or_else(|| format!("Missing right operand for {}", op_str))?;
                let right = build_ast_from_expression(right_pair)?;
                left = Expression::BinaryOperation {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            }
            Ok(left)
        }
        Rule::mul_div_mod => { // Was Rule::term
            let mut inner = pair.into_inner();
            let mut left = build_ast_from_expression(inner.next().unwrap())?;
            while let Some(op_pair) = inner.next() {
                let op_str = op_pair.as_str();
                let op = match op_str {
                    "*" => BinOp::Mul,
                    "/" => BinOp::Div,
                    "//" => BinOp::FloorDiv,
                    "%" => BinOp::Mod,
                    _ => return Err(format!("Unknown mul_op: {}", op_str)),
                };
                let right_pair = inner.next().ok_or_else(|| format!("Missing right operand for {}", op_str))?;
                let right = build_ast_from_expression(right_pair)?;
                left = Expression::BinaryOperation {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            }
            Ok(left)
        }
        Rule::power => { // New handler for power rule
            let mut inner = pair.into_inner();
            let mut left = build_ast_from_expression(inner.next().unwrap())?;
            while let Some(op_pair) = inner.next() {
                let op_str = op_pair.as_str();
                let op = match op_str {
                    "**" => BinOp::Pow,
                    _ => return Err(format!("Unknown pow_op: {}", op_str)),
                };
                let right_pair = inner.next().ok_or_else(|| format!("Missing right operand for {}", op_str))?;
                let right = build_ast_from_expression(right_pair)?;
                left = Expression::BinaryOperation {
                    left: Box::new(left),
                    op,
                    right: Box::new(right),
                };
            }
            Ok(left)
        }
        Rule::factor => {
            let mut inner = pair.into_inner();
            build_ast_from_expression(inner.next().unwrap())
        }
        _ => Err(format!("Unknown expression type: {:?}", pair.as_rule())),
    }
}

// You will need to define your AST structures and a function
// to convert the pest::iterators::Pairs<Rule> into your AST.
