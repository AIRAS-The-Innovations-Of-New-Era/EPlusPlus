// Codegen module placeholder
use crate::ast::{AstNode, Expression, Statement, BinOp, UnaryOp, AssignmentOperator};
use std::collections::HashSet;

pub fn generate_cpp_code_with_toplevel(ast_nodes: &[AstNode], is_toplevel: bool) -> Result<String, String> {
    let mut declared_vars = HashSet::new();
    _generate_cpp_code_with_vars(ast_nodes, is_toplevel, &mut declared_vars)
}

fn _generate_cpp_code_with_vars(ast_nodes: &[AstNode], is_toplevel: bool, declared_vars: &mut HashSet<String>) -> Result<String, String> {
    let mut cpp_out = String::new();
    if is_toplevel {
        cpp_out.push_str("#include <iostream>\n");
        cpp_out.push_str("#include <string>\n");
        cpp_out.push_str("#include <vector>\n");
        cpp_out.push_str("#include <algorithm>\n");
        cpp_out.push_str("#include <cmath> // Added for std::pow\n\n");
        // Basic print functions
        cpp_out.push_str("void eppx_print(const std::string& s) { std::cout << s << std::endl; }\n");
        cpp_out.push_str("void eppx_print(long long x) { std::cout << x << std::endl; }\n");
        cpp_out.push_str("void eppx_print(double x) { std::cout << x << std::endl; }\n");
        cpp_out.push_str("void eppx_print(bool b) { std::cout << (b ? \"true\" : \"false\") << std::endl; }\n\n");
        // Simple range helper for for loops
        cpp_out.push_str("std::vector<long long> eppx_range(long long n) {\n");
        cpp_out.push_str("    std::vector<long long> result;\n");
        cpp_out.push_str("    for (long long i = 0; i < n; ++i) {\n");
        cpp_out.push_str("        result.push_back(i);\n");
        cpp_out.push_str("    }\n");
        cpp_out.push_str("    return result;\n");
        cpp_out.push_str("}\n\n");
    }    // First pass: emit all function definitions at the top level
    for node in ast_nodes {
        if let AstNode::Statement(Statement::FunctionDef { name, params, body }) = node {
            // For now, use auto for parameters to let C++ deduce types
            let param_list = params.iter().map(|p| format!("auto {}", p)).collect::<Vec<_>>().join(", ");
            let mut declared_vars_fn = HashSet::new();
            for p in params {
                declared_vars_fn.insert(p.clone());
            }
            let body_cpp = _generate_cpp_code_with_vars(body, false, &mut declared_vars_fn)?;
            // Use auto return type to let C++ deduce
            cpp_out.push_str(&format!("auto {}({}) {{\n", name, param_list));
            cpp_out.push_str(&body_cpp);
            // Only add default return if no explicit return found in body
            let has_return = body.iter().any(|node| {
                matches!(node, AstNode::Statement(Statement::Return(_)))
            });
            if !has_return {
                cpp_out.push_str("    return 0;\n");
            }
            cpp_out.push_str("}\n\n");
        }
    }
    if is_toplevel {
        cpp_out.push_str("int main() {\n");
    }
    // Second pass: emit all non-function statements (main body)
    for node in ast_nodes {
        if let AstNode::Statement(Statement::FunctionDef { .. }) = node {
            continue;
        }
        match node {
            AstNode::Statement(Statement::Print(expr)) => {
                let expr_code = emit_expression_cpp(expr)?;
                cpp_out.push_str(&format!("    eppx_print({});\n", expr_code));
            }            AstNode::Statement(Statement::Assignment { name, operator, value }) => {
                let value_cpp = emit_expression_cpp(value)?;
                if !declared_vars.contains(name) {
                    // For new variables, declare and initialize in one statement
                    let type_str = match **value {
                        Expression::StringLiteral(_) => "std::string",
                        Expression::IntegerLiteral(_) => "long long",
                        Expression::FloatLiteral(_) => "double",
                        Expression::BinaryOperation { ref op, .. } => match op {
                            BinOp::And | BinOp::Or | BinOp::Eq | BinOp::NotEq | BinOp::Gt |
                            BinOp::Lt | BinOp::GtEq | BinOp::LtEq | BinOp::Is |
                            BinOp::IsNot | BinOp::In | BinOp::NotIn => "int",
                            _ => "auto"
                        },
                        Expression::UnaryOperation { ref op, .. } => match op {
                            UnaryOp::Not => "int",
                            UnaryOp::BitNot => "long long",
                        },
                        _ => "auto",
                    };
                    // Declare and initialize in one step
                    match operator {
                        AssignmentOperator::Assign => {
                            cpp_out.push_str(&format!("    {} {} = {};\n", type_str, name, value_cpp));
                        }
                        _ => {
                            // For compound assignments, we need the variable to exist first
                            // This shouldn't happen in well-formed code, but handle it gracefully
                            cpp_out.push_str(&format!("    {} {} = 0;\n", type_str, name));
                            let op_assign_str = match operator {
                                AssignmentOperator::AddAssign => format!("    {} += {};\n", name, value_cpp),
                                AssignmentOperator::SubAssign => format!("    {} -= {};\n", name, value_cpp),
                                AssignmentOperator::MulAssign => format!("    {} *= {};\n", name, value_cpp),
                                AssignmentOperator::DivAssign => format!("    {} /= {};\n", name, value_cpp),
                                AssignmentOperator::ModAssign => format!("    {} %= {};\n", name, value_cpp),
                                AssignmentOperator::PowAssign => format!("    {} = static_cast<long long>(std::pow({}, {}));\n", name, name, value_cpp),
                                AssignmentOperator::FloorDivAssign => format!("    {} /= {};\n", name, value_cpp),
                                AssignmentOperator::BitAndAssign => format!("    {} &= {};\n", name, value_cpp),
                                AssignmentOperator::BitOrAssign => format!("    {} |= {};\n", name, value_cpp),
                                AssignmentOperator::BitXorAssign => format!("    {} ^= {};\n", name, value_cpp),
                                AssignmentOperator::LShiftAssign => format!("    {} <<= {};\n", name, value_cpp),
                                AssignmentOperator::RShiftAssign => format!("    {} >>= {};\n", name, value_cpp),
                                _ => unreachable!()
                            };
                            cpp_out.push_str(&op_assign_str);
                        }
                    }
                    declared_vars.insert(name.clone());
                } else {
                    // Variable already exists, just assign
                    let op_assign_str = match operator {
                        AssignmentOperator::Assign => format!("    {} = {};\n", name, value_cpp),
                        AssignmentOperator::AddAssign => format!("    {} += {};\n", name, value_cpp),
                        AssignmentOperator::SubAssign => format!("    {} -= {};\n", name, value_cpp),
                        AssignmentOperator::MulAssign => format!("    {} *= {};\n", name, value_cpp),
                        AssignmentOperator::DivAssign => format!("    {} /= {};\n", name, value_cpp),
                        AssignmentOperator::ModAssign => format!("    {} %= {};\n", name, value_cpp),
                        AssignmentOperator::PowAssign => format!("    {} = static_cast<long long>(std::pow({}, {}));\n", name, name, value_cpp),
                        AssignmentOperator::FloorDivAssign => format!("    {} /= {};\n", name, value_cpp),
                        AssignmentOperator::BitAndAssign => format!("    {} &= {};\n", name, value_cpp),
                        AssignmentOperator::BitOrAssign => format!("    {} |= {};\n", name, value_cpp),
                        AssignmentOperator::BitXorAssign => format!("    {} ^= {};\n", name, value_cpp),
                        AssignmentOperator::LShiftAssign => format!("    {} <<= {};\n", name, value_cpp),
                        AssignmentOperator::RShiftAssign => format!("    {} >>= {};\n", name, value_cpp),
                    };
                    cpp_out.push_str(&op_assign_str);
                }
            }
            AstNode::Statement(Statement::If { condition, then_body, elifs, else_body }) => {
                let mut chain = String::new();
                let emit_block = |stmts: &Vec<AstNode>, declared_vars: &mut HashSet<String>| -> Result<String, String> {
                    let mut block = String::new();
                    for stmt in stmts {
                        let inner = _generate_cpp_code_with_vars(&[stmt.clone()], false, declared_vars)?;
                        for line in inner.lines() {
                            block.push_str(line);
                            block.push('\n');
                        }
                    }
                    Ok(block)
                };
                let cond_cpp = emit_expression_cpp(condition)?;
                chain.push_str(&format!("    if ({}) {{\n", cond_cpp));
                chain.push_str(&emit_block(then_body, declared_vars)?);
                chain.push_str("    }");
                for (elif_cond, elif_body) in elifs {
                    let elif_cond_cpp = emit_expression_cpp(elif_cond)?;
                    chain.push_str(&format!(" else if ({}) {{\n", elif_cond_cpp));
                    chain.push_str(&emit_block(elif_body, declared_vars)?);
                    chain.push_str("    }");
                }
                if let Some(else_body) = else_body {
                    chain.push_str(" else {\n");
                    chain.push_str(&emit_block(&else_body, declared_vars)?);
                    chain.push_str("    }");
                }
                chain.push_str("\n");
                cpp_out.push_str(&chain);
            }
            AstNode::Statement(Statement::While { condition, body }) => {
                let emit_block = |stmts: &Vec<AstNode>, declared_vars: &mut HashSet<String>| -> Result<String, String> {
                    let mut block = String::new();
                    for stmt in stmts {
                        let inner = _generate_cpp_code_with_vars(&[stmt.clone()], false, declared_vars)?;
                        for line in inner.lines() {
                            block.push_str(line);
                            block.push('\n');
                        }
                    }
                    Ok(block)
                };
                let cond_cpp = emit_expression_cpp(condition)?;
                let mut while_code = String::new();
                while_code.push_str(&format!("    while ({}) {{\n", cond_cpp));
                while_code.push_str(&emit_block(body, declared_vars)?);
                while_code.push_str("    }\n");
                cpp_out.push_str(&while_code);
            }            AstNode::Statement(Statement::For { var, iterable, body }) => {
                let emit_block = |stmts: &Vec<AstNode>, declared_vars: &mut HashSet<String>| -> Result<String, String> {
                    let mut block = String::new();
                    for stmt in stmts {
                        let inner = _generate_cpp_code_with_vars(&[stmt.clone()], false, declared_vars)?;
                        for line in inner.lines() {
                            block.push_str(line);
                            block.push('\n');
                        }
                    }
                    Ok(block)
                };
                let iterable_cpp = emit_expression_cpp(iterable)?;
                let mut for_code = String::new();
                if !declared_vars.contains(var) {
                    for_code.push_str(&format!("    long long {};\n", var));
                    declared_vars.insert(var.clone());
                }
                for_code.push_str(&format!("    for (auto {}_val : {}) {{\n", var, iterable_cpp));
                for_code.push_str(&format!("        {} = {}_val;\n", var, var));
                for_code.push_str(&emit_block(body, declared_vars)?);
                for_code.push_str("    }\n");
                cpp_out.push_str(&for_code);
            }
            AstNode::Statement(Statement::Return(expr)) => {
                if let Some(return_expr) = expr {
                    let return_value = emit_expression_cpp(return_expr)?;
                    cpp_out.push_str(&format!("    return {};\n", return_value));
                } else {
                    cpp_out.push_str("    return;\n"); // For void-like functions returning nothing explicitly
                }
            }
            AstNode::Statement(Statement::ExpressionStatement(expr)) => {
                let expr_code = emit_expression_cpp(expr)?;
                cpp_out.push_str(&format!("    {};\n", expr_code)); // Emit the expression, typically for side effects
            }
            _ => {}
        }
    }
    if is_toplevel {
        cpp_out.push_str("    return 0;\n}\n");
    }
    Ok(cpp_out)
}

