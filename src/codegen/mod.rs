// Codegen module placeholder
use crate::ast::{AstNode, Expression, Statement, BinOp, UnaryOp, AssignmentOperator, Comprehension};
use std::collections::{HashMap, HashSet};

// Placeholder for SymbolTable, FunctionTable, and TypeMap
// These would typically be more complex and live in their own modules (e.g., semantic or typechecker)

// Define VariableInfo struct for symbol table entries
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub type_name: String,
    pub is_const: bool,
}

#[allow(dead_code)]
pub struct SymbolTable {
    scopes: Vec<HashMap<String, VariableInfo>>, // var_name -> VariableInfo
    current_scope_index: usize,
}

#[allow(dead_code)]
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
        Self {
            scopes: self.scopes.clone(),
            current_scope_index: self.current_scope_index,
        }
    }
}

#[allow(dead_code)]
pub struct FunctionSignature {
    #[allow(dead_code)]
    pub param_types: Vec<String>, // Simplified: type names as strings
    pub return_type: String,    // Simplified: type name as string
}

pub struct FunctionTable {
    functions: HashMap<String, FunctionSignature>, // func_name -> signature
}

#[allow(dead_code)]
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

fn indent_code(code: &str) -> String {
    code.lines().map(|line| format!("    {}", line)).collect::<Vec<_>>().join("\n") + "\n"
}

#[allow(dead_code)]
pub fn generate_cpp_code_with_toplevel(ast_nodes: &[AstNode], is_toplevel: bool) -> Result<String, String> {
    let mut declared_vars = HashSet::new();
    let mut symbol_table = SymbolTable::new();
    let mut function_table = FunctionTable::new(); // Made mutable
    let mut type_map = TypeMap::new();
    _generate_cpp_code_with_vars(ast_nodes, is_toplevel, &mut declared_vars, &mut symbol_table, &mut function_table, &mut type_map)
}

