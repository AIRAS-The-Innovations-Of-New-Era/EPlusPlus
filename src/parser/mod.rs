use pest_derive::Parser;
use pest::Parser;
use pest::iterators::{Pair, Pairs}; // Added Pairs
use std::fs;
use std::path::Path;

use crate::ast::{AstNode, Expression, Statement, BinOp, UnaryOp, AssignmentOperator, Decorator, Argument}; // Added Decorator and Argument

#[derive(Parser)]
#[grammar = "parser/grammar.pest"]
pub struct EppParser;

// Helper function to process escape sequences in string literals
fn process_escape_sequences(input: &str) -> Result<String, String> {
    let mut result = String::new();
    let mut chars = input.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '\\' {
            if let Some(&next_ch) = chars.peek() {
                chars.next(); // consume the next character
                match next_ch {
                    'n' => result.push('\n'),
                    't' => result.push('\t'),
                    'r' => result.push('\r'),
                    'b' => result.push('\u{0008}'), // backspace
                    'f' => result.push('\u{000C}'), // form feed
                    'v' => result.push('\u{000B}'), // vertical tab
                    '0' => result.push('\0'),
                    '\\' => result.push('\\'),
                    '"' => result.push('"'),
                    '\'' => result.push('\''),
                    'a' => result.push('\u{0007}'), // bell
                    _ => return Err(format!("Unknown escape sequence: \\{}", next_ch)),
                }
            } else {
                return Err("Incomplete escape sequence".to_string());
            }
        } else {
            result.push(ch);
        }
    }
    
    Ok(result)
}