pub fn generate_cpp_code(ast_nodes: &[AstNode]) -> Result<String, String> {
    generate_cpp_code_with_toplevel(ast_nodes, true)
}

fn emit_expression_cpp(expr: &Expression) -> Result<String, String> {
    match expr {
        Expression::StringLiteral(s) => Ok(format!("std::string(\"{}\")", s.replace("\\", "\\\\").replace("\"", "\\\""))),
        Expression::IntegerLiteral(i) => Ok(format!("{}LL", i)), // Suffix LL for long long
        Expression::FloatLiteral(f) => Ok(format!("{}", f)), // Float literals
        Expression::Identifier(name) => Ok(name.clone()),        Expression::UnaryOperation { op, operand } => {
            let operand_cpp = emit_expression_cpp(operand)?;
            match op {
                UnaryOp::Not => Ok(format!("!({})", operand_cpp)), // C++ bool context, will cast to int if needed by print
                UnaryOp::BitNot => Ok(format!("~({})", operand_cpp)),
            }
        }
        Expression::FunctionCall { name, args } => {
            match name.as_str() {
                "range" => {
                    // For now, just support range(n) - generates 0 to n-1
                    if args.len() == 1 {
                        let arg_cpp = emit_expression_cpp(&args[0])?;
                        Ok(format!("eppx_range({})", arg_cpp))
                    } else {
                        Err("range() with multiple arguments not yet supported".to_string())
                    }
                }
                _ => {
                    // User-defined function call
                    let arg_list = args.iter().map(|a| emit_expression_cpp(a)).collect::<Result<Vec<_>,_>>()?.join(", ");
                    Ok(format!("{}({})", name, arg_list))
                }
            }
        }
        Expression::BinaryOperation { left, op, right } => {
            let l = emit_expression_cpp(left)?;
            let r = emit_expression_cpp(right)?;
            let op_str = match op {
                // Arithmetic
                BinOp::Add => "+",
                BinOp::Sub => "-",
                BinOp::Mul => "*",
                BinOp::Div => "/", // C++ int division truncates
                BinOp::Mod => "%",
                BinOp::Pow => return Ok(format!("static_cast<long long>(std::pow({}, {}))", l, r)),
                BinOp::FloorDiv => "/", // C++ int division truncates, matches Python // for positive results
                // Comparison
                BinOp::Eq => "==",
                BinOp::NotEq => "!=",
                BinOp::Lt => "<",
                BinOp::Gt => ">",
                BinOp::LtEq => "<=",
                BinOp::GtEq => ">=",                // Logical
                BinOp::And => {
                    // In Python, 'and' returns the first falsy value or the last value
                    // In C++, && returns bool. For simplicity, we'll use && but cast to bool context
                    return Ok(format!("(({}) && ({}))", l, r));
                },
                BinOp::Or => {
                    // In Python, 'or' returns the first truthy value or the last value  
                    // In C++, || returns bool. For simplicity, we'll use || but cast to bool context
                    return Ok(format!("(({}) || ({}))", l, r));
                },
                // Bitwise
                BinOp::BitAnd => "&",
                BinOp::BitOr => "|",
                BinOp::BitXor => "^",
                BinOp::LShift => "<<",
                BinOp::RShift => ">>",
                // Identity (basic C++ translation, not Python's object identity)
                BinOp::Is => return Ok(format!("({} == {}) /* Placeholder for IS */", l, r)), // Primitive check
                BinOp::IsNot => return Ok(format!("({} != {}) /* Placeholder for IS NOT */", l, r)), // Primitive check
                // Membership (basic C++ string translation, not general purpose)
                BinOp::In => {
                    // Very basic string 'in' check. Assumes l is char/substring, r is string.
                    // This is a placeholder and needs a proper runtime type system.
                    // Example: r.find(l) != std::string::npos
                    // For now, let's assume r is a string and l is a string to find.
                    // This is highly simplified.
                    return Ok(format!("({}.find({}) != std::string::npos) /* Placeholder for IN */", r, l));
                }
                BinOp::NotIn => {
                    return Ok(format!("({}.find({}) == std::string::npos) /* Placeholder for NOT IN */", r, l));
                }
            };
            Ok(format!("({} {} {})", l, op_str, r))
        }
        _ => Err(String::from("Unsupported expression type for C++ codegen"))
    }
}

