// Codegen module placeholder
use crate::ast::{AstNode, Expression, Statement, BinOp, UnaryOp, AssignmentOperator, Decorator, Argument};
use std::collections::{HashMap, HashSet};

// Placeholder for SymbolTable, FunctionTable, and TypeMap
// These would typically be more complex and live in their own modules (e.g., semantic or typechecker)

// Define VariableInfo struct for symbol table entries
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub type_name: String,
    pub is_const: bool,
}

pub struct SymbolTable {
    scopes: Vec<HashMap<String, VariableInfo>>, // var_name -> VariableInfo
    current_scope_index: usize,
}

impl SymbolTable {    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            current_scope_index: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        } else {
            // Error or warning: trying to exit global scope
        }
    }    pub fn add_variable(&mut self, name: &str, var_type: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), VariableInfo { 
                type_name: var_type.to_string(), 
                is_const: false 
            });
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&VariableInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(var_info) = scope.get(name) {
                return Some(var_info);
            }
        }
        None
    }
    // Add a fork method for lambda scopes or similar isolated contexts
    pub fn fork(&self) -> SymbolTable {
        // Create a new SymbolTable that inherits or can access outer scopes,
        // but modifications are isolated. For simplicity, let's start fresh for lambda body,
        // relying on C++ capture for outer variables.
        // A more sophisticated fork would clone the current scope chain or link to it.
        // For [=] capture, the lambda body doesn't strictly need to see outer E++ scopes
        // during its own C++ code generation, as C++ handles the capture.
        // However, for type checking or more complex captures, this would be important.
        // For now, a simple new scope is sufficient for codegen of [=] lambdas.
        let new_table = SymbolTable::new();
        // If we want to provide read-only access to outer scopes for some reason:
        // new_table.scopes = self.scopes.clone(); // This would require VariableInfo to be Clone
        // new_table.enter_scope(); // Then push a new mutable scope for the lambda itself.
        // For [=] capture, the C++ lambda itself handles access to captured variables.
        // The E++ symbol table within the lambda body primarily needs to know about lambda parameters.
        new_table // Returns a new, independent symbol table for the lambda's parameters and body.
                  // This is a simplification. Proper closure support would need more.
    }
}

pub struct FunctionSignature {
    #[allow(dead_code)]
    pub param_types: Vec<String>, // Simplified: type names as strings
    pub return_type: String,    // Simplified: type name as string
}

pub struct FunctionTable {
    functions: HashMap<String, FunctionSignature>, // func_name -> signature
}

impl FunctionTable {
    pub fn new() -> Self {
        FunctionTable { functions: HashMap::new() }
    }