fn generate_statement_list_cpp(
    ast_nodes: &[AstNode],
    declared_vars: &mut HashSet<String>,
    symbol_table: &mut SymbolTable,
    function_table: &mut FunctionTable,
    type_map: &mut TypeMap,
) -> Result<String, String> {
    let mut cpp_out = String::new();
    for node in ast_nodes {
        if matches!(node, AstNode::Statement(Statement::FunctionDef { .. }) | AstNode::Statement(Statement::ClassDef { .. })) {
            continue;
        }
          match node {
            AstNode::Statement(Statement::Assignment { target, operator, value }) => {
                let value_cpp = emit_expression_cpp(value, symbol_table, function_table, type_map)?;
                let target_cpp = emit_expression_cpp(target, symbol_table, function_table, type_map)?;
                let is_simple_var = matches!(**target, Expression::Identifier(_));
                if is_simple_var && !declared_vars.contains(&target_cpp) {
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
                            cpp_out.push_str(&format!("    {} {} = {};
",
 type_str, target_cpp, value_cpp));
                        }
                        _ => {
                            cpp_out.push_str(&format!("    {} {} = {}; // WARN: Compound assignment on new var
",
 type_str, target_cpp, value_cpp));
                        }
                    }
                    declared_vars.insert(target_cpp.clone());
                    symbol_table.add_variable(&target_cpp, &type_str);
                } else {
                    match operator {
                        AssignmentOperator::Assign => cpp_out.push_str(&format!("    {} = {};
",
 target_cpp, value_cpp)),
                        AssignmentOperator::AddAssign => cpp_out.push_str(&format!("    {} += {};
",
 target_cpp, value_cpp)),
                        AssignmentOperator::SubAssign => cpp_out.push_str(&format!("    {} -= {};
",
 target_cpp, value_cpp)),
                        AssignmentOperator::MulAssign => cpp_out.push_str(&format!("    {} *= {};
",
 target_cpp, value_cpp)),
                        AssignmentOperator::DivAssign => cpp_out.push_str(&format!("    {} /= {};
",
 target_cpp, value_cpp)),
                        AssignmentOperator::ModAssign => cpp_out.push_str(&format!("    {} %= {};
",
 target_cpp, value_cpp)),
                        AssignmentOperator::PowAssign => cpp_out.push_str(&format!("    {} = static_cast<long long>(std::pow(static_cast<double>({}), static_cast<double>({})));
",
 target_cpp, target_cpp, value_cpp)),
                        AssignmentOperator::FloorDivAssign => cpp_out.push_str(&format!("    {} = static_cast<long long>(std::floor(static_cast<double>({}) / static_cast<double>({})));
",
 target_cpp, target_cpp, value_cpp)),
                        AssignmentOperator::BitAndAssign => cpp_out.push_str(&format!("    {} &= {};
",
 target_cpp, value_cpp)),
                        AssignmentOperator::BitOrAssign => cpp_out.push_str(&format!("    {} |= {};
",
 target_cpp, value_cpp)),
                        AssignmentOperator::BitXorAssign => cpp_out.push_str(&format!("    {} ^= {};
",
 target_cpp, value_cpp)),
                        AssignmentOperator::LShiftAssign => cpp_out.push_str(&format!("    {} <<= {};
",
 target_cpp, value_cpp)),
                        AssignmentOperator::RShiftAssign => cpp_out.push_str(&format!("    {} >>= {};
",
 target_cpp, value_cpp)),
                    }
                }
            }
            AstNode::Statement(Statement::Print(expr)) => {
                match &**expr {
                    Expression::TupleLiteral(args) => {
                        // Multiple arguments - print each separated by space
                        cpp_out.push_str("    std::cout");
                        for (i, arg) in args.iter().enumerate() {
                            if i > 0 {
                                cpp_out.push_str(" << \" \"");
                            }
                            let arg_cpp = emit_expression_cpp(arg, symbol_table, function_table, type_map)?;
                            let safe_arg = match arg {
                                Expression::BinaryOperation { .. } => format!("({})", arg_cpp),
                                _ => arg_cpp,
                            };
                            cpp_out.push_str(&format!(" << {}", safe_arg));
                        }
                        cpp_out.push_str(" << std::endl;\n");
                    }
                    _ => {
                        // Single argument
                        let expr_cpp = emit_expression_cpp(expr, symbol_table, function_table, type_map)?;
                        let safe_expr = match &**expr {
                            Expression::BinaryOperation { .. } => format!("({})", expr_cpp),
                            _ => expr_cpp,
                        };
                        cpp_out.push_str(&format!("    std::cout << {} << std::endl;\n", safe_expr));
                    }
                }
            }
            AstNode::Statement(Statement::If { condition, then_body, elifs, else_body }) => {
                let mut chain = String::new();
                let emit_block = |stmts: &Vec<AstNode>, declared_vars: &mut HashSet<String>, symbol_table: &mut SymbolTable, function_table: &mut FunctionTable, type_map: &mut TypeMap| -> Result<String, String> {
                    let mut block_symbol_table = symbol_table.fork();
                    block_symbol_table.enter_scope();
                    let inner = generate_statement_list_cpp(stmts, declared_vars, &mut block_symbol_table, function_table, type_map)?;
                    block_symbol_table.exit_scope();
                    Ok(indent_code(&inner))
                };
                let cond_cpp = emit_expression_cpp(condition, symbol_table, function_table, type_map)?;
                chain.push_str(&format!("    if ({}) {{
",
 cond_cpp));
                chain.push_str(&emit_block(then_body, declared_vars, symbol_table, function_table, type_map)?);
                chain.push_str("    }");
                for (elif_cond, elif_body) in elifs {
                    let elif_cond_cpp = emit_expression_cpp(elif_cond, symbol_table, function_table, type_map)?;
                    chain.push_str(&format!(" else if ({}) {{
",
 elif_cond_cpp));
                    chain.push_str(&emit_block(elif_body, declared_vars, symbol_table, function_table, type_map)?);
                    chain.push_str("    }");
                }
                if let Some(else_body_nodes) = else_body {
                    chain.push_str(" else {
");
                    chain.push_str(&emit_block(&else_body_nodes, declared_vars, symbol_table, function_table, type_map)?);
                    chain.push_str("    }");
                }
                chain.push_str("
");
                cpp_out.push_str(&chain);
            }
            AstNode::Statement(Statement::While { condition, body }) => {
                let emit_block = |stmts: &Vec<AstNode>, declared_vars: &mut HashSet<String>, symbol_table: &mut SymbolTable, function_table: &mut FunctionTable, type_map: &mut TypeMap| -> Result<String, String> {
                    let mut block_symbol_table = symbol_table.fork();
                    block_symbol_table.enter_scope();
                    let inner = generate_statement_list_cpp(stmts, declared_vars, &mut block_symbol_table, function_table, type_map)?;
                    block_symbol_table.exit_scope();
                    Ok(indent_code(&inner))
                };
                let cond_cpp = emit_expression_cpp(condition, symbol_table, function_table, type_map)?;
                let mut while_code = String::new();
                while_code.push_str(&format!("    while ({}) {{
",
 cond_cpp));
                while_code.push_str(&emit_block(body, declared_vars, symbol_table, function_table, type_map)?);
                while_code.push_str("    }
");
                cpp_out.push_str(&while_code);
            }
            AstNode::Statement(Statement::For { vars, iterable, body }) => {
                let emit_block = |stmts: &Vec<AstNode>, declared_vars: &mut HashSet<String>, symbol_table: &mut SymbolTable, function_table: &mut FunctionTable, type_map: &mut TypeMap| -> Result<String, String> {
                    let mut block_symbol_table = symbol_table.fork();
                    block_symbol_table.enter_scope();
                    for var in vars {
                        block_symbol_table.add_variable(var, "auto");
                    }
                    let inner = generate_statement_list_cpp(stmts, declared_vars, &mut block_symbol_table, function_table, type_map)?;
                    block_symbol_table.exit_scope();
                    Ok(indent_code(&inner))
                };
                let iterable_cpp = emit_expression_cpp(iterable, symbol_table, function_table, type_map)?;
                let mut for_code = String::new();
                if vars.len() == 1 {
                    for_code.push_str(&format!("    for (auto {} : {}) {{
",
 vars[0], iterable_cpp));
                } else {
                    for_code.push_str(&format!("    for (auto __eppx_tuple : {}) {{
",
 iterable_cpp));
                    for (i, var) in vars.iter().enumerate() {
                        for_code.push_str(&format!("        auto {} = std::get<{}>(__eppx_tuple);
",
 var, i));
                    }
                }
                for_code.push_str(&emit_block(body, declared_vars, symbol_table, function_table, type_map)?);
                for_code.push_str("    }
");
                cpp_out.push_str(&for_code);
            }
            AstNode::Statement(Statement::Return(expr)) => {
                if let Some(return_expr) = expr {
                    let return_value = emit_expression_cpp(return_expr, symbol_table, function_table, type_map)?;
                    cpp_out.push_str(&format!("    return {};
",
 return_value));
                } else {
                    cpp_out.push_str("    return;
");
                }
            }
            AstNode::Statement(Statement::ExpressionStatement(expr)) => {
                match &**expr {
                    Expression::Identifier(name) if name == "pass" => {
                        cpp_out.push_str("    ; // pass statement
");
                    }
                    _ => {
                        let expr_code = emit_expression_cpp(expr, symbol_table, function_table, type_map)?;
                        cpp_out.push_str(&format!("    {};
",
 expr_code));
                    }
                }
            }
            AstNode::Statement(Statement::Break) => {
                cpp_out.push_str("    break;
");
            }
            AstNode::Statement(Statement::Continue) => {
                cpp_out.push_str("    continue;
");
            }
            AstNode::Statement(Statement::Pass) => {
                cpp_out.push_str("    ; // pass statement
");
            }
            AstNode::Statement(Statement::TryExcept { try_body, excepts, else_body, finally_body }) => {
                let mut try_code = String::new();
                try_code.push_str("    try {\n");
                let mut block_symbol_table = symbol_table.fork();
                block_symbol_table.enter_scope();
                try_code.push_str(&indent_code(&generate_statement_list_cpp(try_body, declared_vars, &mut block_symbol_table, function_table, type_map)?));
                block_symbol_table.exit_scope();
                try_code.push_str("    }\n");
                for except in excepts {
                    try_code.push_str("    catch (");
                    if let Some(ref _exc_type) = except.exception_type {
                        try_code.push_str("std::exception& eppx_exc"); // Use different parameter name
                    } else {
                        try_code.push_str("std::exception& eppx_exc");
                    }
                    try_code.push_str(") {\n");
                    let mut except_symbol_table = symbol_table.fork();
                    except_symbol_table.enter_scope();
                    if let Some(ref name) = except.name {
                        try_code.push_str(&format!("        auto {} = eppx_exc.what();\n", name));
                    }
                    try_code.push_str(&indent_code(&generate_statement_list_cpp(&except.body, declared_vars, &mut except_symbol_table, function_table, type_map)?));
                    except_symbol_table.exit_scope();
                    try_code.push_str("    }\n");
                }
                if let Some(else_body_nodes) = else_body {
                    try_code.push_str("    // else block\n");
                    let mut else_symbol_table = symbol_table.fork();
                    else_symbol_table.enter_scope();
                    try_code.push_str(&indent_code(&generate_statement_list_cpp(&else_body_nodes, declared_vars, &mut else_symbol_table, function_table, type_map)?));
                    else_symbol_table.exit_scope();
                }
                if let Some(finally_body_nodes) = finally_body {
                    try_code.push_str("    // finally block\n");
                    let mut finally_symbol_table = symbol_table.fork();
                    finally_symbol_table.enter_scope();
                    try_code.push_str(&indent_code(&generate_statement_list_cpp(&finally_body_nodes, declared_vars, &mut finally_symbol_table, function_table, type_map)?));
                    finally_symbol_table.exit_scope();
                }
                cpp_out.push_str(&try_code);
            }
            AstNode::Statement(Statement::Raise(expr)) => {
                if let Some(expr) = expr {
                    let exc_cpp = emit_expression_cpp(&expr, symbol_table, function_table, type_map)?;
                    // Convert non-string expressions to strings for std::runtime_error
                    let string_exc = match expr {
                        Expression::StringLiteral(_) => exc_cpp,
                        _ => format!("std::to_string({})", exc_cpp),
                    };
                    cpp_out.push_str(&format!("    throw std::runtime_error({});\n", string_exc));
                } else {
                    cpp_out.push_str("    throw std::runtime_error(\"E++ exception\");\n");
                }
            }
            AstNode::Statement(Statement::With { items, body }) => {
                // Generate unique IDs for this with statement to avoid conflicts
                use std::collections::hash_map::DefaultHasher;
                use std::hash::{Hash, Hasher};
                let mut hasher = DefaultHasher::new();
                format!("{:?}", items).hash(&mut hasher);
                let unique_id = hasher.finish();
                
                // Generate C++ RAII-style with statement using context managers
                for (i, item) in items.iter().enumerate() {
                    let context_expr_cpp = emit_expression_cpp(&item.context_expr, symbol_table, function_table, type_map)?;
                    
                    // Create a context manager variable with unique name
                    let cm_var = format!("eppx_cm_{}_{}", unique_id, i);
                    cpp_out.push_str(&format!("    auto {} = eppx_with_file({});\n", cm_var, context_expr_cpp));
                    
                    // Call __enter__ method - use original variable name when specified
                    let enter_var = if let Some(var_name) = &item.optional_vars {
                        var_name.clone()  // Use the original variable name as specified by user
                    } else {
                        format!("eppx_ctx_{}_{}", unique_id, i)
                    };
                    
                    cpp_out.push_str(&format!("    auto {} = {}.__enter__();\n", enter_var, cm_var));
                    
                    // Add the context variable to the symbol table
                    symbol_table.add_variable(&enter_var, "auto");
                }
                
                // Generate the body in a try block to ensure __exit__ is called
                cpp_out.push_str("    try {\n");
                
                // Create new scope for the with body
                symbol_table.enter_scope();
                let body_cpp = generate_statement_list_cpp(body, declared_vars, symbol_table, function_table, type_map)?;
                cpp_out.push_str(&indent_code(&body_cpp));
                symbol_table.exit_scope();
                
                cpp_out.push_str("    }\n");
                
                // Generate __exit__ calls in reverse order (LIFO)
                for i in (0..items.len()).rev() {
                    let cm_var = format!("eppx_cm_{}_{}", unique_id, i);
                    cpp_out.push_str(&format!("    catch (...) {{\n"));
                    cpp_out.push_str(&format!("        {}.__exit__(\"exception\", \"exception occurred\", \"\");\n", cm_var));
                    cpp_out.push_str("        throw;\n");
                    cpp_out.push_str("    }\n");
                    cpp_out.push_str(&format!("    {}.__exit__(\"\", \"\", \"\");\n", cm_var));
                }
            }
            _ => {}
        }
    }
    Ok(cpp_out)
}

fn _generate_cpp_code_with_vars(
    ast_nodes: &[AstNode],
    is_toplevel: bool,
    declared_vars: &mut HashSet<String>,
    symbol_table: &mut SymbolTable,
    function_table: &mut FunctionTable, // Made mutable
    type_map: &mut TypeMap,
) -> Result<String, String> {
    if !is_toplevel {
        return generate_statement_list_cpp(ast_nodes, declared_vars, symbol_table, function_table, type_map);
    }

    let mut cpp_out = String::new();
    
    if is_toplevel {
        cpp_out.push_str("#include <iostream>
");
        cpp_out.push_str("#include <string>
");
        cpp_out.push_str("#include <vector>
");
        cpp_out.push_str("#include <algorithm>
");
        cpp_out.push_str("#include <cmath> // For std::pow
");
        cpp_out.push_str("#include <complex> // For std::complex
");
        cpp_out.push_str("#include <tuple>   // For std::tuple
");
        cpp_out.push_str("#include <map>     // For std::map
");
        cpp_out.push_str("#include <set>     // For std::set
");
        cpp_out.push_str("#include <unordered_set> // For std::unordered_set
");        cpp_out.push_str("#include <sstream> // For stringstream
");
        cpp_out.push_str("#include <bitset>  // For bitset
");
        cpp_out.push_str("#include <functional> // For std::hash
");
        cpp_out.push_str("#include \"../stdlib/builtins.hpp\" // For file I/O functions
");
        cpp_out.push_str("\n");
        
        // Stream operators for C++ container types to enable printing
        cpp_out.push_str("// Stream operators for container types\n");
        cpp_out.push_str("template<typename T>\n");
        cpp_out.push_str("std::ostream& operator<<(std::ostream& os, const std::vector<T>& vec) {\n");
        cpp_out.push_str("    os << \"[\";\n");
        cpp_out.push_str("    for (size_t i = 0; i < vec.size(); ++i) {\n");
        cpp_out.push_str("        if (i > 0) os << \", \";\n");
        cpp_out.push_str("        os << vec[i];\n");
        cpp_out.push_str("    }\n");
        cpp_out.push_str("    return os << \"]\";\n");
        cpp_out.push_str("}\n");
        cpp_out.push_str("\n");
        
        cpp_out.push_str("template<typename K, typename V>\n");
        cpp_out.push_str("std::ostream& operator<<(std::ostream& os, const std::map<K, V>& m) {\n");
        cpp_out.push_str("    os << \"{\";\n");
        cpp_out.push_str("    bool first = true;\n");
        cpp_out.push_str("    for (const auto& pair : m) {\n");
        cpp_out.push_str("        if (!first) os << \", \";\n");
        cpp_out.push_str("        os << pair.first << \": \" << pair.second;\n");
        cpp_out.push_str("        first = false;\n");
        cpp_out.push_str("    }\n");
        cpp_out.push_str("    return os << \"}\";\n");
        cpp_out.push_str("}\n");
        cpp_out.push_str("\n");
        
        cpp_out.push_str("template<typename T>\n");
        cpp_out.push_str("std::ostream& operator<<(std::ostream& os, const std::set<T>& s) {\n");
        cpp_out.push_str("    os << \"{\";\n");
        cpp_out.push_str("    bool first = true;\n");
        cpp_out.push_str("    for (const auto& item : s) {\n");
        cpp_out.push_str("        if (!first) os << \", \";\n");
        cpp_out.push_str("        os << item;\n");
        cpp_out.push_str("        first = false;\n");
        cpp_out.push_str("    }\n");
        cpp_out.push_str("    return os << \"}\";\n");
        cpp_out.push_str("}\n");
        cpp_out.push_str("\n");
        
        cpp_out.push_str("template<typename T>\n");
        cpp_out.push_str("std::ostream& operator<<(std::ostream& os, const std::unordered_set<T>& s) {\n");
        cpp_out.push_str("    os << \"frozenset({\";\n");
        cpp_out.push_str("    bool first = true;\n");
        cpp_out.push_str("    for (const auto& item : s) {\n");
        cpp_out.push_str("        if (!first) os << \", \";\n");
        cpp_out.push_str("        os << item;\n");
        cpp_out.push_str("        first = false;\n");
        cpp_out.push_str("    }\n");
        cpp_out.push_str("    return os << \"})\";\n");
        cpp_out.push_str("}\n");
        cpp_out.push_str("\n");
        
        cpp_out.push_str("template<typename T>\n");
        cpp_out.push_str("std::ostream& operator<<(std::ostream& os, const std::complex<T>& c) {\n");
        cpp_out.push_str("    return os << \"(\" << c.real() << (c.imag() >= 0 ? \"+\" : \"\") << c.imag() << \"j)\";\n");
        cpp_out.push_str("}\n");
        cpp_out.push_str("\n");
        
        // Tuple printing helper
        cpp_out.push_str("template<typename Tuple, size_t... Is>\n");
        cpp_out.push_str("void print_tuple_impl(std::ostream& os, const Tuple& t, std::index_sequence<Is...>) {\n");
        cpp_out.push_str("    ((os << (Is == 0 ? \"\" : \", \") << std::get<Is>(t)), ...);\n");
        cpp_out.push_str("}\n");
        cpp_out.push_str("\n");
        
        cpp_out.push_str("template<typename... Args>\n");
        cpp_out.push_str("std::ostream& operator<<(std::ostream& os, const std::tuple<Args...>& t) {\n");
        cpp_out.push_str("    os << \"(\";\n");
        cpp_out.push_str("    if constexpr (sizeof...(Args) > 0) {\n");
        cpp_out.push_str("        print_tuple_impl(os, t, std::index_sequence_for<Args...>{});\n");
        cpp_out.push_str("    }\n");
        cpp_out.push_str("    if constexpr (sizeof...(Args) == 1) {\n");
        cpp_out.push_str("        os << \",\";\n");
        cpp_out.push_str("    }\n");
        cpp_out.push_str("    return os << \")\";\n");
        cpp_out.push_str("}\n");
        cpp_out.push_str("\n");
        
        // Basic print functions - single argument versions
        cpp_out.push_str("void eppx_print(const std::string& s) { std::cout << s << std::endl; }
");
        cpp_out.push_str("void eppx_print(long long x) { std::cout << x << std::endl; }
");
        cpp_out.push_str("void eppx_print(double x) { std::cout << x << std::endl; }
");
        cpp_out.push_str("void eppx_print(bool b) { std::cout << (b ? \"true\" : \"false\") << std::endl; }
");
        cpp_out.push_str("void eppx_print(const std::complex<long long>& c) { std::cout << \"(\" << c.real() << (c.imag() >= 0 ? \"+\" : \"\") << c.imag() << \"j)\" << std::endl; }
");
        cpp_out.push_str("void eppx_print(const std::complex<double>& c) { std::cout << \"(\" << c.real() << (c.imag() >= 0 ? \"+\" : \"\") << c.imag() << \"j)\" << std::endl; }\n");
        cpp_out.push_str("void eppx_print(std::nullptr_t) { std::cout << \"None\" << std::endl; }\n");
        // Print functions with comprehensive type coverage to avoid ambiguity
        cpp_out.push_str("// Basic type printing functions\n");
        cpp_out.push_str("void eppx_print_single(bool b) { std::cout << (b ? \"true\" : \"false\"); }\n");
        cpp_out.push_str("void eppx_print_single(char c) { std::cout << c; }\n");
        cpp_out.push_str("void eppx_print_single(signed char x) { std::cout << static_cast<int>(x); }\n");
        cpp_out.push_str("void eppx_print_single(unsigned char x) { std::cout << static_cast<unsigned int>(x); }\n");
        cpp_out.push_str("void eppx_print_single(short x) { std::cout << x; }\n");
        cpp_out.push_str("void eppx_print_single(unsigned short x) { std::cout << x; }\n");
        cpp_out.push_str("void eppx_print_single(int x) { std::cout << x; }\n");
        cpp_out.push_str("void eppx_print_single(unsigned int x) { std::cout << x; }\n");
        cpp_out.push_str("void eppx_print_single(long x) { std::cout << x; }\n");
        cpp_out.push_str("void eppx_print_single(unsigned long x) { std::cout << x; }\n");
        cpp_out.push_str("void eppx_print_single(long long x) { std::cout << x; }\n");
        cpp_out.push_str("void eppx_print_single(unsigned long long x) { std::cout << x; }\n");
        cpp_out.push_str("void eppx_print_single(float x) { std::cout << x; }\n");
        cpp_out.push_str("void eppx_print_single(double x) { std::cout << x; }\n");
        cpp_out.push_str("void eppx_print_single(long double x) { std::cout << x; }\n");
        cpp_out.push_str("void eppx_print_single(const std::string& s) { std::cout << s; }\n");
        cpp_out.push_str("void eppx_print_single(const char* s) { std::cout << s; }\n");
        cpp_out.push_str("void eppx_print_single(std::nullptr_t) { std::cout << \"None\"; }\n");
        // Container printing functions
        cpp_out.push_str("template<typename T> void eppx_print_single(const std::vector<T>& vec) {\n");
        cpp_out.push_str("    std::cout << \"[\";\n");
        cpp_out.push_str("    for (size_t i = 0; i < vec.size(); ++i) {\n");
        cpp_out.push_str("        if (i > 0) std::cout << \", \";\n");
        cpp_out.push_str("        std::cout << vec[i];\n");
        cpp_out.push_str("    }\n");
        cpp_out.push_str("    std::cout << \"]\";\n");
        cpp_out.push_str("}\n");
        cpp_out.push_str("template<typename K, typename V> void eppx_print_single(const std::map<K, V>& m) {\n");
        cpp_out.push_str("    std::cout << \"{\";\n");
        cpp_out.push_str("    bool first = true;\n");
        cpp_out.push_str("    for (const auto& pair : m) {\n");
        cpp_out.push_str("        if (!first) std::cout << \", \";\n");
        cpp_out.push_str("        std::cout << pair.first << \": \" << pair.second;\n");
        cpp_out.push_str("        first = false;\n");
        cpp_out.push_str("    }\n");
        cpp_out.push_str("    std::cout << \"}\";\n");
        cpp_out.push_str("}\n");
        cpp_out.push_str("template<typename T> void eppx_print_single(const std::set<T>& s) {\n");
        cpp_out.push_str("    std::cout << \"{\";\n");
        cpp_out.push_str("    bool first = true;\n");
        cpp_out.push_str("    for (const auto& item : s) {\n");
        cpp_out.push_str("        if (!first) std::cout << \", \";\n");
        cpp_out.push_str("        std::cout << item;\n");
        cpp_out.push_str("        first = false;\n");
        cpp_out.push_str("    }\n");
        cpp_out.push_str("    std::cout << \"}\";\n");
        cpp_out.push_str("}\n");
        
        // Multi-argument variadic print function
        cpp_out.push_str("template<typename T, typename... Args> void eppx_print(T&& first, Args&&... args) {\n");
        cpp_out.push_str("    eppx_print_single(first);\n");
        cpp_out.push_str("    if constexpr (sizeof...(args) > 0) {\n");
        cpp_out.push_str("        std::cout << \" \";\n");
        cpp_out.push_str("        eppx_print(args...);\n");
        cpp_out.push_str("    } else {\n");
        cpp_out.push_str("        std::cout << std::endl;\n");
        cpp_out.push_str("    }\n");
        cpp_out.push_str("}\n");
        // Print functions for data structures (placeholders)
        cpp_out.push_str("template<typename T> void eppx_print(const std::vector<T>& vec) { std::cout << \"list object (size: \" << vec.size() << \")\" << std::endl; }\n");
        cpp_out.push_str("template<typename K, typename V> void eppx_print(const std::map<K, V>& m) { std::cout << \"dict object (size: \" << m.size() << \")\" << std::endl; }\n");
        cpp_out.push_str("template<typename T> void eppx_print(const std::set<T>& s) { std::cout << \"set object (size: \" << s.size() << \")\" << std::endl; }\n");
        cpp_out.push_str("template<typename T> void eppx_print(const std::unordered_set<T>& s) { std::cout << \"frozenset object (size: \" << s.size() << \")\" << std::endl; }\n");
        cpp_out.push_str("template <typename... Args> void eppx_print(const std::tuple<Args...>& t) { std::cout << \"tuple object (size: \" << sizeof...(Args) << \")\" << std::endl; }\n");
    }
    
    // First pass: emit all function definitions and class definitions at the top level
    // This helps with C++'s requirement for declaration before use.
    for node in ast_nodes {
        match node {
            AstNode::Statement(Statement::FunctionDef { name, params, body, decorators }) => {
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
                }
                let template_clause = if !template_params_gen.is_empty() {
                    format!("template<{}>
",
 template_params_gen.join(", "))
                } else {
                    "".to_string()
                };
                let param_list_cpp = call_params_gen.join(", ");

                // Populate FunctionTable
                let sig = FunctionSignature { param_types: param_types_for_signature, return_type: "auto".to_string() };
                function_table.add_function(name, sig);

                // Function body (symbol_table already has params in its current scope)
                let mut function_body_declared_vars = HashSet::new();
                let body_cpp = generate_statement_list_cpp(body, &mut function_body_declared_vars, symbol_table, function_table, type_map)?;
                symbol_table.exit_scope(); // End of function scope

                // Add decorator wrapper comments/code
                cpp_out.push_str(&decorator_wrappers);
                
                // Determine return type based on function body analysis
                let return_type = if has_explicit_return_type(body) {
                    analyze_return_type(body)
                } else {
                    "long long".to_string() // Default return type for E++ functions
                };
                
                cpp_out.push_str(&format!("{}{} {}({}) {{
",
 template_clause, return_type, name, param_list_cpp));
                cpp_out.push_str(&indent_code(&body_cpp));
                let has_return = body.iter().any(|node| matches!(node, AstNode::Statement(Statement::Return(_))));
                if !has_return {
                    // Default return for functions without explicit return
                    if return_type == "void" {
                        // No return needed for void functions
                    } else if return_type == "long long" {
                        cpp_out.push_str("    return 0LL; // Default return if none explicit\n");
                    } else if return_type == "double" {
                        cpp_out.push_str("    return 0.0; // Default return if none explicit\n");
                    } else if return_type == "std::string" {
                        cpp_out.push_str("    return \"\"; // Default return if none explicit\n");
                    } else if return_type == "bool" {
                        cpp_out.push_str("    return false; // Default return if none explicit\n");
                    } else {
                        // For auto return type, don't add default return - let compiler handle it
                        // This avoids conflicts when auto deduction is involved
                    }
                }
                cpp_out.push_str("}

");
            }
            AstNode::Statement(Statement::ClassDef { name, base, body }) => {
                if let Some(base_name) = base {
                    cpp_out.push_str(&format!("struct {} : public {} {{\n", name, base_name));
                } else {
                    cpp_out.push_str(&format!("struct {} {{\n", name));
                }                // First pass: collect attributes (assignments) and methods
                let mut constructor_params: Vec<String> = Vec::new();
                let mut constructor_body: String = String::new();
                let mut has_init = false;
                let mut instance_vars: HashSet<String> = HashSet::new();
                let mut static_vars: Vec<(String, String, String)> = Vec::new(); // (name, type, value)
                
                symbol_table.enter_scope(); // Class scope

                // Scan for instance variables in __init__ method
                for class_node in body {
                    if let AstNode::Statement(Statement::FunctionDef { name: method_name, body: method_body, .. }) = class_node {
                        if method_name == "__init__" {
                            for stmt in method_body {
                                if let AstNode::Statement(Statement::Assignment { target, .. }) = stmt {
                                    if let Expression::AttributeAccess { object, attr } = &**target {
                                        if let Expression::Identifier(obj_name) = &**object {
                                            if obj_name == "self" {
                                                instance_vars.insert(attr.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Emit instance variable declarations
                for var in &instance_vars {
                    cpp_out.push_str(&format!("    long long {};\n", var));
                }                for class_node in body {
                    match class_node {
                        AstNode::Statement(Statement::Assignment { target, operator: AssignmentOperator::Assign, value }) => {
                            // Collect static class variables
                            if let Expression::Identifier(member_name) = &**target {
                                if !instance_vars.contains(member_name) {
                                    let value_cpp = emit_expression_cpp(value, symbol_table, function_table, type_map)?;
                                    let type_str = infer_cpp_type_for_static_member(value);
                                    // Emit static declaration
                                    cpp_out.push_str(&format!("    static {} {};\n", type_str, member_name));
                                    // Store for later definition
                                    static_vars.push((member_name.clone(), type_str, value_cpp));
                                }
                            }
                        }
                        AstNode::Statement(Statement::FunctionDef { name: method_name, params, body: method_body, .. }) => {
                            symbol_table.enter_scope(); // Method scope
                            for p_name in params.iter().filter(|p| **p != "self") {
                                symbol_table.add_variable(p_name, "long long");
                            }

                            let mut method_declared_vars = HashSet::new();
                            let body_cpp = generate_statement_list_cpp(method_body, &mut method_declared_vars, symbol_table, function_table, type_map)?;                            if method_name == "__init__" {
                                has_init = true;
                                let params_cpp: Vec<String> = params.iter().filter(|p| **p != "self").map(|p| format!("long long {}", p)).collect();
                                constructor_params = params_cpp;
                                constructor_body = indent_code(&body_cpp);                            } else {
                                let return_type = infer_return_type_from_body(method_body, method_name, base);                                // --- Polymorphism: virtual/override ---
                                let is_override = base.is_some();
                                let is_private = method_name.starts_with('_') && method_name != "__str__" && method_name != "__init__";
                                let virtual_str = if !is_override { "virtual " } else { "" };
                                let override_str = if is_override { " override" } else { "" };
                                // For now, emit all methods as public except _underscore ones (but not __special__)
                                if is_private {
                                    cpp_out.push_str("private:\n");
                                }
                                let params_cpp = params.iter().filter(|p| **p != "self").map(|p| format!("long long {}", p)).collect::<Vec<_>>().join(", ");
                                cpp_out.push_str(&format!("    {}{} {}({}){} {{\n", virtual_str, return_type, method_name, params_cpp, override_str));
                                cpp_out.push_str(&indent_code(&body_cpp));
                                
                                let has_any_return = method_body.iter().any(|node| matches!(node, AstNode::Statement(Statement::Return(_))));
                                if !has_any_return {
                                    if return_type == "std::string" {
                                        cpp_out.push_str("        return \"\";\n");
                                    }
                                }
                                cpp_out.push_str("    }\n");
                                if is_private {
                                    cpp_out.push_str("public:\n");
                                }
                                // ---
                            }

                            symbol_table.exit_scope(); // Exit method scope
                        }
                        _ => { /* Ignore other statements for now */ }
                    }
                }

                if has_init {
                    cpp_out.push_str(&format!("    {}({}) {{
",
 name, constructor_params.join(", ")));
                    cpp_out.push_str(&constructor_body);
                    cpp_out.push_str("    }\n");
                }

                // Add a default constructor if no __init__ is defined
                if !has_init {
                    cpp_out.push_str(&format!("    {}() {{}}
",
 name));
                }

                // --- Encapsulation: public/private sections ---
                cpp_out.push_str("public:\n");
                // ---

                cpp_out.push_str("};\n");
                
                // Define static class variables outside the class
                for (var_name, var_type, var_value) in static_vars {
                    cpp_out.push_str(&format!("{} {}::{} = {};\n", var_type, name, var_name, var_value));
                }
                
                symbol_table.exit_scope(); // Exit class scope
            }
            _ => {} // Other statement types are handled in the second pass (for main's body)
        }
    }
    if is_toplevel {
        cpp_out.push_str("int main() {\n");
        let main_body_cpp = generate_statement_list_cpp(ast_nodes, declared_vars, symbol_table, function_table, type_map)?;
        cpp_out.push_str(&main_body_cpp);
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
            let escaped_s = s.replace("\\", "\\\\")
                             .replace("\"", "\\\"")
                             .replace("\n", "\\n")
                             .replace("\t", "\\t")
                             .replace("\r", "\\r");
            Ok(format!("std::string(\"{}\")", escaped_s))
        }
        Expression::IntegerLiteral(i) => Ok(format!("{}LL", i)), // Suffix LL for long long
        Expression::FloatLiteral(f) => Ok(format!("{}", f)), // Float literals
        Expression::NoneLiteral => Ok("nullptr".to_string()),
        Expression::BooleanLiteral(b) => Ok(format!("{}", b)),
        Expression::Identifier(name) => {
            // Handle builtin functions
            match name.as_str() {
                "sum" => Ok("eppx_sum".to_string()),
                "all" => Ok("eppx_all".to_string()),
                "any" => Ok("eppx_any".to_string()),
                "reversed" => Ok("eppx_reversed".to_string()),
                "list" => Ok("eppx_list".to_string()),
                "len" => Ok("eppx_len".to_string()),
                "zip" => Ok("eppx_zip".to_string()),
                "range" => Ok("eppx_range".to_string()),
                "max" => Ok("eppx_max".to_string()),
                "min" => Ok("eppx_min".to_string()),
                _ => Ok(name.clone()),
            }
        },        Expression::UnaryOperation { op, operand } => {
            let operand_cpp = emit_expression_cpp(operand, symbol_table, function_table, type_map)?;
            match op {
                UnaryOp::Not => Ok(format!("!({})", operand_cpp)), // Added parentheses for safety
                UnaryOp::BitNot => Ok(format!("~({})", operand_cpp)), // Implement BitNot
                UnaryOp::Negate => Ok(format!("-({})", operand_cpp)), // Unary minus
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
        Expression::AttributeAccess { object, attr } => {
            if let Expression::Identifier(name) = &**object {
                if name == "self" {
                    return Ok(format!("this->{}", attr));
                }
                // Class attribute access: ClassName.x
                // If the identifier is a class name, emit ClassName::x
                // (Assume class names are capitalized, variables are not)
                if name.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) {
                    return Ok(format!("{}::{}", name, attr));
                }
            }
            let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
            Ok(format!("{}.{}", object_cpp, attr))
        }
        Expression::Index { object, index } => {
            let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
            let index_cpp = emit_expression_cpp(index, symbol_table, function_table, type_map)?;
            Ok(format!("{}[{}]", object_cpp, index_cpp))
        }
        Expression::Call { callee, args } => {
            let mut args_cpp = Vec::new();
            for arg in args {
                args_cpp.push(emit_expression_cpp(arg, symbol_table, function_table, type_map)?);
            }            // Handle special built-in functions first
            if let Expression::Identifier(name) = &**callee {
                match name.as_str() {
                    // Print function
                    "print" => {
                        return Ok(format!("eppx_print({})", args_cpp.join(", ")));
                    }
                    
                    // Range function
                    "range" if args.len() == 1 => {
                        return Ok(format!("eppx_range({})", args_cpp[0]));
                    }
                    
                    // Mathematical functions
                    "abs" if args.len() == 1 => {
                        return Ok(format!("std::abs({})", args_cpp[0]));
                    }
                    "pow" if args.len() == 2 => {
                        return Ok(format!("std::pow({}, {})", args_cpp[0], args_cpp[1]));
                    }                    "max" if args.len() >= 2 => {
                        return Ok(format!("eppx_max({})", args_cpp.join(", ")));
                    }
                    "min" if args.len() >= 2 => {
                        return Ok(format!("eppx_min({})", args_cpp.join(", ")));
                    }
                    "round" if args.len() == 1 => {
                        return Ok(format!("std::round({})", args_cpp[0]));
                    }
                    
                    // Type conversions
                    "int" if args.len() == 1 => {
                        return Ok(format!("static_cast<long long>({})", args_cpp[0]));
                    }
                    "float" if args.len() == 1 => {
                        return Ok(format!("static_cast<double>({})", args_cpp[0]));
                    }
                    "bool" if args.len() == 1 => {
                        return Ok(format!("static_cast<bool>({})", args_cpp[0]));
                    }
                    "str" if args.len() == 1 => {
                        return Ok(format!("std::to_string({})", args_cpp[0]));
                    }
                    
                    // String functions
                    "len" if args.len() == 1 => {
                        return Ok(format!("{}.size()", args_cpp[0]));
                    }
                    "chr" if args.len() == 1 => {
                        return Ok(format!("std::string(1, static_cast<char>({}))", args_cpp[0]));
                    }
                    "ord" if args.len() == 1 => {
                        return Ok(format!("static_cast<int>({}[0])", args_cpp[0]));
                    }
                    
                    // Utility functions
                    "hex" if args.len() == 1 => {
                        return Ok(format!("eppx_hex({})", args_cpp[0]));
                    }
                    "bin" if args.len() == 1 => {
                        return Ok(format!("eppx_bin({})", args_cpp[0]));
                    }
                    "oct" if args.len() == 1 => {
                        return Ok(format!("eppx_oct({})", args_cpp[0]));
                    }
                    
                    // Collection functions
                    "sum" if args.len() == 1 => {
                        return Ok(format!("eppx_sum({})", args_cpp[0]));
                    }
                    "all" if args.len() == 1 => {
                        return Ok(format!("eppx_all({})", args_cpp[0]));
                    }
                    "any" if args.len() == 1 => {
                        return Ok(format!("eppx_any({})", args_cpp[0]));
                    }
                    "reversed" if args.len() == 1 => {
                        return Ok(format!("eppx_reversed({})", args_cpp[0]));
                    }
                    
                    // Collection constructors
                    "list" if args.len() == 0 => {
                        return Ok("std::vector<eppx_variant>{}".to_string());
                    }
                    "list" if args.len() == 1 => {
                        return Ok(format!("eppx_to_list({})", args_cpp[0]));
                    }
                    "tuple" if args.len() == 0 => {
                        return Ok("std::tuple<>{}".to_string());
                    }
                    "dict" if args.len() == 0 => {
                        return Ok("std::map<eppx_variant, eppx_variant>{}".to_string());
                    }
                    "set" if args.len() == 0 => {
                        return Ok("std::set<eppx_variant>{}".to_string());
                    }
                    "set" if args.len() == 1 => {
                        return Ok(format!("eppx_to_set({})", args_cpp[0]));
                    }
                    
                    // I/O functions
                    "input" if args.len() == 0 => {
                        return Ok("eppx_input()".to_string());
                    }
                    "input" if args.len() == 1 => {
                        return Ok(format!("eppx_input({})", args_cpp[0]));
                    }
                    
                    // File I/O functions
                    "open" if args.len() >= 1 && args.len() <= 7 => {
                        let mut open_args = vec![args_cpp[0].clone()];
                        if args.len() >= 2 { open_args.push(args_cpp[1].clone()); } else { open_args.push("\"r\"".to_string()); }
                        if args.len() >= 3 { open_args.push(args_cpp[2].clone()); } else { open_args.push("-1".to_string()); }
                        if args.len() >= 4 { open_args.push(args_cpp[3].clone()); } else { open_args.push("\"\"".to_string()); }
                        if args.len() >= 5 { open_args.push(args_cpp[4].clone()); } else { open_args.push("\"strict\"".to_string()); }
                        if args.len() >= 6 { open_args.push(args_cpp[5].clone()); } else { open_args.push("\"\"".to_string()); }
                        if args.len() >= 7 { open_args.push(args_cpp[6].clone()); } else { open_args.push("true".to_string()); }
                        return Ok(format!("eppx_open({})", open_args.join(", ")));
                    }
                    
                    // Type checking functions
                    "type" if args.len() == 1 => {
                        return Ok(format!("eppx_type({})", args_cpp[0]));
                    }
                    "isinstance" if args.len() == 2 => {
                        return Ok(format!("eppx_isinstance({}, {})", args_cpp[0], args_cpp[1]));
                    }
                    "callable" if args.len() == 1 => {
                        return Ok(format!("eppx_callable({})", args_cpp[0]));
                    }
                    
                    // Object introspection
                    "id" if args.len() == 1 => {
                        return Ok(format!("reinterpret_cast<uintptr_t>(&{})", args_cpp[0]));
                    }
                    "hasattr" if args.len() == 2 => {
                        return Ok(format!("eppx_hasattr({}, {})", args_cpp[0], args_cpp[1]));
                    }
                    "getattr" if args.len() == 2 => {
                        return Ok(format!("eppx_getattr({}, {})", args_cpp[0], args_cpp[1]));
                    }
                    "getattr" if args.len() == 3 => {
                        return Ok(format!("eppx_getattr({}, {}, {})", args_cpp[0], args_cpp[1], args_cpp[2]));
                    }
                    "setattr" if args.len() == 3 => {
                        return Ok(format!("eppx_setattr({}, {}, {})", args_cpp[0], args_cpp[1], args_cpp[2]));
                    }
                    "delattr" if args.len() == 2 => {
                        return Ok(format!("eppx_delattr({}, {})", args_cpp[0], args_cpp[1]));
                    }
                    
                    // Hash function
                    "hash" if args.len() == 1 => {
                        return Ok(format!("std::hash<eppx_variant>{{}}({})", args_cpp[0]));
                    }
                    
                    // Advanced functions that need custom implementation
                    "enumerate" if args.len() == 1 => {
                        return Ok(format!("eppx_enumerate({})", args_cpp[0]));
                    }
                    "zip" if args.len() >= 2 => {
                        return Ok(format!("eppx_zip({})", args_cpp.join(", ")));
                    }
                    "map" if args.len() >= 2 => {
                        return Ok(format!("eppx_map({}, {{{}}})", args_cpp[0], args_cpp[1..].join(", ")));
                    }
                    "filter" if args.len() == 2 => {
                        return Ok(format!("eppx_filter({}, {})", args_cpp[0], args_cpp[1]));
                    }
                    "sorted" if args.len() == 1 => {
                        return Ok(format!("eppx_sorted({})", args_cpp[0]));
                    }
                    
                    _ => {} // Fall through to generic function call
                }
            }
            
            // Handle file method calls
            if let Expression::AttributeAccess { object, attr } = &**callee {
                match attr.as_str() {
                    "read" => {
                        let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
                        if args.is_empty() {
                            return Ok(format!("{}->read()", object_cpp));
                        } else {
                            return Ok(format!("{}->read({})", object_cpp, args_cpp[0]));
                        }
                    }
                    "readline" => {
                        let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
                        if args.is_empty() {
                            return Ok(format!("{}->readline()", object_cpp));
                        } else {
                            return Ok(format!("{}->readline({})", object_cpp, args_cpp[0]));
                        }
                    }
                    "readlines" => {
                        let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
                        return Ok(format!("{}->readlines()", object_cpp));
                    }
                    "write" => {
                        let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
                        if !args.is_empty() {
                            return Ok(format!("{}->write({})", object_cpp, args_cpp[0]));
                        }
                    }
                    "writelines" => {
                        let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
                        if !args.is_empty() {
                            return Ok(format!("{}->writelines({})", object_cpp, args_cpp[0]));
                        }
                    }
                    "close" => {
                        let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
                        return Ok(format!("{}->close()", object_cpp));
                    }
                    "flush" => {
                        let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
                        return Ok(format!("{}->flush()", object_cpp));
                    }
                    "seek" => {
                        let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
                        if !args.is_empty() {
                            return Ok(format!("{}->seek({})", object_cpp, args_cpp[0]));
                        }
                    }
                    "tell" => {
                        let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
                        return Ok(format!("{}->tell()", object_cpp));
                    }
                    "upper" => {
                        let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
                        return Ok(format!("eppx_upper({})", object_cpp));
                    }
                    "lower" => {
                        let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
                        return Ok(format!("eppx_lower({})", object_cpp));
                    }
                    "size" | "length" => {
                        let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
                        return Ok(format!("eppx_len({})", object_cpp));
                    }
                    _ => {}
                }
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
            
            Ok(format!("([&]({}) {{ return {}; }})", params_cpp, body_cpp))
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
        // Comprehensions
        Expression::ListComprehension { element, comprehension } => {
            emit_comprehension_cpp(element, comprehension, "list", symbol_table, function_table, type_map)
        }
        Expression::DictComprehension { key, value, comprehension } => {
            emit_dict_comprehension_cpp(key, value, comprehension, symbol_table, function_table, type_map)
        }
        Expression::SetComprehension { element, comprehension } => {
            emit_comprehension_cpp(element, comprehension, "set", symbol_table, function_table, type_map)
        }
        Expression::GeneratorExpression { element, comprehension } => {
            emit_comprehension_cpp(element, comprehension, "generator", symbol_table, function_table, type_map)
        }
        Expression::Index { object, index } => {
            let object_cpp = emit_expression_cpp(object, symbol_table, function_table, type_map)?;
            let index_cpp = emit_expression_cpp(index, symbol_table, function_table, type_map)?;
            Ok(format!("{}[{}]", object_cpp, index_cpp))
        }
        // The duplicate Lambda match arm that was here (around lines 450-458) is removed.
        // The primary Expression::Lambda handler is earlier and correctly uses symbol_table.
    }
}

// Helper functions for function analysis
fn has_explicit_return_type(body: &[AstNode]) -> bool {
    // Simple heuristic: if the function has any return statements with expressions
    body.iter().any(|node| {
        if let AstNode::Statement(Statement::Return(Some(_))) = node {
            true
        } else {
            false
        }
    })
}

fn analyze_return_type(body: &[AstNode]) -> String {
    // Analyze function body to determine return type
    for node in body {
        if let AstNode::Statement(Statement::Return(Some(expr))) = node {
            return match &**expr {
                Expression::IntegerLiteral(_) => "long long".to_string(),
                Expression::FloatLiteral(_) => "double".to_string(),
                Expression::StringLiteral(_) => "std::string".to_string(),
                Expression::BooleanLiteral(_) => "bool".to_string(),
                Expression::NoneLiteral => "std::nullptr_t".to_string(),
                _ => "auto".to_string(),
            };
        }
    }
    "void".to_string()
}

fn generate_decorator_wrappers(_decorators: &[crate::ast::Decorator]) -> Result<String, String> {
    // Placeholder implementation for decorator support
    // In a full implementation, this would generate wrapper functions
    Ok(String::new())
}

fn infer_cpp_type_for_static_member(value: &Expression) -> String {
    match value {
        Expression::IntegerLiteral(_) => "long long".to_string(),
        Expression::FloatLiteral(_) => "double".to_string(),
        Expression::StringLiteral(_) => "std::string".to_string(),
        Expression::BooleanLiteral(_) => "bool".to_string(),
        Expression::NoneLiteral => "std::nullptr_t".to_string(),
        _ => "auto".to_string(),
    }
}

fn infer_return_type_from_body(body: &[AstNode], method_name: &str, _base: &Option<String>) -> String {
    // Analyze method body to determine return type
    for node in body {
        if let AstNode::Statement(Statement::Return(Some(expr))) = node {
            return match &**expr {
                Expression::IntegerLiteral(_) => "long long".to_string(),
                Expression::FloatLiteral(_) => "double".to_string(),
                Expression::StringLiteral(_) => "std::string".to_string(),
                Expression::BooleanLiteral(_) => "bool".to_string(),
                Expression::NoneLiteral => "std::nullptr_t".to_string(),
                // For complex expressions, use a generic type instead of auto
                // since virtual functions can't have auto return type
                Expression::BinaryOperation { .. } => {
                    // Most binary operations in E++ result in strings or numbers
                    // For string concatenation, assume string; for arithmetic, assume long long
                    if method_name == "__str__" || is_likely_string_expression(expr) {
                        "std::string".to_string()
                    } else {
                        "long long".to_string()
                    }
                },
                Expression::Call { .. } => {
                    // Function calls - assume they return the expected type for this method
                    if method_name == "__str__" {
                        "std::string".to_string()
                    } else {
                        "long long".to_string()
                    }
                },
                _ => {
                    // For other expressions, use a sensible default
                    if method_name == "__str__" {
                        "std::string".to_string()
                    } else {
                        "long long".to_string()
                    }
                },
            };
        }
    }
    
    // Special method names
    if method_name == "__str__" {
        "std::string".to_string()
    } else {
        "void".to_string()
    }
}

// Helper function to guess if an expression is likely to result in a string
fn is_likely_string_expression(expr: &Expression) -> bool {
    match expr {
        Expression::StringLiteral(_) => true,
        Expression::BinaryOperation { left, op, right } => {
            match op {
                crate::ast::BinOp::Add => {
                    // String concatenation if either operand is a string
                    is_likely_string_expression(left) || is_likely_string_expression(right)
                },
                _ => false,
            }
        },
        Expression::Call { callee, .. } => {
            // str() function calls result in strings
            matches!(callee.as_ref(), Expression::Identifier(name) if name == "str")
        },
        _ => false,
    }
}

// Helper function to emit comprehensions (list, set, generator)
fn emit_comprehension_cpp(
    element: &Expression,
    comprehension: &Comprehension,
    comp_type: &str,
    symbol_table: &mut SymbolTable,
    function_table: &FunctionTable,
    type_map: &mut TypeMap,
) -> Result<String, String> {
    // Create a new scope for the comprehension variable(s)
    symbol_table.enter_scope();
    
    // Add all target variables to symbol table
    for target in &comprehension.target {
        symbol_table.add_variable(target, "auto");
    }
    
    let iter_cpp = emit_expression_cpp(&comprehension.iter, symbol_table, function_table, type_map)?;
    let element_cpp = emit_expression_cpp(element, symbol_table, function_table, type_map)?;
    
    // Generate if conditions
    let mut condition_cpp = String::new();
    if !comprehension.ifs.is_empty() {
        let conditions: Result<Vec<String>, String> = comprehension.ifs.iter()
            .map(|cond| emit_expression_cpp(cond, symbol_table, function_table, type_map))
            .collect();
        let conditions = conditions?;
        condition_cpp = format!("if ({}) ", conditions.join(" && "));
    }
    
    symbol_table.exit_scope();
    
    // Generate target pattern for destructuring
    let target_pattern = if comprehension.target.len() == 1 {
        format!("auto {}", comprehension.target[0])
    } else {
        // For pairs (like from zip), use structured binding
        format!("auto [{}]", comprehension.target.join(", "))
    };
    
    match comp_type {
        "list" => {
            // Check if this is a nested comprehension by examining the element expression
            let is_nested_comprehension = matches!(element, 
                Expression::ListComprehension { .. } | 
                Expression::DictComprehension { .. } | 
                Expression::SetComprehension { .. } | 
                Expression::GeneratorExpression { .. }
            );
            
            if is_nested_comprehension {
                // For nested comprehensions, use a different approach to avoid decltype issues
                // We'll just use auto and let the first push_back determine the type
                Ok(format!(
                    "([&]() {{ \
                        std::vector<decltype({})> temp_vec; \
                        for ({} : {}) {{ \
                            auto temp_elem = {}; \
                            {}{{ temp_vec.push_back(temp_elem); }} \
                        }} \
                        return temp_vec; \
                    }})()",
                    // For the decltype, we'll create a dummy instance of the inner comprehension type
                    "std::vector<long long>{}", target_pattern, iter_cpp, element_cpp, condition_cpp
                ))
            } else {
                Ok(format!(
                    "([&]() {{ \
                        std::vector<long long> temp_vec; \
                        for ({} : {}) {{ \
                            auto temp_elem = {}; \
                            {}{{ temp_vec.push_back(static_cast<long long>(temp_elem)); }} \
                        }} \
                        return temp_vec; \
                    }})()",
                    target_pattern, iter_cpp, element_cpp, condition_cpp
                ))
            }
        }
        "set" => {
            Ok(format!(
                "([&]() {{ \
                    std::set<long long> temp_set; \
                    for ({} : {}) {{ \
                        auto temp_elem = {}; \
                        {}{{ temp_set.insert(static_cast<long long>(temp_elem)); }} \
                    }} \
                    return temp_set; \
                }})()",
                target_pattern, iter_cpp, element_cpp, condition_cpp
            ))
        }
        "generator" => {
            // For generator expressions, we'll use a lambda that returns a vector for simplicity
            // In a full implementation, this would be a proper iterator/generator
            Ok(format!(
                "([&]() {{ \
                    std::vector<long long> temp_vec; \
                    for ({} : {}) {{ \
                        auto temp_elem = {}; \
                        {}{{ temp_vec.push_back(static_cast<long long>(temp_elem)); }} \
                    }} \
                    return temp_vec; \
                }})()",
                target_pattern, iter_cpp, element_cpp, condition_cpp
            ))
        }
        _ => Err(format!("Unsupported comprehension type: {}", comp_type))
    }
}

// Helper function to emit dictionary comprehensions
fn emit_dict_comprehension_cpp(
    key: &Expression,
    value: &Expression,
    comprehension: &Comprehension,
    symbol_table: &mut SymbolTable,
    function_table: &FunctionTable,
    type_map: &mut TypeMap,
) -> Result<String, String> {
    // Create a new scope for the comprehension variable(s)
    symbol_table.enter_scope();
    
    // Add all target variables to symbol table
    for target in &comprehension.target {
        symbol_table.add_variable(target, "auto");
    }
    
    let iter_cpp = emit_expression_cpp(&comprehension.iter, symbol_table, function_table, type_map)?;
    let key_cpp = emit_expression_cpp(key, symbol_table, function_table, type_map)?;
    let value_cpp = emit_expression_cpp(value, symbol_table, function_table, type_map)?;
    
    // Generate if conditions
    let mut condition_cpp = String::new();
    if !comprehension.ifs.is_empty() {
        let conditions: Result<Vec<String>, String> = comprehension.ifs.iter()
            .map(|cond| emit_expression_cpp(cond, symbol_table, function_table, type_map))
            .collect();
        let conditions = conditions?;
        condition_cpp = format!("if ({}) ", conditions.join(" && "));
    }
    
    symbol_table.exit_scope();
    
    // Generate target pattern for destructuring
    let target_pattern = if comprehension.target.len() == 1 {
        format!("auto {}", comprehension.target[0])
    } else {
        // For pairs (like from zip), use structured binding
        format!("auto [{}]", comprehension.target.join(", "))
    };
    
    // Generate the comprehension using stringstream for both key and value conversion
    Ok(format!(
        "([&]() {{ \
            std::map<std::string, std::string> temp_map; \
            for ({} : {}) {{ \
                auto temp_key = {}; \
                auto temp_value = {}; \
                {}{{ \
                    std::ostringstream key_ss; \
                    key_ss << temp_key; \
                    std::ostringstream value_ss; \
                    value_ss << temp_value; \
                    temp_map[key_ss.str()] = value_ss.str(); \
                }} \
            }} \
            return temp_map; \
        }})()",
        target_pattern, iter_cpp, key_cpp, value_cpp, condition_cpp
    ))
}