/// Preprocesses Python-style indentation into explicit @INDENT@ and @DEDENT@ tokens for the parser.
pub fn preprocess_indentation(input: &str) -> String {
    let mut result = String::new();
    let mut indent_stack: Vec<usize> = vec![0];
    
    for line in input.lines() {
        let trimmed = line.trim(); // Trim both sides to correctly identify blank lines or comments
        let is_blank_or_comment = trimmed.is_empty() || trimmed.starts_with('#');
        
        if is_blank_or_comment {
            result.push_str(line);
            result.push('\n');
            continue;
        }
        
        let indent = line.chars().take_while(|c| *c == ' ' || *c == '\t').count();
        let current_indent = *indent_stack.last().unwrap();
        
        if indent > current_indent {
            // Indentation increased
            indent_stack.push(indent);
            result.push_str("@INDENT@
");
        } else if indent < current_indent {
            // Indentation decreased - emit DEDENT tokens for each level we're backing out
            while indent < *indent_stack.last().unwrap() {
                indent_stack.pop();
                result.push_str("@DEDENT@
");
            }
        }
        // If indent == current_indent, we stay at the same level
        
        result.push_str(line.trim_start());
        result.push('\n');
    }
    
    // Close any remaining indents
    while indent_stack.len() > 1 {
        indent_stack.pop();
        result.push_str("@DEDENT@\n");
    }
    result
}

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
        Rule::expression => {
            // expression = { logical_or } (or similar highest precedence rule)
            // It always has one inner pair representing the start of the expression hierarchy.
            let inner_expr_pair = pair.into_inner().next().ok_or_else(|| "Expression rule is empty".to_string())?;
            build_ast_from_expression(inner_expr_pair)
        }        Rule::factor => {
            let mut inner_pairs = pair.into_inner();
            let atom_pair = inner_pairs.next().ok_or_else(|| "Factor rule is empty, expected atom".to_string())?;

            // The first part of a factor must be an atom.
            // The atom_pair itself will have a rule like Rule::atom.
            let mut current_expr = build_ast_from_expression(atom_pair)?;
            // Process potential call suffixes and attribute access: (call_suffix | attr_access)*
            let remaining_pairs: Vec<_> = inner_pairs.collect();
            for suffix_pair in remaining_pairs {
                match suffix_pair.as_rule() {
                    Rule::call_suffix => {
                        let mut args = Vec::new();
                        // call_suffix_pair.into_inner() gives what's inside the parentheses.
                        for inner_pair in suffix_pair.into_inner() {
                            if inner_pair.as_rule() == Rule::argument_list {
                                // argument_list = { argument ~ ("," ~ argument)* }
                                // argument = { keyword_argument | expression }
                                for arg_pair in inner_pair.into_inner() {
                                    match arg_pair.as_rule() {
                                        Rule::argument => {
                                            // Parse the argument content
                                            let arg_content = arg_pair.into_inner().next().ok_or("Empty argument")?;
                                            if arg_content.as_rule() == Rule::keyword_argument {
                                                // For function calls, we'll ignore keyword arguments for now
                                                // and just parse the value part
                                                let mut kw_inner = arg_content.into_inner();
                                                let _name = kw_inner.next(); // Skip the name
                                                let value_pair = kw_inner.next().ok_or("Keyword argument missing value")?;
                                                args.push(build_ast_from_expression(value_pair)?);
                                            } else {
                                                // Regular expression argument
                                                args.push(build_ast_from_expression(arg_content)?);
                                            }
                                        }
                                        _ => {
                                            // Direct expression (fallback for old grammar compatibility)
                                            args.push(build_ast_from_expression(arg_pair)?);
                                        }
                                    }
                                }
                            } else if matches!(inner_pair.as_rule(), Rule::logical_or | Rule::expression) {
                                // Single argument case - the grammar might produce a direct expression instead of argument_list
                                args.push(build_ast_from_expression(inner_pair)?);
                            } else {
                                // This case implies that there was something between the parentheses,
                                // but it wasn't an argument_list or expression. This would be a grammar mismatch.
                                return Err(format!(
                                    "Expected Rule::argument_list inside call suffix, found {:?} with content '{}'",
                                    inner_pair.as_rule(),
                                    inner_pair.as_str()
                                ));
                            }
                        }
                        // If call_suffix_pair.into_inner().next() is None, it means empty parentheses `()`,
                        // so args remains empty, which is correct.

                        current_expr = Expression::Call {
                            callee: Box::new(current_expr),
                            args,
                        };
                    }
                    Rule::attr_access => {
                        // attr_access = { "." ~ identifier }
                        let attr_name = suffix_pair.into_inner().next().ok_or("Missing attribute name")?;
                        current_expr = Expression::AttributeAccess {
                            object: Box::new(current_expr),
                            attr: attr_name.as_str().to_string(),
                        };
                    }
                    _ => return Err(format!("Unexpected suffix rule: {:?}", suffix_pair.as_rule())),
                }
            }
            Ok(current_expr)
        }
        Rule::atom => {
            // atom = { lambda_expression | list_literal | ... | identifier | "(" ~ expression ~ ")" }
            // The pair for 'atom' contains one of these alternatives as its single inner rule.
            let inner_atom_content_pair = pair.into_inner().next().ok_or_else(|| "Atom rule is empty".to_string())?;
            build_ast_from_expression(inner_atom_content_pair)
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
        }        Rule::mul_div_mod => {
            build_recursive_ast_from_binary_expr_rule(pair.into_inner(), build_ast_from_expression, &[
                (Rule::mul_op, vec![
                    ("*", BinOp::Mul), ("/", BinOp::Div), 
                    ("//", BinOp::FloorDiv), ("%", BinOp::Mod)
                ])
            ], None)
        }
        Rule::unary_plus_minus => {
            let mut inner = pair.into_inner();
            let first_child = inner.next().unwrap();
            if first_child.as_rule() == Rule::unary_plus_minus_op {
                let op_str = first_child.as_str();
                let operand = build_ast_from_expression(inner.next().unwrap())?;
                match op_str {
                    "+" => Ok(operand), // Unary + is a no-op in most contexts
                    "-" => Ok(Expression::UnaryOperation { op: UnaryOp::Negate, operand: Box::new(operand) }),
                    _ => Err(format!("Unknown unary operator: {}", op_str))
                }
            } else {
                build_ast_from_expression(first_child)
            }
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
        }        Rule::string_literal => {
            let full_str = pair.as_str();
            let (_quote_char, content) = if full_str.starts_with('"') {
                ('"', &full_str[1..full_str.len()-1])
            } else if full_str.starts_with('\'') {
                ('\'', &full_str[1..full_str.len()-1])
            } else {
                return Err("Invalid string literal".to_string());
            };
            // Process escape sequences
            let processed_content = process_escape_sequences(content)?;
            Ok(Expression::StringLiteral(processed_content))
        }Rule::integer_literal => {
            let val = pair.as_str().parse::<i64>().map_err(|e| format!("Invalid integer: {}", e))?;
            Ok(Expression::IntegerLiteral(val))
        }        Rule::float_literal => {
            let val = pair.as_str().parse::<f64>().map_err(|e| format!("Invalid float: {}", e))?;
            Ok(Expression::FloatLiteral(val))
        }
        Rule::none_literal => { // Added for None
            Ok(Expression::NoneLiteral)
        }
        Rule::identifier => {
            Ok(Expression::Identifier(pair.as_str().to_string()))
        }
        // Rule::function_call is removed. It's handled by Rule::factor now.
        Rule::lambda_expression => {
            let mut inner_pairs = pair.into_inner();
            let mut params = Vec::new();

            // Peek at the first inner pair to see if it's a parameter_list
            // Grammar: lambda_expression = { "lambda" ~ parameter_list? ~ ":" ~ expression }
            // inner_pairs will contain [parameter_list (if present), expression]
            
            let first_inner_pair = inner_pairs.peek().ok_or_else(|| "Lambda expression is empty".to_string())?;

            if first_inner_pair.as_rule() == Rule::parameter_list {
                let param_list_pair = inner_pairs.next().unwrap(); // Consume parameter_list
                for param_ident_pair in param_list_pair.into_inner() {
                    // parameter_list = { identifier ~ ("," ~ identifier)* }
                    // Each inner pair of parameter_list should be an identifier
                    if param_ident_pair.as_rule() == Rule::identifier {
                        params.push(param_ident_pair.as_str().to_string());
                    } else {
                        return Err(format!("Expected identifier in lambda parameter list, got {:?}", param_ident_pair.as_rule()));
                    }
                }
            }
            // Else, no parameter_list, params remains empty.            // The next (or first, if no params) pair must be the expression body
            let body_expr_pair = inner_pairs.next().ok_or_else(|| "Lambda expression missing body".to_string())?;
            // Since expression is a silent rule (_{ logical_or }), the actual rule will be logical_or
            // We can accept either expression or logical_or as valid body rules
            if body_expr_pair.as_rule() != Rule::expression && body_expr_pair.as_rule() != Rule::logical_or {
                 return Err(format!("Expected expression for lambda body, got {:?} with content '{}'", body_expr_pair.as_rule(), body_expr_pair.as_str()));
            }

            let body = build_ast_from_expression(body_expr_pair)?;
            Ok(Expression::Lambda {
                params,
                body: Box::new(body),
            })
        }        Rule::list_literal => {
            // Parse list literal: [expr1, expr2, ...]
            let mut elements = Vec::new();
            for expr_pair in pair.into_inner() {
                // Since expression is a silent rule, we get the actual expression rule
                // Skip any potential comma tokens and process all expressions
                elements.push(build_ast_from_expression(expr_pair)?);
            }
            Ok(Expression::ListLiteral(elements))
        }
        Rule::tuple_literal => {
            let mut elements = Vec::new();
            for expr_pair in pair.into_inner() {
                elements.push(build_ast_from_expression(expr_pair)?);
            }
            Ok(Expression::TupleLiteral(elements))
        }
        Rule::dict_literal => {
            let mut entries = Vec::new();
            for entry_pair in pair.into_inner() {
                let mut entry_inner = entry_pair.into_inner();
                let key = build_ast_from_expression(entry_inner.next().unwrap())?;
                let value = build_ast_from_expression(entry_inner.next().unwrap())?;
                entries.push((key, value));
            }
            Ok(Expression::DictLiteral(entries))
        }
        Rule::set_literal => {
            let mut elements = Vec::new();
            for expr_pair in pair.into_inner() {
                elements.push(build_ast_from_expression(expr_pair)?);
            }
            Ok(Expression::SetLiteral(elements))
        }
        Rule::frozenset_literal => {
            let mut inner = pair.into_inner();
            let list_expr = build_ast_from_expression(inner.next().unwrap())?;
            if let Expression::ListLiteral(elements) = list_expr {
                Ok(Expression::FrozensetLiteral(elements))
            } else {
                Err("frozenset() expects a list literal".to_string())
            }
        }
        Rule::complex_literal => {
            let mut inner = pair.into_inner();
            let real = build_ast_from_expression(inner.next().unwrap())?;
            let imag = build_ast_from_expression(inner.next().unwrap())?;
            Ok(Expression::ComplexLiteral(Box::new(real), Box::new(imag)))
        }
        // Catch-all for rules that should have been handled by `expression` or `factor`'s recursion,
        // or are actual terminals not listed above.
        _ => Err(format!("Unhandled expression rule: {:?}\nContent: '{}'", pair.as_rule(), pair.as_str())),    }
}

