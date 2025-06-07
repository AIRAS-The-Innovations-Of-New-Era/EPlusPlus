// Codegen module placeholder
use crate::ast::{AstNode, Expression, Statement, BinOp, UnaryOp, AssignmentOperator};
use std::collections::HashSet;

pub fn generate_cpp_code_with_toplevel(ast_nodes: &[AstNode], is_toplevel: bool) -> Result<String, String> {
    let mut cpp_out = String::new();
    let mut declared_vars = HashSet::new(); // Track declared variables
    if is_toplevel {
        cpp_out.push_str("#include <iostream>\n");
        cpp_out.push_str("#include <string>\n");
        cpp_out.push_str("#include <vector>\n"); // For potential future use with 'in' on lists
        cpp_out.push_str("#include <algorithm>\n"); // For std::find, potentially for 'in'
        cpp_out.push_str("#include <cmath> // Added for std::pow\n\n");
        // Basic print functions
        cpp_out.push_str("void eppx_print(const std::string& s) { std::cout << s << std::endl; }\n");
        cpp_out.push_str("void eppx_print(long long x) { std::cout << x << std::endl; }\n"); // Changed int to long long
        cpp_out.push_str("void eppx_print(double x) { std::cout << x << std::endl; }\n"); // Added double for floats
        cpp_out.push_str("void eppx_print(int x) { std::cout << x << std::endl; }\n"); // Keep int for bools
        cpp_out.push_str("void eppx_print(bool b) { std::cout << (b ? \"true\" : \"false\") << std::endl; }\n\n");
        // Placeholder for a simple E++ object type for identity checks if needed later
        // cpp_out.push_str("struct EppxObject { long long id; /* other data */ };\n");
        cpp_out.push_str("int main() {\n");
    }
    for node in ast_nodes {
        match node {
            AstNode::Statement(Statement::Print(expr)) => {
                let expr_code = emit_expression_cpp(expr)?;
                cpp_out.push_str(&format!("    eppx_print({});\n", expr_code));
            }
            AstNode::Statement(Statement::Assignment { name, operator, value }) => {
                let value_cpp = emit_expression_cpp(value)?;
                if !declared_vars.contains(name) {
                    let type_str = match **value {
                        Expression::StringLiteral(_) => "std::string",
                        Expression::IntegerLiteral(_) => "long long",
                        Expression::FloatLiteral(_) => "double",
                        Expression::BinaryOperation { ref op, .. } => match op {
                            BinOp::And | BinOp::Or | BinOp::Eq | BinOp::NotEq | BinOp::Gt |
                            BinOp::Lt | BinOp::GtEq | BinOp::LtEq | BinOp::Is |
                            BinOp::IsNot | BinOp::In | BinOp::NotIn => "int",
                            _ => "double"
                        },
                        Expression::UnaryOperation { ref op, .. } => match op {
                            UnaryOp::Not => "int",
                            UnaryOp::BitNot => "long long",
                        },
                        Expression::Identifier(_) => "auto",
                        _ => "auto",
                    };
                    cpp_out.push_str(&format!("    {} {};\n", type_str, name));
                    declared_vars.insert(name.clone());
                }
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
            AstNode::Statement(Statement::If { condition, then_body, elifs, else_body }) => {
                let mut chain = String::new();
                let emit_block = |stmts: &Vec<AstNode>| -> Result<String, String> {
                    let mut block = String::new();
                    for stmt in stmts {
                        let inner = generate_cpp_code_with_toplevel(&[stmt.clone()], false)?;
                        for line in inner.lines() {
                            block.push_str(line);
                            block.push('\n');
                        }
                    }
                    Ok(block)
                };
                let cond_cpp = emit_expression_cpp(condition)?;
                chain.push_str(&format!("    if ({}) {{\n", cond_cpp));
                chain.push_str(&emit_block(then_body)?);
                chain.push_str("    }");
                for (elif_cond, elif_body) in elifs {
                    let elif_cond_cpp = emit_expression_cpp(elif_cond)?;
                    chain.push_str(&format!(" else if ({}) {{\n", elif_cond_cpp));
                    chain.push_str(&emit_block(elif_body)?);
                    chain.push_str("    }");
                }
                if let Some(else_body) = else_body {
                    chain.push_str(" else {\n");
                    chain.push_str(&emit_block(&else_body)?);
                    chain.push_str("    }");
                }
                chain.push_str("\n");
                cpp_out.push_str(&chain);
            }
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
        Expression::Identifier(name) => Ok(name.clone()),
        Expression::UnaryOperation { op, operand } => {
            let operand_cpp = emit_expression_cpp(operand)?;
            match op {
                UnaryOp::Not => Ok(format!("!({})", operand_cpp)), // C++ bool context, will cast to int if needed by print
                UnaryOp::BitNot => Ok(format!("~({})", operand_cpp)),
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
        },
        Expression::UnaryOperation { ref op, .. } => match op {
            UnaryOp::BitNot => "long long", // Bitwise not results in a number
            UnaryOp::Not => "int",       // Logical not results in bool (int in C++)
        },
        Expression::Identifier(_) => "auto", // If assigned from another var, auto should work.
        Expression::Call { .. } => "auto", // Placeholder for function call return types
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

