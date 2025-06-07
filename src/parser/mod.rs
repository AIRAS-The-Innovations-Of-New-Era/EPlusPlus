use pest_derive::Parser;
use pest::Parser;
use pest::iterators::{Pair, Pairs}; // Added Pairs
use std::fs;
use std::path::Path;

use crate::ast::{AstNode, Expression, Statement, BinOp, UnaryOp, AssignmentOperator}; // Added UnaryOp

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct EppParser;

// Renamed from parse_bin_op_recursive
fn build_recursive_ast_from_binary_expr_rule(
    mut pairs: Pairs<Rule>, // Make it mutable
    sub_rule_parser: fn(Pair<Rule>) -> Result<Expression, String>,
    rule_to_op_str_map: &[(Rule, Vec<(&str, BinOp)>)],
    // For rules that are just wrappers around operator strings (like add_op, mul_op)
    direct_op_str_map: Option<&[(&str, BinOp)]>
) -> Result<Expression, String> {
    let mut left = sub_rule_parser(pairs.next().unwrap())?;

    while let Some(op_pair_or_direct_op_str_pair) = pairs.next() {
        let op: BinOp;
        // Check if this level of grammar has a specific op rule (e.g. add_op, mul_op)
        // or if the op string is directly part of the current rule (e.g. comparison_identity_membership)
        if let Some(direct_map) = direct_op_str_map {
             let op_str = op_pair_or_direct_op_str_pair.as_str();
             op = direct_map.iter()
                .find(|(s, _)| *s == op_str)
                .map(|(_, bin_op)| bin_op.clone())
                .ok_or_else(|| format!("Unknown direct operator string: {}", op_str))?;
        } else {
            // Expecting an op_rule here (e.g. logical_or_op, bitwise_and_op)
            let op_rule = op_pair_or_direct_op_str_pair.as_rule(); // This is the <op_type>_op rule
            let op_str_from_rule = op_pair_or_direct_op_str_pair.as_str(); // This is the actual operator string e.g. "or", "&"
            
            op = rule_to_op_str_map
                .iter()
                .find(|(rule, _)| *rule == op_rule) // Match the <op_type>_op rule
                .and_then(|(_, specific_ops)| { // Now find the specific operator string within that rule's possibilities
                    specific_ops.iter().find(|(s, _)| *s == op_str_from_rule)
                })
                .map(|(_, bin_op)| bin_op.clone())
                .ok_or_else(|| format!("Unknown operator: {} for rule {:?}", op_str_from_rule, op_rule))?;
        }
        
        let right_pair = pairs.next().ok_or_else(|| format!("Missing right operand for operator"))?;
        let right = sub_rule_parser(right_pair)?;
        left = Expression::BinaryOperation {
            left: Box::new(left),
            op,
            right: Box::new(right),
        };
    }
    Ok(left)
}

