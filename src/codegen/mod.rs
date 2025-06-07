// Codegen module placeholder
use crate::ast::{AstNode, Expression, Statement, BinOp, AssignmentOperator};

pub fn generate_cpp_code(ast_nodes: &[AstNode]) -> Result<String, String> {
    let mut cpp_out = String::new();
    cpp_out.push_str("#include <iostream>\n");
    cpp_out.push_str("#include <string>\n");
    cpp_out.push_str("#include <cmath> // Added for std::pow\n\n");
    cpp_out.push_str("void eppx_print(const std::string& s) { std::cout << s << std::endl; }\n");
    cpp_out.push_str("void eppx_print(int x) { std::cout << x << std::endl; }\n\n");
    cpp_out.push_str("int main() {\n");
    for node in ast_nodes {
        match node {
            AstNode::Statement(Statement::Print(expr)) => {
                match &**expr {
                    Expression::StringLiteral(s) => {
                        let escaped_s = s.replace("\\", "\\\\").replace("\"", "\\\"");
                        cpp_out.push_str(&format!("    eppx_print(\"{}\");\n", escaped_s));
                    }
                    Expression::IntegerLiteral(i) => {
                        cpp_out.push_str(&format!("    eppx_print({});\n", i));
                    }
                    Expression::Identifier(name) => {
                        cpp_out.push_str(&format!("    eppx_print({});\n", name));
                    }
                    Expression::BinaryOperation { .. } => {
                        let expr_code = emit_expression_cpp(expr);
                        cpp_out.push_str(&format!("    eppx_print({});\n", expr_code));
                    }
                    _ => {}
                }
            }
            AstNode::Statement(Statement::Assignment { name, operator, value }) => {
                let value_cpp = emit_expression_cpp(value);
                match operator {
                    AssignmentOperator::Assign => {
                        // Determine type based on expression type - simplistic for now
                        // A proper symbol table and type system would be needed for robust type deduction
                        match **value {
                            Expression::StringLiteral(_) => {
                                cpp_out.push_str(&format!("    std::string {} = {};\n", name, value_cpp));
                            }
                            Expression::IntegerLiteral(_) |
                            Expression::BinaryOperation { .. } |
                            Expression::Identifier(_) => { // Assuming int/auto for these for now
                                cpp_out.push_str(&format!("    auto {} = {};\n", name, value_cpp));
                            }
                            _ => { // Default to auto, might cause C++ errors if type is not clear
                                cpp_out.push_str(&format!("    auto {} = {};\n", name, value_cpp));
                            }
                        }
                    }
                    AssignmentOperator::AddAssign => {
                        cpp_out.push_str(&format!("    {} += {};\n", name, value_cpp));
                    }
                    AssignmentOperator::SubAssign => {
                        cpp_out.push_str(&format!("    {} -= {};\n", name, value_cpp));
                    }
                    AssignmentOperator::MulAssign => {
                        cpp_out.push_str(&format!("    {} *= {};\n", name, value_cpp));
                    }
                    AssignmentOperator::DivAssign => {
                        cpp_out.push_str(&format!("    {} /= {};\n", name, value_cpp));
                    }
                    AssignmentOperator::ModAssign => {
                        cpp_out.push_str(&format!("    {} %= {};\n", name, value_cpp));
                    }
                    AssignmentOperator::PowAssign => {
                        // C++ does not have a direct **= operator. std::pow returns double.
                        // Requires careful handling of types, assuming integer context for now.
                        cpp_out.push_str(&format!("    {} = static_cast<long long>(std::pow({}, {}));\n", name, name, value_cpp));
                    }
                    AssignmentOperator::FloorDivAssign => {
                        // Similar to PowAssign, C++ / for integers truncates.
                        // Python's //= floors. This is a simplification.
                        cpp_out.push_str(&format!("    {} /= {};\n", name, value_cpp)); // Simplified
                    }
                    // Implement other assignment operators as needed
                }
            }
        }
    }
    cpp_out.push_str("    return 0;\n}\n");
    Ok(cpp_out)
}

fn emit_expression_cpp(expr: &Expression) -> String {
    match expr {
        Expression::StringLiteral(s) => format!("\"{}\"", s.replace("\\", "\\\\").replace("\"", "\\\"")),
        Expression::IntegerLiteral(i) => format!("{}", i),
        Expression::Identifier(name) => name.clone(),
        Expression::BinaryOperation { left, op, right } => {
            let l = emit_expression_cpp(left);
            let r = emit_expression_cpp(right);
            let op_str = match op {
                BinOp::Add => "+",
                BinOp::Sub => "-",
                BinOp::Mul => "*",
                BinOp::Div => "/", 
                BinOp::Mod => "%",
                BinOp::Pow => return format!("static_cast<long long>(std::pow({}, {}))", l, r),
                BinOp::FloorDiv => "/", 
                BinOp::Eq => "==",
                BinOp::NotEq => "!=",
                BinOp::Lt => "<",
                BinOp::Gt => ">",
                BinOp::LtEq => "<=",
                BinOp::GtEq => ">=",
                _ => "/* unsupported op */",
            };
            format!("({} {} {})", l, op_str, r)
        }
        _ => String::from("/* unsupported expr */"),
    }
}