    #[allow(dead_code)]
    pub fn add_function(&mut self, name: &str, signature: FunctionSignature) {
        self.functions.insert(name.to_string(), signature);
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionSignature> {
        self.functions.get(name)
    }
}

// TypeMap could be used for more advanced type inference or checking, 
// mapping AST node IDs or variable names to resolved types.
// For now, it can be simple or integrated into SymbolTable/FunctionTable if not heavily used.
pub struct TypeMap {
    #[allow(dead_code)]
    types: HashMap<String, String>, // Placeholder: e.g., variable_name -> concrete_type
}

impl TypeMap {
    pub fn new() -> Self {
        TypeMap { types: HashMap::new() }
    }
    // Add methods as needed, e.g., to store and retrieve type information
}

pub fn generate_cpp_code_with_toplevel(ast_nodes: &[AstNode], is_toplevel: bool) -> Result<String, String> {
    let mut declared_vars = HashSet::new();
    let mut symbol_table = SymbolTable::new();
    let mut function_table = FunctionTable::new(); // Made mutable
    let mut type_map = TypeMap::new();
    _generate_cpp_code_with_vars(ast_nodes, is_toplevel, &mut declared_vars, &mut symbol_table, &mut function_table, &mut type_map)
}

fn _generate_cpp_code_with_vars(
    ast_nodes: &[AstNode],
    is_toplevel: bool,
    declared_vars: &mut HashSet<String>,
    symbol_table: &mut SymbolTable,
    function_table: &mut FunctionTable, // Made mutable
    type_map: &mut TypeMap,
) -> Result<String, String> {
    let mut cpp_out = String::new();
    if is_toplevel {
        cpp_out.push_str("#include <iostream>\n");
        cpp_out.push_str("#include <string>\n");
        cpp_out.push_str("#include <vector>\n");
        cpp_out.push_str("#include <algorithm>\n");
        cpp_out.push_str("#include <cmath> // For std::pow\n");
        cpp_out.push_str("#include <complex> // For std::complex\n");
        cpp_out.push_str("#include <tuple>   // For std::tuple\n");
        cpp_out.push_str("#include <map>     // For std::map\n");
        cpp_out.push_str("#include <set>     // For std::set\n");
        cpp_out.push_str("#include <unordered_set> // For std::unordered_set\n\n");

        // Basic print functions
        cpp_out.push_str("void eppx_print(const std::string& s) { std::cout << s << std::endl; }\n");
        cpp_out.push_str("void eppx_print(long long x) { std::cout << x << std::endl; }\n");
        cpp_out.push_str("void eppx_print(double x) { std::cout << x << std::endl; }\n");
        cpp_out.push_str("void eppx_print(bool b) { std::cout << (b ? \"true\" : \"false\") << std::endl; }\n");
        cpp_out.push_str("void eppx_print(const std::complex<long long>& c) { std::cout << \"(\" << c.real() << (c.imag() >= 0 ? \"+\" : \"\") << c.imag() << \"j)\" << std::endl; }\n");
        cpp_out.push_str("void eppx_print(const std::complex<double>& c) { std::cout << \"(\" << c.real() << (c.imag() >= 0 ? \"+\" : \"\") << c.imag() << \"j)\" << std::endl; }\n");
        cpp_out.push_str("void eppx_print(std::nullptr_t) { std::cout << \"None\" << std::endl; }\n");
        // Print functions for data structures (placeholders)
        cpp_out.push_str("template<typename T> void eppx_print(const std::vector<T>& vec) { std::cout << \"list object (size: \" << vec.size() << \")\" << std::endl; }\n");
        cpp_out.push_str("template<typename K, typename V> void eppx_print(const std::map<K, V>& m) { std::cout << \"dict object (size: \" << m.size() << \")\" << std::endl; }\n");
        cpp_out.push_str("template<typename T> void eppx_print(const std::set<T>& s) { std::cout << \"set object (size: \" << s.size() << \")\" << std::endl; }\n");
        cpp_out.push_str("template<typename T> void eppx_print(const std::unordered_set<T>& s) { std::cout << \"frozenset object (size: \" << s.size() << \")\" << std::endl; }\n");
        cpp_out.push_str("template <typename... Args> void eppx_print(const std::tuple<Args...>& t) { std::cout << \"tuple object (size: \" << sizeof...(Args) << \")\" << std::endl; }\n\n");

        // Simple range helper for for loops
        cpp_out.push_str("std::vector<long long> eppx_range(long long n) {\n");
        cpp_out.push_str("    std::vector<long long> result;\n");
        cpp_out.push_str("    for (long long i = 0; i < n; ++i) {\n");
        cpp_out.push_str("        result.push_back(i);\n");
        cpp_out.push_str("    }\n");
        cpp_out.push_str("    return result;\n");
        cpp_out.push_str("}\n\n");
        // Helper for creating frozenset
        cpp_out.push_str("template<typename T> std::unordered_set<T> eppx_internal_make_frozenset(const std::vector<T>& initial_elements) { return std::unordered_set<T>(initial_elements.begin(), initial_elements.end()); }\n\n");
    }    // First pass: emit all function definitions and class definitions at the top level
    // This helps with C++'s requirement for declaration before use.
    for node in ast_nodes {        match node {            AstNode::Statement(Statement::FunctionDef { name, params, body, decorators }) => {
                // Generate decorator-wrapped function
                let decorator_wrappers = generate_decorator_wrappers(decorators)?;
                
                // Generate template parameters and function parameter list
                let mut template_params_gen = Vec::new();
                let mut call_params_gen = Vec::new();
                let mut param_types_for_signature = Vec::new();

                symbol_table.enter_scope(); // Scope for function parameters and body

                for (i, p_name) in params.iter().enumerate() {
                    let type_param_name = format!("T{}", i);
                    template_params_gen.push(format!("typename {}", type_param_name));
                    call_params_gen.push(format!("{} {}", type_param_name, p_name));
                    param_types_for_signature.push(type_param_name.clone());
                    // Add parameter to symbol table with its generic type
                    symbol_table.add_variable(p_name, &type_param_name);
                }                let template_clause = if !template_params_gen.is_empty() {
                    format!("template<{}>\n", template_params_gen.join(", "))
                } else {
                    "".to_string()
                };
                let param_list_cpp = call_params_gen.join(", ");

                // Populate FunctionTable
                let sig = FunctionSignature { param_types: param_types_for_signature, return_type: "auto".to_string() };
                function_table.add_function(name, sig);

                // Function body (symbol_table already has params in its current scope)
                let mut function_body_declared_vars = HashSet::new();
                let body_cpp = _generate_cpp_code_with_vars(body, false, &mut function_body_declared_vars, symbol_table, function_table, type_map)?;
                  symbol_table.exit_scope(); // End of function scope

                // Add decorator wrapper comments/code
                cpp_out.push_str(&decorator_wrappers);
                cpp_out.push_str(&format!("{}auto {}({}) {{\n", template_clause, name, param_list_cpp));
                cpp_out.push_str(&body_cpp);
                let has_return = body.iter().any(|node| matches!(node, AstNode::Statement(Statement::Return(_))));
                if !has_return {
                    // Default return for void-like functions or if E++ allows implicit None return
                    // C++ functions returning auto must have a return statement.
                    // For simplicity, if E++ implies returning 0 or void:
                    // This might need adjustment based on E++ function semantics.
                    // If it's truly auto, it must deduce from a return.
                    // Let's assume functions implicitly return 0 if no other return.
                    cpp_out.push_str("    return 0; // Default return if none explicit\n");
                }
                cpp_out.push_str("}\n\n");
            }
            AstNode::Statement(Statement::ClassDef { name, body }) => {
                cpp_out.push_str(&format!("struct {} {{\n", name));
                // Process class body for static members and methods
                // This is a simplified model: assignments become static members, defs become methods.
                // No special handling for __init__ or self yet.
                for class_node in body {
                    match class_node {
                        AstNode::Statement(Statement::Assignment { name: member_name, operator: AssignmentOperator::Assign, value }) => {
                            // Generate static inline member
                            let value_cpp = emit_expression_cpp(value, symbol_table, function_table, type_map)?;
                            let type_str = infer_cpp_type_for_static_member(value);
                            cpp_out.push_str(&format!("    static inline {} {} = {};\n", type_str, member_name, value_cpp));
                        }                        AstNode::Statement(Statement::FunctionDef { name: method_name, params, body: _method_body, decorators: _ }) => {
                            // Generate member function (method)
                            cpp_out.push_str(&emit_method_cpp(method_name, params, _method_body)?);
                        }
                        _ => { /* Other statements in class body might be ignored or handled later */ }
                    }
                }
                cpp_out.push_str("};\n\n");
            }
            _ => {} // Other statement types are handled in the second pass (for main's body)
        }
    }
    if is_toplevel {
        cpp_out.push_str("int main() {\n");
    }
    // Second pass: emit all non-function statements (main body)
    for node in ast_nodes {
        if matches!(node, AstNode::Statement(Statement::FunctionDef { .. }) | AstNode::Statement(Statement::ClassDef { .. })) {
            continue;
        }
        // DEBUG: Print node kind for troubleshooting
        // eprintln!("Codegen node: {:?}", node); // Removed debug print
        match node {            AstNode::Statement(Statement::Print(expr)) => {
                let expr_code = emit_expression_cpp(expr, symbol_table, function_table, type_map)?;
                cpp_out.push_str(&format!("    eppx_print({});\n", expr_code));
            }
            AstNode::Statement(Statement::Assignment { name, operator, value }) => {
                let value_cpp = emit_expression_cpp(value, symbol_table, function_table, type_map)?;
                if !declared_vars.contains(name) {                    // Infer type for declaration (simplified)
                    let type_str = match &**value {
                        Expression::IntegerLiteral(_) => "long long".to_string(),
                        Expression::FloatLiteral(_) => "double".to_string(),
                        Expression::StringLiteral(_) => "std::string".to_string(),
                        Expression::BooleanLiteral(_) => "bool".to_string(),
                        Expression::Lambda { .. } => "auto".to_string(),
                        _ => "auto".to_string(),
                    };

                    match operator {
                        AssignmentOperator::Assign => {
                            cpp_out.push_str(&format!("    {} {} = {};\n", type_str, name, value_cpp));
                        }
                        // For other operators, it implies the variable must already exist or this is a shorthand.
                        // This logic might need refinement if E++ allows `+=` etc. on first assignment.
                        // Assuming for now that `+=` etc. require prior declaration.
                        _ => { // This case should ideally be an error if var not declared and not simple assign
                           cpp_out.push_str(&format!("    {} {} = {}; // WARN: Compound assignment on new var\n", type_str, name, value_cpp));
                        }
                    }
                    declared_vars.insert(name.clone());
                    symbol_table.add_variable(name, &type_str); // Add to symbol table
                } else {
                    // Variable already declared, apply assignment operator
                    match operator {
                        AssignmentOperator::Assign => {
                            cpp_out.push_str(&format!("    {} = {};\n", name, value_cpp));
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
                            cpp_out.push_str(&format!("    {} = static_cast<long long>(std::pow(static_cast<double>({}), static_cast<double>({})));\n", name, name, value_cpp));
                        }
                        AssignmentOperator::FloorDivAssign => {
                            cpp_out.push_str(&format!("    {} = static_cast<long long>(std::floor(static_cast<double>({}) / static_cast<double>({})));\n", name, name, value_cpp));
                        }
                        AssignmentOperator::BitAndAssign => {
                            cpp_out.push_str(&format!("    {} &= {};\n", name, value_cpp));
                        }                        AssignmentOperator::BitOrAssign => {
                            cpp_out.push_str(&format!("    {} |= {};\n", name, value_cpp));
                        }
                        AssignmentOperator::BitXorAssign => {
                            cpp_out.push_str(&format!("    {} ^= {};\n", name, value_cpp));
                        }
                        AssignmentOperator::LShiftAssign => {
                            cpp_out.push_str(&format!("    {} <<= {};\n", name, value_cpp));
                        }
                        AssignmentOperator::RShiftAssign => {
                            cpp_out.push_str(&format!("    {} >>= {};\n", name, value_cpp));
                        }
                    }
                }
            }
            AstNode::Statement(Statement::If { condition, then_body, elifs, else_body }) => {
                let mut chain = String::new();                let emit_block = |stmts: &Vec<AstNode>, declared_vars: &mut HashSet<String>, symbol_table: &mut SymbolTable, function_table: &mut FunctionTable, type_map: &mut TypeMap| -> Result<String, String> {
                    let mut block_symbol_table = symbol_table.fork(); // Fork for new scope
                    block_symbol_table.enter_scope();
                    let mut block = String::new();
                    for stmt in stmts {
                        let inner = _generate_cpp_code_with_vars(&[stmt.clone()], false, declared_vars, &mut block_symbol_table, function_table, type_map)?;
                        for line in inner.lines() {
                            block.push_str("    "); // Indent
                            block.push_str(line);
                            block.push('\n');
                        }
                    }
                    block_symbol_table.exit_scope();
                    Ok(block)
                };
                let cond_cpp = emit_expression_cpp(condition, symbol_table, function_table, type_map)?;
                chain.push_str(&format!("    if ({}) {{\n", cond_cpp));
                chain.push_str(&emit_block(then_body, declared_vars, symbol_table, function_table, type_map)?);
                chain.push_str("    }");
                for (elif_cond, elif_body) in elifs {
                    let elif_cond_cpp = emit_expression_cpp(elif_cond, symbol_table, function_table, type_map)?;
                    chain.push_str(&format!(" else if ({}) {{\n", elif_cond_cpp));
                    chain.push_str(&emit_block(elif_body, declared_vars, symbol_table, function_table, type_map)?);
                    chain.push_str("    }");
                }
                if let Some(else_body_nodes) = else_body { // Renamed to avoid conflict
                    chain.push_str(" else {\n");
                    chain.push_str(&emit_block(&else_body_nodes, declared_vars, symbol_table, function_table, type_map)?);
                    chain.push_str("    }");
                }
                chain.push_str("\n");
                cpp_out.push_str(&chain);
            }            AstNode::Statement(Statement::While { condition, body }) => {
                let emit_block = |stmts: &Vec<AstNode>, declared_vars: &mut HashSet<String>, symbol_table: &mut SymbolTable, function_table: &mut FunctionTable, type_map: &mut TypeMap| -> Result<String, String> {
                    let mut block_symbol_table = symbol_table.fork();
                    block_symbol_table.enter_scope();
                    let mut block = String::new();
                    for stmt in stmts {
                        let inner = _generate_cpp_code_with_vars(&[stmt.clone()], false, declared_vars, &mut block_symbol_table, function_table, type_map)?;
                        for line in inner.lines() {
                            block.push_str("    "); // Indent
                            block.push_str(line);
                            block.push('\n');
                        }
                    }
                    block_symbol_table.exit_scope();
                    Ok(block)
                };
                let cond_cpp = emit_expression_cpp(condition, symbol_table, function_table, type_map)?;
                let mut while_code = String::new();
                while_code.push_str(&format!("    while ({}) {{\n", cond_cpp));
                while_code.push_str(&emit_block(body, declared_vars, symbol_table, function_table, type_map)?);
                while_code.push_str("    }\n");
                cpp_out.push_str(&while_code);
            }            AstNode::Statement(Statement::For { var, iterable, body }) => {
                let emit_block = |stmts: &Vec<AstNode>, declared_vars: &mut HashSet<String>, symbol_table: &mut SymbolTable, function_table: &mut FunctionTable, type_map: &mut TypeMap| -> Result<String, String> {
                    let mut block_symbol_table = symbol_table.fork();
                    block_symbol_table.enter_scope();
                    // Add loop variable to scope if not already (it's declared by the for loop construct)
                    // This depends on how E++ scoping for loop vars works. Assuming it's part of the inner scope.
                    block_symbol_table.add_variable(var, "auto"); // Type might be inferred from iterable

                    let mut block = String::new();
                    for stmt in stmts {
                        let inner = _generate_cpp_code_with_vars(&[stmt.clone()], false, declared_vars, &mut block_symbol_table, function_table, type_map)?;
                        for line in inner.lines() {
                            block.push_str("        "); // Further indent for for-loop body
                            block.push_str(line);
                            block.push('\n');
                        }
                    }
                    block_symbol_table.exit_scope();
                    Ok(block)
                };
                let iterable_cpp = emit_expression_cpp(iterable, symbol_table, function_table, type_map)?;
                let mut for_code = String::new();
                // Ensure loop variable is declared if it's the first time.
                // C++ for-range declares the variable in its scope.
                // We need to ensure `var` is known to the symbol_table for the body.
                // The `emit_block` above handles adding `var` to its new scope.

                // For C++ range-based for, the variable is declared in the loop.
                // We don't need to pre-declare `var` outside if it's a fresh variable for the loop.
                // If `var` shadows an outer variable, C++ handles that naturally.
                for_code.push_str(&format!("    for (auto {} : {}) {{\n", var, iterable_cpp));
                // No need for: for_code.push_str(&format!("        {} = {}_val;\n", var, var));
                // The `var` in `auto var : iterable_cpp` is the loop variable.
                for_code.push_str(&emit_block(body, declared_vars, symbol_table, function_table, type_map)?); // Pass original symbol_table, emit_block creates sub-scope
                for_code.push_str("    }\n");
                cpp_out.push_str(&for_code);
            }
            AstNode::Statement(Statement::Return(expr)) => {
                if let Some(return_expr) = expr {
                    let return_value = emit_expression_cpp(return_expr, symbol_table, function_table, type_map)?;
                    cpp_out.push_str(&format!("    return {};\n", return_value));
                } else {
                    cpp_out.push_str("    return;\n");
                }
            }
            AstNode::Statement(Statement::ExpressionStatement(expr)) => {
                match &**expr { // Dereference expr to match against Expression
                    Expression::Identifier(name) if name == "pass" => {
                        cpp_out.push_str("    ; // pass statement\n");
                    }
                    _ => {
                        let expr_code = emit_expression_cpp(expr, symbol_table, function_table, type_map)?;
                        cpp_out.push_str(&format!("    {};\n", expr_code));
                    }
                }
            }
            AstNode::Statement(Statement::Break) => {
                cpp_out.push_str("    break;\n");
            }
            AstNode::Statement(Statement::Continue) => {
                cpp_out.push_str("    continue;\n");
            }
            AstNode::Statement(Statement::Pass) => {
                cpp_out.push_str("    ; // pass statement\n");
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
    // Initialize tables here as well if this is an alternative entry point
    let mut symbol_table = SymbolTable::new();
    let mut function_table = FunctionTable::new();
    let mut type_map = TypeMap::new();
    let mut declared_vars = HashSet::new(); // declared_vars for the main/global scope
    _generate_cpp_code_with_vars(ast_nodes, true, &mut declared_vars, &mut symbol_table, &mut function_table, &mut type_map)
}

pub fn emit_expression_cpp(
    expr: &Expression,
    symbol_table: &mut SymbolTable,
    function_table: &FunctionTable,
    type_map: &mut TypeMap,
) -> Result<String, String> {
    match expr {
        Expression::StringLiteral(s) => {
            let escaped_s = s.replace("\\", "\\\\").replace("\"", "\\\"");
            Ok(format!("std::string(\"{}\")", escaped_s))
        }
        Expression::IntegerLiteral(i) => Ok(format!("{}LL", i)), // Suffix LL for long long
        Expression::FloatLiteral(f) => Ok(format!("{}", f)), // Float literals
        Expression::NoneLiteral => Ok("nullptr".to_string()),
        Expression::BooleanLiteral(b) => Ok(format!("{}", b)),
        Expression::Identifier(name) => Ok(name.clone()),
        Expression::UnaryOperation { op, operand } => {
            let operand_cpp = emit_expression_cpp(operand, symbol_table, function_table, type_map)?;
            match op {
                UnaryOp::Not => Ok(format!("!({})", operand_cpp)), // Added parentheses for safety
                UnaryOp::BitNot => Ok(format!("~({})", operand_cpp)), // Implement BitNot
            }
        }
        Expression::ListLiteral(elements) => {
            if elements.is_empty() {
                // Default to std::vector<long long> for empty lists.
                // Consider std::vector<std::any> or std::vector<std::monostate> if more general empty lists are needed.
                Ok("std::vector<long long>{}".to_string())
            } else {
                let mut elements_cpp = Vec::new();
                for el in elements {
                    elements_cpp.push(emit_expression_cpp(el, symbol_table, function_table, type_map)?);
                }
                // Use C++17 Class Template Argument Deduction (CTAD)
                Ok(format!("std::vector{{{}}}", elements_cpp.join(", ")))
            }
        }
        Expression::Call { callee, args } => {
            let mut args_cpp = Vec::new();
            for arg in args {
                args_cpp.push(emit_expression_cpp(arg, symbol_table, function_table, type_map)?);
            }

            // Handle special built-in functions first
            if let Expression::Identifier(name) = &**callee {
                if name == "range" && args.len() == 1 {
                    // args_cpp will have one element here
                    return Ok(format!("eppx_range({})", args_cpp[0]));
                }
                // Add other special functions like print (if it were an expression), len, etc.
                // if name == "len" && args.len() == 1 {
                //     return Ok(format!("{}.size()", args_cpp[0])); // Example for a vector/string
                // }
            }
            
            // Generic function call
            let callee_cpp = emit_expression_cpp(callee, symbol_table, function_table, type_map)?;
            Ok(format!("{}({})", callee_cpp, args_cpp.join(", ")))
        }
        Expression::Lambda { params, body } => {
            // symbol_table.enter_scope(); // Lambda introduces a new scope
            // let params_cpp = params.iter().map(|p| format!("auto {}", p)).collect::<Vec<String>>().join(", ");
            // for p_name in params {
            //     symbol_table.add_variable(p_name, "auto"); // Add lambda params to its scope
            // }
            // let body_cpp = emit_expression_cpp(body, symbol_table, function_table, type_map)?; // Lambdas often return expressions
            // symbol_table.exit_scope();
            // Ok(format!("[=]({}) {{ return {}; }})", params_cpp, body_cpp))
            // Simplified lambda generation, proper scoping for body needs care if it's a block
            
            // Create a temporary new symbol table for the lambda's scope to avoid interference
            let mut lambda_symbol_table = symbol_table.fork(); // Assumes SymbolTable has a fork method or similar for isolated scopes

            let params_cpp = params.iter().map(|p| format!("auto {}", p)).collect::<Vec<String>>().join(", ");
            for p_name in params {
                lambda_symbol_table.add_variable(p_name, "auto");
            }

            // Lambdas in E++ have a single expression as body.
            // If the body were a block of statements, it would need _generate_cpp_code_with_vars
            let body_cpp = emit_expression_cpp(body, &mut lambda_symbol_table, function_table, type_map)?;
            
            Ok(format!("([=]({}) {{ return {}; }})", params_cpp, body_cpp))
        }
        Expression::BinaryOperation { left, op, right } => {
            let l = emit_expression_cpp(left, symbol_table, function_table, type_map)?;
            let r = emit_expression_cpp(right, symbol_table, function_table, type_map)?;
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
                    return Ok(format!("{} && {}", l, r));
                },
                BinOp::Or => {
                    // In Python, 'or' returns the first truthy value or the last value  
                    // In C++, || returns bool. For simplicity, we'll use || but cast to bool context
                    return Ok(format!("{} || {}", l, r));
                },
                // Bitwise
                BinOp::BitAnd => "&",
                BinOp::BitOr => "|",
                BinOp::BitXor => "^",
                BinOp::LShift => "<<",
                BinOp::RShift => ">>",
                // Identity (basic C++ translation, not Python's object identity)
                BinOp::Is => return Ok(format!("{} == {} /* Placeholder for IS */", l, r)), // Primitive check
                BinOp::IsNot => return Ok(format!("{} != {} /* Placeholder for IS NOT */", l, r)), // Primitive check
                // Membership (basic C++ string translation, not general purpose)
                BinOp::In => {
                    // Very basic string 'in' check. Assumes l is char/substring, r is string.
                    // This is a placeholder and needs a proper runtime type system.
                    // Example: r.find(l) != std::string::npos
                    // For now, let's assume r is a string and l is a string to find.
                    // This is highly simplified.
                    return Ok(format!("{}.find({}) != std::string::npos /* Placeholder for IN */", r, l));
                }
                BinOp::NotIn => {
                    return Ok(format!("{}.find({}) == std::string::npos /* Placeholder for NOT IN */", r, l));
                }
            };
            Ok(format!("{} {} {}", l, op_str, r))
        }
        Expression::TupleLiteral(elements) => {
            Ok(format!("std::make_tuple({})", elements.iter().map(|e| emit_expression_cpp(e, symbol_table, function_table, type_map)).collect::<Result<Vec<_>,_>>()?.join(", ")))
        }
        Expression::DictLiteral(entries) => {
            let entries_cpp = entries.iter().map(|(k, v)| {
                let k_cpp = emit_expression_cpp(k, symbol_table, function_table, type_map)?;
                let v_cpp = emit_expression_cpp(v, symbol_table, function_table, type_map)?;
                Ok::<String, String>(format!("{{{}, {}}}", k_cpp, v_cpp))
            }).collect::<Result<Vec<_>,_>>()?.join(", ");
            Ok(format!("std::map<std::string, long long>{{{}}}", entries_cpp)) // Assuming K=string, V=long long
        }
        Expression::SetLiteral(elements) => {
            let elems_cpp = elements.iter().map(|e| emit_expression_cpp(e, symbol_table, function_table, type_map)).collect::<Result<Vec<_>,_>>()?.join(", ");
            Ok(format!("std::set<long long>{{{}}}", elems_cpp)) // Assuming T=long long
        }
        Expression::FrozensetLiteral(elements) => {
            // C++ doesn't have a direct frozenset. std::set is mutable.
            // For const-correctness, it would be `const std::set<T>`.
            // The helper `eppx_internal_make_frozenset` returns `std::unordered_set`.
            // Let's align with that or use `std::set` and rely on `const` at assignment.
            let elems_cpp = elements.iter().map(|e| emit_expression_cpp(e, symbol_table, function_table, type_map)).collect::<Result<Vec<_>,_>>()?.join(", ");
            // Using std::unordered_set as per the helper function provided earlier.
            // This requires elements to be collected into a vector first for the helper.
            // Ok(format!("eppx_internal_make_frozenset<long long>(std::vector<long long>{{{}}})", elems_cpp))
            // Simpler: use std::set and rely on const if variable is const. Or use unordered_set directly.
            Ok(format!("std::unordered_set<long long>{{{}}}", elems_cpp)) // Assuming T=long long
        }
        Expression::ComplexLiteral(real, imag) => {
            let real_cpp = emit_expression_cpp(real, symbol_table, function_table, type_map)?;
            let imag_cpp = emit_expression_cpp(imag, symbol_table, function_table, type_map)?;
            Ok(format!("std::complex<double>({}, {})", real_cpp, imag_cpp))
        }
        // The duplicate Lambda match arm that was here (around lines 450-458) is removed.
        // The primary Expression::Lambda handler is earlier and correctly uses symbol_table.
        _ => Err(String::from("Unsupported expression type for C++ codegen"))
    }
}

// Helper to map E++ type names to C++ type names
fn map_type_to_cpp(epp_type: &str) -> String {
    match epp_type {
        "int" => "int".to_string(),
        "float" => "double".to_string(),
        "string" => "std::string".to_string(),
        "bool" => "bool".to_string(),
        "void" => "void".to_string(),
        _ => "auto".to_string(),
    }
}

fn _emit_cpp_for_function_definition(
    name: &str,
    params: &Vec<String>, // Now correctly just names
    _body: &Statement,
    return_type_hint: &Option<String>,
    _symbol_table: &mut SymbolTable,
    _function_table: &FunctionTable,
    _type_map: &mut TypeMap,
) -> Result<String, String> {
    let params_cpp = params
        .iter()
        .map(|p_name| format!("auto {}", p_name)) // Corrected: iterate over p_name directly
        .collect::<Vec<String>>()
        .join(", ");

    let return_type_cpp = match return_type_hint {
        Some(rt) => map_type_to_cpp(rt),
        None => "auto".to_string(),
    };

    Ok(format!(
        "{} {}({})",
        return_type_cpp, name, params_cpp
    ))
}

fn _emit_cpp_for_variable_declaration(
    name: &str,
    type_hint: &Option<String>,
    value: &Expression,
    symbol_table: &mut SymbolTable,
    function_table: &FunctionTable,
    type_map: &mut TypeMap,
    is_const: bool,
) -> Result<String, String> {
    let mut type_cpp_str = "auto".to_string();

    if let Some(hint) = type_hint {
        type_cpp_str = map_type_to_cpp(hint);
    }

    // Corrected: Pass all required arguments to emit_expression_cpp
    let value_cpp = emit_expression_cpp(value, symbol_table, function_table, type_map)?;    if type_hint.is_none() || type_hint.as_deref() == Some("auto") {
        match value {
            Expression::IntegerLiteral(_) => type_cpp_str = "long long".to_string(),
            Expression::FloatLiteral(_) => type_cpp_str = "double".to_string(),
            Expression::StringLiteral(_) => type_cpp_str = "std::string".to_string(),
            Expression::BooleanLiteral(_) => type_cpp_str = "bool".to_string(),
            Expression::ListLiteral(_elements) => {
                // type_cpp_str = "std::vector<auto>".to_string(); // Keep auto for now
            }
            Expression::Lambda { .. } => {
                type_cpp_str = "auto".to_string();
            }
            Expression::Call { callee, .. } => {
                if let Expression::Identifier(func_name) = &**callee {
                    if let Some(func_info) = function_table.get_function(func_name) {
                        type_cpp_str = func_info.return_type.clone();
                    }
                }
            }
            _ => {}
        }
    }

    let const_qualifier = if is_const { "const " } else { "" };
    let declaration = format!(
        "{}{} {} = {};",
        const_qualifier, type_cpp_str, name, value_cpp
    );
    symbol_table.add_variable(name, &type_cpp_str); // Assuming add_variable now takes (name, type)
    Ok(declaration)
}

// Helper functions for code generation

fn indent_code(code: &str) -> String {
    code.lines()
        .map(|line| if line.trim().is_empty() { line.to_string() } else { format!("    {}", line) })
        .collect::<Vec<String>>()
        .join("\n")
}

fn infer_cpp_type_for_static_member(expr: &Expression) -> String {
    match expr {
        Expression::IntegerLiteral(_) => "long long".to_string(),
        Expression::FloatLiteral(_) => "double".to_string(),
        Expression::StringLiteral(_) => "std::string".to_string(),
        Expression::BooleanLiteral(_) => "bool".to_string(),
        _ => "auto".to_string(),
    }
}

fn emit_method_cpp(method_name: &str, params: &[String], _body: &[AstNode]) -> Result<String, String> {
    let params_cpp = params.iter().map(|p| format!("auto {}", p)).collect::<Vec<String>>().join(", ");
    Ok(format!("auto {}({});", method_name, params_cpp))
}

fn generate_decorator_wrappers(decorators: &[Decorator]) -> Result<String, String> {
    let mut wrapper_code = String::new();
    
    for decorator in decorators {
        match decorator {
            Decorator::Simple(name) => {
                wrapper_code.push_str(&format!("// @{} decorator\n", name));
                match name.as_str() {
                    "timer" => {
                        wrapper_code.push_str("// Timer decorator: measures execution time\n");
                    }
                    "staticmethod" => {
                        wrapper_code.push_str("// Static method decorator\n");
                    }
                    "property" => {
                        wrapper_code.push_str("// Property decorator\n");
                    }
                    "cache" => {
                        wrapper_code.push_str("// Cache decorator: memoizes function results\n");
                    }
                    _ => {
                        wrapper_code.push_str(&format!("// Unknown decorator: {}\n", name));
                    }
                }
            }            Decorator::WithArgs(name, args) => {
                wrapper_code.push_str(&format!("// @{}(...) decorator with {} arguments\n", name, args.len()));
                
                // Generate argument info for debugging
                for (i, arg) in args.iter().enumerate() {
                    match arg {
                        Argument::Positional(expr) => {
                            wrapper_code.push_str(&format!("// Arg {}: positional\n", i));
                        }
                        Argument::Keyword(name, expr) => {
                            wrapper_code.push_str(&format!("// Arg {}: {}=<value>\n", i, name));
                        }
                    }
                }
                
                match name.as_str() {
                    "retry" => {
                        wrapper_code.push_str("// Retry decorator: retries function on failure\n");
                        // Look for 'times' keyword argument
                        for arg in args {
                            if let Argument::Keyword(param_name, _expr) = arg {
                                if param_name == "times" {
                                    wrapper_code.push_str("// Found 'times' parameter for retry\n");
                                }
                            }
                        }
                    }
                    _ => {
                        wrapper_code.push_str(&format!("// Unknown decorator with args: {}\n", name));
                    }
                }
            }
        }
    }
    
    Ok(wrapper_code)
}