// Helper to build assign_target (identifier or attribute chain) as Expression
fn build_ast_from_assign_target(pair: Pair<Rule>) -> Result<Expression, String> {
    match pair.as_rule() {
        Rule::identifier => {
            Ok(Expression::Identifier(pair.as_str().to_string()))
        }
        // Support attribute chains: identifier ('.' identifier)*
        Rule::assign_target => {
            let mut inner = pair.into_inner();
            let mut expr = Expression::Identifier(inner.next().unwrap().as_str().to_string());
            for attr in inner {
                expr = Expression::AttributeAccess {
                    object: Box::new(expr),
                    attr: attr.as_str().to_string(),
                };
            }
            Ok(expr)
        }
        _ => Err(format!("Invalid assign_target: {:?}", pair.as_rule())),
    }
}

// Function to parse for loop targets (single variable or tuple unpacking)
fn parse_for_target(pair: Pair<Rule>) -> Result<Vec<String>, String> {
    match pair.as_rule() {
        Rule::for_target => {
            let inner = pair.into_inner().next().unwrap();
            parse_for_target(inner) // Recurse into the actual target
        },
        Rule::identifier => {
            // Single variable
            Ok(vec![pair.as_str().to_string()])
        },
        Rule::for_tuple_unpacking => {
            // Multiple variables (tuple unpacking)
            let vars: Vec<String> = pair.into_inner()
                .map(|p| p.as_str().to_string())
                .collect();
            Ok(vars)
        },
        _ => Err(format!("Unexpected for target rule: {:?}", pair.as_rule()))
    }
}