// Renamed from build_ast_from_expression_final
fn build_ast_from_expression(pair: Pair<Rule>) -> Result<Expression, String> {
    match pair.as_rule() {
        Rule::expression | Rule::factor => { 
            let inner_pair = pair.into_inner().next().unwrap();
            build_ast_from_expression(inner_pair) // Recursive call to the renamed function
        }
        Rule::logical_or => {
            build_recursive_ast_from_binary_expr_rule(pair.into_inner(), build_ast_from_expression, &[
                (Rule::logical_or_op, vec![("or", BinOp::Or)])
            ], None)
        }
        Rule::logical_and => {
            build_recursive_ast_from_binary_expr_rule(pair.into_inner(), build_ast_from_expression, &[
                (Rule::logical_and_op, vec![("and", BinOp::And)])
            ], None)
        }
        Rule::logical_not_expr => {
            let mut inner = pair.into_inner();
            let first_child = inner.next().unwrap();
            if first_child.as_rule() == Rule::logical_not_op {
                let operand = build_ast_from_expression(inner.next().unwrap())?;
                Ok(Expression::UnaryOperation { op: UnaryOp::Not, operand: Box::new(operand) })
            } else {
                // No "not", so it's just the comparison_identity_membership part
                build_ast_from_expression(first_child)
            }
        }
        Rule::comparison_identity_membership => {
            build_recursive_ast_from_binary_expr_rule(pair.into_inner(), build_ast_from_expression, &[], Some(&[
                ("==", BinOp::Eq), ("!=", BinOp::NotEq), (">", BinOp::Gt), ("<", BinOp::Lt),
                (">=", BinOp::GtEq), ("<=", BinOp::LtEq), ("is", BinOp::Is),
                ("is not", BinOp::IsNot), ("in", BinOp::In), ("not in", BinOp::NotIn)
            ]))
        }
        Rule::bitwise_or => {
            build_recursive_ast_from_binary_expr_rule(pair.into_inner(), build_ast_from_expression, &[
                (Rule::bitwise_or_op, vec![("|", BinOp::BitOr)])
            ], None)
        }
        Rule::bitwise_xor => {
            build_recursive_ast_from_binary_expr_rule(pair.into_inner(), build_ast_from_expression, &[
                (Rule::bitwise_xor_op, vec![("^", BinOp::BitXor)])
            ], None)
        }
        Rule::bitwise_and => {
            build_recursive_ast_from_binary_expr_rule(pair.into_inner(), build_ast_from_expression, &[
                (Rule::bitwise_and_op, vec![("&", BinOp::BitAnd)])
            ], None)
        }
        Rule::shift => {
            build_recursive_ast_from_binary_expr_rule(pair.into_inner(), build_ast_from_expression, &[
                (Rule::shift_op, vec![("<<", BinOp::LShift), (">>", BinOp::RShift)])
            ], None)
        }
        Rule::add_sub => {
            build_recursive_ast_from_binary_expr_rule(pair.into_inner(), build_ast_from_expression, &[
                (Rule::add_op, vec![("+", BinOp::Add), ("-", BinOp::Sub)])
            ], None)
        }
        Rule::mul_div_mod => {
            build_recursive_ast_from_binary_expr_rule(pair.into_inner(), build_ast_from_expression, &[
                (Rule::mul_op, vec![
                    ("*", BinOp::Mul), ("/", BinOp::Div), 
                    ("//", BinOp::FloorDiv), ("%", BinOp::Mod)
                ])
            ], None)
        }
        Rule::unary_bitwise_not_power => {
            let mut inner = pair.into_inner();
            let first_child = inner.next().unwrap();
            if first_child.as_rule() == Rule::unary_bitwise_not_op { 
                let operand = build_ast_from_expression(inner.next().unwrap())?;
                Ok(Expression::UnaryOperation { op: UnaryOp::BitNot, operand: Box::new(operand) })
            } else {
                build_ast_from_expression(first_child) 
            }
        }
        Rule::power => {
            build_recursive_ast_from_binary_expr_rule(pair.into_inner(), build_ast_from_expression, &[
                (Rule::pow_op, vec![("**", BinOp::Pow)])
            ], None)
        }
        Rule::string_literal => {
            let full_str = pair.as_str();
            let content = full_str[1..full_str.len()-1]
                .replace("\\\\", "\\")
                .replace("\\\"", "\"")
                .replace("\\n", "\n")
                .replace("\\t", "\t");
            Ok(Expression::StringLiteral(content))
        }        Rule::integer_literal => {
            let val = pair.as_str().parse::<i64>().map_err(|e| format!("Invalid integer: {}", e))?;
            Ok(Expression::IntegerLiteral(val))
        }
        Rule::float_literal => {
            let val = pair.as_str().parse::<f64>().map_err(|e| format!("Invalid float: {}", e))?;
            Ok(Expression::FloatLiteral(val))
        }
        Rule::identifier => {
            Ok(Expression::Identifier(pair.as_str().to_string()))
        }
        // Catch-all for rules that should have been handled by `expression` or `factor`'s recursion,
        // or are actual terminals not listed above.
        _ => Err(format!("Unhandled expression rule: {:?}\nContent: '{}'", pair.as_rule(), pair.as_str())),
    }
}