fn _emit_cpp_for_variable_declaration(name: &str, value: &Box<Expression>, is_new_declaration: bool, _existing_vars: &mut HashSet<String>) -> String {    // Determine the C++ type based on the expression type
    // This is a simplified type inference. A real system would be more complex.
    let type_str = match **value {
        Expression::StringLiteral(_) => "std::string",
        Expression::IntegerLiteral(_) => "long long", // Use long long for integers
        Expression::FloatLiteral(_) => "double", // Use double for floats
        Expression::BinaryOperation { ref op, .. } => match op {
            BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::Mod |
            BinOp::Pow | BinOp::FloorDiv |
            BinOp::BitAnd | BinOp::BitOr | BinOp::BitXor |
            BinOp::LShift | BinOp::RShift => "long long", // Arithmetic/Bitwise ops likely result in numbers
            BinOp::Eq | BinOp::NotEq | BinOp::Gt | BinOp::Lt | BinOp::GtEq | BinOp::LtEq |
            BinOp::And | BinOp::Or | BinOp::Is | BinOp::IsNot | BinOp::In | BinOp::NotIn => "int", // Comparison/Logical ops result in bool (int in C++)
        },        Expression::UnaryOperation { ref op, .. } => match op {
            UnaryOp::BitNot => "long long", // Bitwise not results in a number
            UnaryOp::Not => "int",       // Logical not results in bool (int in C++)
        },        Expression::Identifier(_) => "auto", // If assigned from another var, auto should work.
        Expression::FunctionCall { .. } => "auto", // Placeholder for function call return types
        Expression::Call { .. } => "auto", // Placeholder for call return types
        // Add other expression types as needed
        // _ => "auto" // Default to auto if type is unknown or complex
    };

    // If it's a new declaration, we don't have to worry about the existing type
    if is_new_declaration {
        format!("{} {}", type_str, name)
    } else {
        // For assignments, we assume the type matches the existing variable's type
        // In a real system, we'd check the symbol table or existing declarations
        format!("{} = {}", name, emit_expression_cpp(value).unwrap())
    }
}