// Function to parse arguments (positional and keyword)
fn parse_argument(pair: Pair<Rule>) -> Result<Argument, String> {
    match pair.as_rule() {
        Rule::keyword_argument => {
            let mut inner = pair.into_inner();
            let name_pair = inner.next().ok_or("Keyword argument missing name")?;
            let value_pair = inner.next().ok_or("Keyword argument missing value")?;
            let name = name_pair.as_str().to_string();
            let value = build_ast_from_expression(value_pair)?;
            Ok(Argument::Keyword(name, value))
        }
        _ => {
            // Treat as positional argument (expression)
            let expr = build_ast_from_expression(pair)?;
            Ok(Argument::Positional(expr))
        }
    }
}

// Function to parse decorator
fn parse_decorator(pair: Pair<Rule>) -> Result<Decorator, String> {
    let mut inner = pair.into_inner(); // decorator_name, decorator_args?
    
    let name_pair = inner.next().ok_or("Decorator missing name")?;
    let decorator_name = name_pair.as_str().to_string();
    
    if let Some(args_pair) = inner.next() {
        // Decorator has arguments
        let mut args = Vec::new();
        for arg_pair in args_pair.into_inner() {
            if arg_pair.as_rule() == Rule::argument_list {
                for inner_arg_pair in arg_pair.into_inner() {
                    if inner_arg_pair.as_rule() == Rule::argument {
                        // Parse the argument content
                        let arg_content = inner_arg_pair.into_inner().next().ok_or("Empty argument")?;
                        args.push(parse_argument(arg_content)?);
                    } else {
                        // Direct expression (fallback for old grammar compatibility)
                        args.push(parse_argument(inner_arg_pair)?);
                    }
                }
            }
        }
        Ok(Decorator::WithArgs(decorator_name, args))
    } else {
        // Simple decorator without arguments
        Ok(Decorator::Simple(decorator_name))
    }
}