// Renamed from build_ast_from_statement_final
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
            let op_str = inner_rules.next().unwrap().as_str();
            let operator = match op_str {
                "=" => AssignmentOperator::Assign,
                "+=" => AssignmentOperator::AddAssign,
                "-=" => AssignmentOperator::SubAssign,
                "*=" => AssignmentOperator::MulAssign,
                "/=" => AssignmentOperator::DivAssign,
                "%=" => AssignmentOperator::ModAssign,
                "**=" => AssignmentOperator::PowAssign,
                "//=" => AssignmentOperator::FloorDivAssign,
                "&=" => AssignmentOperator::BitAndAssign,
                "|=" => AssignmentOperator::BitOrAssign,
                "^=" => AssignmentOperator::BitXorAssign,
                ">>=" => AssignmentOperator::RShiftAssign,
                "<<=" => AssignmentOperator::LShiftAssign,
                _ => return Err(format!("Unknown assignment operator: {}", op_str)),
            };
            let value_expr = build_ast_from_expression(inner_rules.next().unwrap())?;
            Ok(AstNode::Statement(Statement::Assignment {
                name,
                operator,
                value: Box::new(value_expr),
            }))
        }
        Rule::if_statement => {
            let mut inner_rules = inner.into_inner();
            let condition_expr = build_ast_from_expression(inner_rules.next().unwrap())?;
            let then_block_pair = inner_rules.next().unwrap();
            let then_body = then_block_pair.into_inner().map(build_ast_from_statement).collect::<Result<Vec<_>, _>>()?;
            // Parse zero or more elifs
            let mut elifs = Vec::new();
            while let Some(next) = inner_rules.peek() {
                if next.as_rule() == Rule::elif_clause {
                    let elif_pair = inner_rules.next().unwrap();
                    let mut elif_inner = elif_pair.into_inner();
                    let elif_cond = build_ast_from_expression(elif_inner.next().unwrap())?;
                    let elif_block = elif_inner.next().unwrap();
                    let elif_body = elif_block.into_inner().map(build_ast_from_statement).collect::<Result<Vec<_>, _>>()?;
                    elifs.push((elif_cond, elif_body));
                } else {
                    break;
                }
            }
            // Optional else
            let else_body = if let Some(next) = inner_rules.peek() {
                if next.as_rule() == Rule::else_clause {
                    let else_pair = inner_rules.next().unwrap();
                    let else_block = else_pair.into_inner().next().unwrap();
                    Some(else_block.into_inner().map(build_ast_from_statement).collect::<Result<Vec<_>, _>>()?)
                } else {
                    None
                }
            } else { None };
            Ok(AstNode::Statement(Statement::If {
                condition: Box::new(condition_expr),
                then_body,
                elifs,
                else_body,
            }))
        }
        _ => Err(format!("Unhandled statement type: {:?}\nContent: '{}'", inner.as_rule(), inner.as_str())),
    }
}

// Renamed from parse_eppx_string_final
pub fn parse_eppx_string(input: &str) -> Result<Vec<AstNode>, String> {
    match EppParser::parse(Rule::program, input) {
        Ok(mut pairs) => {
            let mut ast_nodes = Vec::new();
            let program_pair = pairs.next().unwrap(); 
            for pair in program_pair.into_inner() { 
                match pair.as_rule() {
                    Rule::statement => {
                        ast_nodes.push(build_ast_from_statement(pair)?); // Calls renamed build_ast_from_statement
                    }
                    Rule::EOI => {} 
                    _ => return Err(format!("Unexpected rule in program: {:?}, content: '{}'", pair.as_rule(), pair.as_str())),
                }
            }
            Ok(ast_nodes)
        }
        Err(e) => Err(format!("Parse failed: {}\nDetails: {}", e, e.variant.message())),
    }
}

// Renamed from parse_eppx_file_final
pub fn parse_eppx_file(file_path: &Path) -> Result<Vec<AstNode>, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;
    parse_eppx_string(&content) // Calls renamed parse_eppx_string
}