// Renamed from build_ast_from_statement_final
fn build_ast_from_statement(pair: Pair<Rule>) -> Result<AstNode, String> {
    // Determine the actual specific rule to process.
    // 'pair' is expected to be Rule::statement (when called from program loop or block processing)
    // or Rule::function_definition (when called from program loop).
    let specific_statement_pair = match pair.as_rule() {
        Rule::statement => {
            let pair_str = pair.as_str(); // Get string representation before moving
            pair.into_inner().next().ok_or_else(|| {
                format!("Rule::statement was empty. Content: '{}'", pair_str)
            })?
        }
        Rule::function_definition => pair, // function_definition is processed directly by its own structure
        Rule::class_definition => pair, // class_definition is processed directly
        _ => return Err(format!("build_ast_from_statement called with unexpected rule: {:?}. Content: '{}'", pair.as_rule(), pair.as_str())),
    };

    match specific_statement_pair.as_rule() {        Rule::assignment => {
            let mut inner_rules = specific_statement_pair.into_inner();
            // Parse assign_target as an expression (attribute chain or identifier)
            let lhs_pair = inner_rules.next().unwrap();
            let target_expr = build_ast_from_assign_target(lhs_pair)?;
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
                target: Box::new(target_expr),
                operator,
                value: Box::new(value_expr),
            }))
        }        Rule::if_statement => {
            let mut inner_rules = specific_statement_pair.into_inner(); // condition, block, elif_clause*, else_clause?
            let condition_expr = build_ast_from_expression(inner_rules.next().unwrap())?;
            let then_block_pair = inner_rules.next().unwrap();
            if then_block_pair.as_rule() != Rule::block { return Err("If statement missing then-block".to_string()); }
            let then_body = then_block_pair.into_inner()
                                .filter(|p| p.as_rule() == Rule::statement)
                                .map(build_ast_from_statement).collect::<Result<Vec<_>, _>>()?;
            
            let mut elifs = Vec::new();
            while let Some(peeked_pair) = inner_rules.peek() {
                if peeked_pair.as_rule() == Rule::elif_clause {
                    let elif_pair = inner_rules.next().unwrap(); // consume elif_clause
                    let mut elif_inner = elif_pair.into_inner(); // expression, block
                    let elif_cond = build_ast_from_expression(elif_inner.next().unwrap())?;
                    let elif_block_pair = elif_inner.next().unwrap();
                    if elif_block_pair.as_rule() != Rule::block { return Err("Elif clause missing block".to_string()); }
                    let elif_body = elif_block_pair.into_inner()
                                        .filter(|p| p.as_rule() == Rule::statement)
                                        .map(build_ast_from_statement).collect::<Result<Vec<_>, _>>()?;
                    elifs.push((elif_cond, elif_body));
                } else {
                    break; // Not an elif_clause, could be else_clause or nothing
                }
            }
            
            let else_body = if let Some(peeked_pair) = inner_rules.peek() {
                if peeked_pair.as_rule() == Rule::else_clause {
                    let else_pair = inner_rules.next().unwrap(); // consume else_clause
                    let else_block_pair = else_pair.into_inner().next().unwrap();
                    if else_block_pair.as_rule() != Rule::block { return Err("Else clause missing block".to_string()); }
                    Some(else_block_pair.into_inner()
                            .filter(|p| p.as_rule() == Rule::statement)
                            .map(build_ast_from_statement).collect::<Result<Vec<_>, _>>()?)
                } else { None }
            } else { None };

            Ok(AstNode::Statement(Statement::If {
                condition: Box::new(condition_expr),
                then_body,
                elifs,
                else_body,
            }))
        }        Rule::while_statement => {
            let mut inner_rules = specific_statement_pair.into_inner(); // condition, block
            let condition_expr = build_ast_from_expression(inner_rules.next().unwrap())?;
            let block_pair = inner_rules.next().unwrap();
            if block_pair.as_rule() != Rule::block { return Err("While statement missing block".to_string()); }
            
            let mut body = Vec::new();
            for inner_pair in block_pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::indented_statements => {
                        for stmt_pair in inner_pair.into_inner() {
                            if matches!(stmt_pair.as_rule(), Rule::statement | Rule::function_definition | Rule::class_definition) {
                                body.push(build_ast_from_statement(stmt_pair)?);
                            }
                        }
                    }
                    Rule::statement | Rule::function_definition | Rule::class_definition => {
                        body.push(build_ast_from_statement(inner_pair)?);
                    }
                    _ => { /* Skip INDENT, DEDENT, WHITESPACE, COMMENT */ }
                }
            }
            
            Ok(AstNode::Statement(Statement::While {
                condition: Box::new(condition_expr),
                body,
            }))
        }Rule::for_statement => {
            let mut inner_rules = specific_statement_pair.into_inner(); // for_target, expression, block
            let target_pair = inner_rules.next().unwrap();
            let vars = parse_for_target(target_pair)?;
            let iterable_expr = build_ast_from_expression(inner_rules.next().unwrap())?;
            let block_pair = inner_rules.next().unwrap();
            if block_pair.as_rule() != Rule::block { return Err("For statement missing block".to_string()); }
            
            let mut body = Vec::new();
            for inner_pair in block_pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::indented_statements => {
                        for stmt_pair in inner_pair.into_inner() {
                            if matches!(stmt_pair.as_rule(), Rule::statement | Rule::function_definition | Rule::class_definition) {
                                body.push(build_ast_from_statement(stmt_pair)?);
                            }
                        }
                    }
                    Rule::statement | Rule::function_definition | Rule::class_definition => {
                        body.push(build_ast_from_statement(inner_pair)?);
                    }
                    _ => { /* Skip INDENT, DEDENT, WHITESPACE, COMMENT */ }
                }
            }
            
            Ok(AstNode::Statement(Statement::For {
                vars,
                iterable: Box::new(iterable_expr),
                body,
            }))
        }Rule::function_definition => {
            // specific_statement_pair is Rule::function_definition
            let mut func_def_inner = specific_statement_pair.into_inner(); // decorator*, def, identifier, parameter_list?, block
            
            // Parse decorators first
            let mut decorators = Vec::new();
            while let Some(peeked) = func_def_inner.peek() {
                if peeked.as_rule() == Rule::decorator {
                    let decorator_pair = func_def_inner.next().unwrap();
                    decorators.push(parse_decorator(decorator_pair)?);
                } else {
                    break;
                }
            }
            
            // Skip "def" keyword - it's implicit in the grammar
            let name = func_def_inner.next().unwrap().as_str().to_string();
            
            let mut params = Vec::new();
            if let Some(peeked_param_or_block) = func_def_inner.peek() {
                if peeked_param_or_block.as_rule() == Rule::parameter_list {
                    let param_list_pair = func_def_inner.next().unwrap(); // consume parameter_list
                    for param_ident_pair in param_list_pair.into_inner() {
                        if param_ident_pair.as_rule() == Rule::identifier {
                            params.push(param_ident_pair.as_str().to_string());
                        }
                        // Commas are part of the structure of parameter_list, not separate tokens here
                    }
                }
            }
            
            let block_pair = func_def_inner.next().ok_or_else(|| format!("Function '{}' missing block.", name))?;
            if block_pair.as_rule() != Rule::block { return Err(format!("Function '{}' expected block, got {:?}.", name, block_pair.as_rule())); }
            // Parse function body, handling indented_statements like class bodies
            let mut body = Vec::new();
            for inner_pair in block_pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::indented_statements => {
                        for stmt_pair in inner_pair.into_inner() {
                            if matches!(stmt_pair.as_rule(), Rule::statement | Rule::function_definition | Rule::class_definition) {
                                body.push(build_ast_from_statement(stmt_pair)?);
                            }
                        }
                    }
                    Rule::statement | Rule::function_definition | Rule::class_definition => {
                        body.push(build_ast_from_statement(inner_pair)?);
                    }
                    _ => { /* Skip INDENT, DEDENT, WHITESPACE, COMMENT */ }
                }
            }
            Ok(AstNode::Statement(Statement::FunctionDef {
                name,
                params,
                body,
                decorators,
            }))
        }
        Rule::return_statement => {
            let mut inner_rules = specific_statement_pair.into_inner();
            let expr = if let Some(expr_pair) = inner_rules.next() {
                Some(Box::new(build_ast_from_expression(expr_pair)?))
            } else {
                None
            };
            Ok(AstNode::Statement(Statement::Return(expr)))
        }
        Rule::expression_statement => {
            let expr_pair = specific_statement_pair.into_inner().next().ok_or_else(|| "Expression statement missing expression".to_string())?;
            let expr_node = build_ast_from_expression(expr_pair)?;
            Ok(AstNode::Statement(Statement::ExpressionStatement(Box::new(expr_node))))
        }
        Rule::break_statement => Ok(AstNode::Statement(Statement::Break)),
        Rule::continue_statement => Ok(AstNode::Statement(Statement::Continue)),
        Rule::pass_statement => Ok(AstNode::Statement(Statement::Pass)),
        Rule::class_definition => {
            // specific_statement_pair is Rule::class_definition
            let mut class_def_inner = specific_statement_pair.into_inner();
            let name = class_def_inner.next().unwrap().as_str().to_string();
            // Optionally parse base class (not used yet)
            let mut maybe_base = None;
            if let Some(peeked) = class_def_inner.peek() {
                if peeked.as_rule() == Rule::identifier {
                    maybe_base = Some(class_def_inner.next().unwrap().as_str().to_string());
                }
            }            let block_pair = class_def_inner.next().ok_or_else(|| format!("Class '{}' missing block.", name))?;
            if block_pair.as_rule() != Rule::block { return Err(format!("Class '{}' expected block, got {:?}.", name, block_pair.as_rule())); }
            
            let mut body = Vec::new();
            for inner_pair in block_pair.into_inner() {
                match inner_pair.as_rule() {
                    Rule::indented_statements => {
                        // Parse all statements/function_definitions/class_definitions inside indented_statements
                        for stmt_pair in inner_pair.into_inner() {
                            if matches!(stmt_pair.as_rule(), Rule::statement | Rule::function_definition | Rule::class_definition) {
                                body.push(build_ast_from_statement(stmt_pair)?);
                            }
                        }
                    }
                    Rule::statement | Rule::function_definition | Rule::class_definition => {
                        // Direct statement (fallback)
                        body.push(build_ast_from_statement(inner_pair)?);
                    }
                    _ => {
                        // Skip INDENT, DEDENT, WHITESPACE, COMMENT
                    }
                }
            }
            Ok(AstNode::Statement(Statement::ClassDef {
                name,
                base: maybe_base,
                body,
            }))
        }
        _ => Err(format!(
            "Unhandled specific statement rule: {:?}\nContent: '{}'",
            specific_statement_pair.as_rule(),
            specific_statement_pair.as_str()
        )),
    }
}

// Renamed from parse_eppx_string_final
pub fn parse_eppx_string(input: &str) -> Result<Vec<AstNode>, String> {    let preprocessed = preprocess_indentation(input);
    println!("Preprocessed input:");
    println!("{}", preprocessed);
    println!("--- End preprocessed input ---");

    match EppParser::parse(Rule::program, &preprocessed) {
        Ok(mut pairs) => {
            let program_pair = pairs.next().ok_or_else(|| "Empty program".to_string())?;
            if program_pair.as_rule() != Rule::program {
                return Err(format!("Expected Rule::program, got {:?}", program_pair.as_rule()));
            }

            let mut ast_nodes = Vec::new();
            // Iterate over the inner pairs of the program rule
            for pair in program_pair.into_inner() {
                match pair.as_rule() {
                    Rule::statement | Rule::function_definition | Rule::class_definition => {
                        ast_nodes.push(build_ast_from_statement(pair)?);
                    }
                    Rule::COMMENT | Rule::WHITESPACE => {
                        // skip
                    }
                    _ => {
                        // Print debug info for unexpected rules
                        // eprintln!("Skipping unexpected rule in program: {:?} content: '{}'", pair.as_rule(), pair.as_str());
                    }
                }
            }
            Ok(ast_nodes)
        }
        Err(e) => Err(format!("Parse failed: {}\nDetails: {}\nInput: '{}'", e, e.variant.message(), input)),
    }
}

// Renamed from parse_eppx_file_final
pub fn parse_eppx_file(file_path: &Path) -> Result<Vec<AstNode>, String> {
    let content = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read file {}: {}", file_path.display(), e))?;
    parse_eppx_string(&content) // Calls renamed parse_eppx_string
}
