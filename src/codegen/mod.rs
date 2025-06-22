// Codegen module placeholder
use crate::ast::{AstNode, Expression, Statement, BinOp, UnaryOp, AssignmentOperator, Decorator};
use std::collections::{HashMap, HashSet};

// Placeholder for SymbolTable, FunctionTable, and TypeMap

#[derive(Debug, Clone)]
pub struct VariableInfo {
    pub type_name: String,
    pub is_const: bool,
}

pub struct SymbolTable {
    scopes: Vec<HashMap<String, VariableInfo>>,
    current_scope_index: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self { scopes: vec![HashMap::new()], current_scope_index: 0 }
    }
    pub fn enter_scope(&mut self) { self.scopes.push(HashMap::new()); }
    pub fn exit_scope(&mut self) { if self.scopes.len() > 1 { self.scopes.pop(); } }
    pub fn add_variable(&mut self, name: &str, var_type: &str) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.to_string(), VariableInfo { type_name: var_type.to_string(), is_const: false });
        }
    }
    pub fn get_variable(&self, name: &str) -> Option<&VariableInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(var_info) = scope.get(name) { return Some(var_info); }
        }
        None
    }
    pub fn fork(&self) -> SymbolTable { SymbolTable::new() } // Simplified fork
}

pub struct FunctionSignature {
    #[allow(dead_code)] pub param_types: Vec<String>,
    pub return_type: String,
}

pub struct FunctionTable { functions: HashMap<String, FunctionSignature> }
impl FunctionTable {
    pub fn new() -> Self { FunctionTable { functions: HashMap::new() } }
    #[allow(dead_code)] pub fn add_function(&mut self, name: &str, signature: FunctionSignature) { self.functions.insert(name.to_string(), signature); }
    pub fn get_function(&self, name: &str) -> Option<&FunctionSignature> { self.functions.get(name) }
}

pub struct TypeMap { #[allow(dead_code)] types: HashMap<String, String> }
impl TypeMap { pub fn new() -> Self { TypeMap { types: HashMap::new() } } }

// Helper to compile classinfo argument for isinstance/issubclass
fn compile_classinfo_arg(
    classinfo_expr: &Expression,
    symbol_table: &mut SymbolTable,
    function_table: &FunctionTable,
    type_map: &mut TypeMap
) -> Result<String, String> {
    match classinfo_expr {
        Expression::Identifier(name) => {
            // Could check if 'name' is a known class type from symbol_table if it stores class names.
            // For now, assume any identifier here is intended as a type name string.
            Ok(format!("eppx_variant(std::string(\"{}\"))", name))
        }
        Expression::StringLiteral(name) => {
            let escaped_name = name.replace("\\", "\\\\").replace("\"", "\\\"");
            Ok(format!("eppx_variant(std::string(\"{}\"))", escaped_name))
        }
        Expression::TupleLiteral(elements) => {
            let mut element_strings_cpp = Vec::new();
            for elem_expr in elements {
                match elem_expr {
                    Expression::Identifier(id_name) => {
                        element_strings_cpp.push(format!("eppx_variant(std::string(\"{}\"))", id_name));
                    }
                    Expression::StringLiteral(str_val) => {
                         let escaped_val = str_val.replace("\\", "\\\\").replace("\"", "\\\"");
                         element_strings_cpp.push(format!("eppx_variant(std::string(\"{}\"))", escaped_val));
                    }
                    _ => return Err(format!("Invalid type name in classinfo tuple: expected Identifier or String, got {:?}", elem_expr)),
                }
            }
            Ok(format!("eppx_variant(eppx_list_variant_t{{{}}})", element_strings_cpp.join(", ")))
        }
        _ => {
            // Fallback: compile as a normal expression. This assumes it will evaluate to an eppx_variant
            // containing either a std::string or an eppx_list_variant_t of strings at runtime.
            // This path is more flexible but relies on runtime correctness of the argument.
            emit_expression_cpp(classinfo_expr, symbol_table, function_table, type_map)
        }
    }
}


pub fn generate_cpp_code_with_toplevel(ast_nodes: &[AstNode], is_toplevel: bool) -> Result<String, String> {
    let mut declared_vars = HashSet::new();
    let mut symbol_table = SymbolTable::new();
    let mut function_table = FunctionTable::new();
    let mut type_map = TypeMap::new();
    let mut static_initializers = String::new();

    let generated_code = _generate_cpp_code_with_vars(ast_nodes, is_toplevel, &mut declared_vars, &mut symbol_table, &mut function_table, &mut type_map, &mut static_initializers)?;

    if is_toplevel && !static_initializers.is_empty() {
        let main_start = "int main() {\n";
        let registration_fn_decl = "void eppx_register_all_static_members();\n\n";
        let registration_fn_def = format!("\nvoid eppx_register_all_static_members() {{\n{}}}\n\n", static_initializers);
        let registration_call = "    eppx_register_all_static_members();\n";

        let mut final_cpp = String::new();
        final_cpp.push_str("#include \"builtins.hpp\"\n\n");
        final_cpp.push_str(registration_fn_decl);

        if let Some(main_pos) = generated_code.find(main_start) {
            final_cpp.push_str(&generated_code[..main_pos]);
            final_cpp.push_str(&registration_fn_def);

            let mut main_content_with_call = String::from(main_start);
            main_content_with_call.push_str(registration_call);
            main_content_with_call.push_str(&generated_code[main_pos + main_start.len()..]);
            final_cpp.push_str(&main_content_with_call);
        } else {
            final_cpp.push_str(&generated_code);
            final_cpp.push_str(&registration_fn_def);
            if !generated_code.contains(main_start) {
                final_cpp.push_str("\nint main() {\n");
                final_cpp.push_str(registration_call);
                final_cpp.push_str("    return 0;\n}\n");
            }
        }
        Ok(final_cpp)
    } else {
        if is_toplevel && !generated_code.starts_with("#include \"builtins.hpp\"") {
             Ok(format!("#include \"builtins.hpp\"\n\n{}", generated_code))
        } else { Ok(generated_code) }
    }
}

fn _generate_cpp_code_with_vars(
    ast_nodes: &[AstNode],
    is_toplevel: bool,
    declared_vars: &mut HashSet<String>,
    symbol_table: &mut SymbolTable,
    function_table: &mut FunctionTable,
    type_map: &mut TypeMap,
    static_initializers: &mut String,
) -> Result<String, String> {
    let mut cpp_out = String::new();

    if is_toplevel {
        for node in ast_nodes {
            if let AstNode::Statement(statement) = node {
                match &**statement {
                    Statement::FunctionDef { name, params, body, decorators } => {
                        let decorator_wrappers = generate_decorator_wrappers(decorators)?;
                        cpp_out.push_str(&decorator_wrappers);
                        let mut call_params_gen = Vec::new();
                        symbol_table.enter_scope();
                        for p_name in params.iter() {
                            call_params_gen.push(format!("eppx_variant {}", p_name));
                            symbol_table.add_variable(p_name, "eppx_variant");
                        }
                        let param_list_cpp = call_params_gen.join(", ");
                        function_table.add_function(name, FunctionSignature { param_types: params.iter().map(|_| "eppx_variant".to_string()).collect(), return_type: "eppx_variant".to_string() });

                        let mut function_body_declared_vars = HashSet::new();
                        let body_cpp = _generate_cpp_code_with_vars(body, false, &mut function_body_declared_vars, symbol_table, function_table, type_map, static_initializers)?;
                        symbol_table.exit_scope();

                        cpp_out.push_str(&format!("eppx_variant {}({}) {{
",
 name, param_list_cpp));
                        cpp_out.push_str(&body_cpp);
                        if !body.iter().any(|n| matches!(n, AstNode::Statement(s) if matches!(**s, Statement::Return(_)))) {
                            cpp_out.push_str("    return eppx_variant(std::nullptr_t{}); // Default return None
");
                        }
                        cpp_out.push_str("}

");
                    }
                    Statement::ClassDef { name: class_name_str, base_class, body } => {
                        cpp_out.push_str(&format!("struct {} ", class_name_str));
                        let mut base_class_name_for_map: Option<String> = None;
                        if let Some(base_expr_box) = base_class {
                            if let Expression::Identifier(base_name) = &**base_expr_box {
                                cpp_out.push_str(&format!(": public {} ", base_name));
                                base_class_name_for_map = Some(base_name.clone());
                            } else { return Err("Complex base class expressions not supported yet".to_string()); }
                        }
                        cpp_out.push_str("{
");
                        let mut member_access_initializers_for_this_class = String::new();

                        for class_node in body {
                            if let AstNode::Statement(class_statement) = class_node {
                                match &**class_statement {
                                    Statement::Assignment { name: member_name, operator, value } => {
                                        if operator == &AssignmentOperator::Assign {
                                            cpp_out.push_str(&format!("    static inline eppx_variant {};
", member_name));
                                            let value_cpp = emit_expression_cpp(value, symbol_table, function_table, type_map)?;
                                            member_access_initializers_for_this_class.push_str(&format!(r#"    get_global_class_static_accessors()["{}"].get_attr_s_funcs["{}"] = []() -> eppx_variant {{ return {}::{}; }};
"#, class_name_str, member_name, class_name_str, member_name));
                                            member_access_initializers_for_this_class.push_str(&format!(r#"    get_global_class_static_accessors()["{}"].set_attr_s_funcs["{}"] = [](eppx_variant val) {{ {}::{} = val; }};
"#, class_name_str, member_name, class_name_str, member_name));
                                            member_access_initializers_for_this_class.push_str(&format!(r#"    get_global_class_static_accessors()["{}"].has_attr_s_funcs["{}"] = []() -> bool {{ return true; }};
"#, class_name_str, member_name));
                                            static_initializers.push_str(&format!("    {}::{} = {};
", class_name_str, member_name, value_cpp));
                                        } else { cpp_out.push_str(&format!("    // Skipped compound assignment for static member {}
", member_name)); }
                                    }
                                    Statement::FunctionDef { name: method_name, params, body: method_body, decorators } => {
                                        cpp_out.push_str(&generate_decorator_wrappers(decorators)?);
                                        let method_params_cpp: Vec<String> = params.iter().map(|p_name| format!("eppx_variant {}", p_name)).collect();
                                        let method_param_list_cpp = method_params_cpp.join(", ");

                                        let mut method_body_symbol_table = symbol_table.fork(); method_body_symbol_table.enter_scope();
                                        for p_name in params { method_body_symbol_table.add_variable(p_name, "eppx_variant"); }
                                        let mut method_body_declared_vars = HashSet::new();
                                        let method_body_cpp = _generate_cpp_code_with_vars(method_body, false, &mut method_body_declared_vars, &mut method_body_symbol_table, function_table, type_map, static_initializers)?;
                                        method_body_symbol_table.exit_scope();

                                        cpp_out.push_str(&format!("    static eppx_variant {}({}) {{
",
method_name, method_param_list_cpp));
                                        cpp_out.push_str(&method_body_cpp);
                                        if !method_body.iter().any(|n| if let AstNode::Statement(s) = n { matches!(**s, Statement::Return(_)) } else { false }) { cpp_out.push_str("        return eppx_variant(std::nullptr_t{});
"); }
                                        cpp_out.push_str("    }

");
                                        member_access_initializers_for_this_class.push_str(&format!(r#"    get_global_class_static_accessors()["{}"].has_attr_s_funcs["{}"] = []() -> bool {{ return true; }};
"#, class_name_str, method_name));
                                    }
                                    Statement::Pass => { cpp_out.push_str("    // pass
"); }
                                    _ => { cpp_out.push_str(&format!("    // Skipped in class: {:?}
", class_node)); }
                                }
                            } else {
                                cpp_out.push_str(&format!("    // Skipped in class: {:?}
", class_node));
                            }
                        }
                        cpp_out.push_str("};

");
                        if !member_access_initializers_for_this_class.is_empty() {
                            static_initializers.push_str(&member_access_initializers_for_this_class);
                        }
                        if let Some(base_name) = base_class_name_for_map {
                            static_initializers.push_str(&format!(r#"    get_g_inheritance_map()["{}"] = "{}";
"#, class_name_str, base_name));
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    if is_toplevel && !cpp_out.contains("int main() {") { cpp_out.push_str("int main() {
"); }

    for node in ast_nodes {
        if is_toplevel {
            if let AstNode::Statement(s) = node {
                if matches!(**s, Statement::FunctionDef { .. } | Statement::ClassDef { .. }) {
                    continue;
                }
            }
        }

        if let AstNode::Statement(statement) = node {
            match &**statement {
                Statement::Print(expr) => {
                    let expr_code = emit_expression_cpp(expr, symbol_table, function_table, type_map)?;
                    cpp_out.push_str(&format!("    eppx_print({});
", expr_code));
                }
                Statement::Assignment { name, operator, value } => {
                    let value_cpp = emit_expression_cpp(value, symbol_table, function_table, type_map)?;
                    if !declared_vars.contains(name) && symbol_table.get_variable(name).is_none() {
                        cpp_out.push_str(&format!("    eppx_variant {} = {};
", name, value_cpp));
                        declared_vars.insert(name.clone());
                        symbol_table.add_variable(name, "eppx_variant");
                    } else {
                        match operator {
                            AssignmentOperator::Assign => cpp_out.push_str(&format!("    {} = {};
",
 name, value_cpp)),
                            _ => {
                                let op_str = match operator {
                                    AssignmentOperator::AddAssign => "add",
                                    AssignmentOperator::SubAssign => "sub",
                                    AssignmentOperator::MulAssign => "mul",
                                    AssignmentOperator::DivAssign => "div",
                                    AssignmentOperator::FloorDivAssign => "floordiv",
                                    AssignmentOperator::ModAssign => "mod",
                                    AssignmentOperator::PowAssign => "pow",
                                    AssignmentOperator::BitAndAssign => "bitand",
                                    AssignmentOperator::BitOrAssign => "bitor",
                                    AssignmentOperator::BitXorAssign => "bitxor",
                                    AssignmentOperator::LShiftAssign => "lshift",
                                    AssignmentOperator::RShiftAssign => "rshift",
                                    _ => return Err(format!("Unsupported assignment operator: {:?}", operator))
                                };
                                let op_assign_str = format!("{}_assign", op_str);
                                cpp_out.push_str(&format!("    {} = eppx_binary_op(\"{}\", {}, {});\n", name, op_assign_str, name, value_cpp));
                            }
                        }
                    }
                }
                Statement::If { condition, then_body, elifs, else_body } => {
                    let emit_block_content = |stmts: &Vec<AstNode>, decl_vars: &mut HashSet<String>, sym_table: &mut SymbolTable, func_table: &mut FunctionTable, tm: &mut TypeMap, static_init: &mut String| -> Result<String, String> {
                        let mut block_sym_table = sym_table.fork(); block_sym_table.enter_scope();
                        let mut block_cpp = String::new();
                        let mut block_local_declared_vars = decl_vars.clone();
                        for stmt_node in stmts {
                            let inner_cpp = _generate_cpp_code_with_vars(&[stmt_node.clone()], false, &mut block_local_declared_vars, &mut block_sym_table, func_table, tm, static_init)?;
                            for line in inner_cpp.lines() { block_cpp.push_str(&format!("    {}
", line)); }
                        }
                        block_sym_table.exit_scope(); Ok(block_cpp)
                    };
                    let cond_cpp = emit_expression_cpp(condition, symbol_table, function_table, type_map)?;
                    cpp_out.push_str(&format!("    if (eppx_is_truthy({})) {{
", cond_cpp));
                    cpp_out.push_str(&emit_block_content(then_body, declared_vars, symbol_table, function_table, type_map, static_initializers)?);
                    cpp_out.push_str("    }");
                    for (elif_cond, elif_body) in elifs {
                        let elif_cond_cpp = emit_expression_cpp(elif_cond, symbol_table, function_table, type_map)?;
                        cpp_out.push_str(&format!(" else if (eppx_is_truthy({})) {{
", elif_cond_cpp));
                        cpp_out.push_str(&emit_block_content(elif_body, declared_vars, symbol_table, function_table, type_map, static_initializers)?);
                        cpp_out.push_str("    }");
                    }
                    if let Some(else_b) = else_body {
                        cpp_out.push_str(" else {
");
                        cpp_out.push_str(&emit_block_content(else_b, declared_vars, symbol_table, function_table, type_map, static_initializers)?);
                        cpp_out.push_str("    }");
                    }
                    cpp_out.push_str("
");
                }
                Statement::While { condition, body } => {
                    let emit_block_content = |stmts: &Vec<AstNode>, decl_vars: &mut HashSet<String>, sym_table: &mut SymbolTable, func_table: &mut FunctionTable, tm: &mut TypeMap, static_init: &mut String| -> Result<String, String> {
                        let mut block_sym_table = sym_table.fork(); block_sym_table.enter_scope();
                        let mut block_cpp = String::new();
                        let mut block_local_declared_vars = decl_vars.clone();
                        for stmt_node in stmts {
                            let inner_cpp = _generate_cpp_code_with_vars(&[stmt_node.clone()], false, &mut block_local_declared_vars, &mut block_sym_table, func_table, tm, static_init)?;
                            for line in inner_cpp.lines() { block_cpp.push_str(&format!("    {}
", line));}
                        }
                        block_sym_table.exit_scope(); Ok(block_cpp)
                    };
                    let cond_cpp = emit_expression_cpp(condition, symbol_table, function_table, type_map)?;
                    cpp_out.push_str(&format!("    while (eppx_is_truthy({})) {{
", cond_cpp));
                    cpp_out.push_str(&emit_block_content(body, declared_vars, symbol_table, function_table, type_map, static_initializers)?);
                    cpp_out.push_str("    }
");
                }
                Statement::For { vars, iterable, body } => {
                    let emit_block_content = |stmts: &Vec<AstNode>, decl_vars: &mut HashSet<String>, sym_table: &mut SymbolTable, func_table: &mut FunctionTable, tm: &mut TypeMap, static_init: &mut String, loop_vars_names: &Vec<String>| -> Result<String, String> {
                        let mut block_sym_table = sym_table.fork(); block_sym_table.enter_scope();
                        for var_name in loop_vars_names { block_sym_table.add_variable(var_name, "eppx_variant"); }
                        let mut block_cpp = String::new();
                        let mut block_local_declared_vars = decl_vars.clone();
                        for var_name in loop_vars_names { block_local_declared_vars.insert(var_name.clone()); }

                        for stmt_node in stmts {
                            let inner_cpp = _generate_cpp_code_with_vars(&[stmt_node.clone()], false, &mut block_local_declared_vars, &mut block_sym_table, func_table, tm, static_init)?;
                            for line in inner_cpp.lines() { block_cpp.push_str(&format!("        {}
", line));}
                        }
                        block_sym_table.exit_scope(); Ok(block_cpp)
                    };
                    let iterable_cpp = emit_expression_cpp(iterable, symbol_table, function_table, type_map)?;
                    if vars.len() == 1 {
                        cpp_out.push_str(&format!("    for (eppx_variant {} : eppx_make_iterable({})) {{
",
vars[0], iterable_cpp));
                    } else {
                        return Err("Tuple unpacking in for loop not fully supported in this codegen version".to_string());
                    }
                    cpp_out.push_str(&emit_block_content(body, declared_vars, symbol_table, function_table, type_map, static_initializers, &vars)?);
                    cpp_out.push_str("    }
");
                }
                Statement::Return(expr) => {
                    if let Some(return_expr) = expr {
                        let return_value = emit_expression_cpp(return_expr, symbol_table, function_table, type_map)?;
                        cpp_out.push_str(&format!("    return {};
", return_value));
                    } else {
                        cpp_out.push_str("    return eppx_variant(std::nullptr_t{});
");
                    }
                }
                Statement::ExpressionStatement(expr) => {
                    let expr_code = emit_expression_cpp(expr, symbol_table, function_table, type_map)?;
                    cpp_out.push_str(&format!("    (void)({});
", expr_code));
                }
                Statement::Break => { cpp_out.push_str("    break;
"); }
                Statement::Continue => { cpp_out.push_str("    continue;
"); }
                Statement::Pass => { cpp_out.push_str("    /* pass */;
"); }
                _ => { cpp_out.push_str(&format!("    // Skipped statement in second pass: {:?}
", node));}
            }
        } else {
            cpp_out.push_str(&format!("    // Skipped statement in second pass: {:?}
", node));
        }
    }

    if is_toplevel && !cpp_out.ends_with("}
") && cpp_out.contains("int main() {") {
        cpp_out.push_str("    return 0;
}
");
    }
    Ok(cpp_out)
}

pub fn generate_cpp_code(ast_nodes: &[AstNode]) -> Result<String, String> {
    generate_cpp_code_with_toplevel(ast_nodes, true)
}

// Helper to get the string name from an Expression::Identifier
fn get_identifier_name(expr: &Expression) -> Option<String> {
    if let Expression::Identifier(name) = expr {
        Some(name.clone())
    } else {
        None
    }
}

pub fn emit_expression_cpp(
    expr: &Expression,
    symbol_table: &mut SymbolTable,
    function_table: &FunctionTable,
    type_map: &mut TypeMap,
) -> Result<String, String> {
    match expr {
        Expression::StringLiteral(s) => { let esc = s.replace("\\", "\\\\").replace("\"", "\\\""); Ok(format!("eppx_variant(std::string(\"{}\"))", esc)) },
        Expression::IntegerLiteral(i) => Ok(format!("eppx_variant({}LL)", i)),
        Expression::FloatLiteral(f) => Ok(format!("eppx_variant({})", f)),
        Expression::NoneLiteral => Ok("eppx_variant(std::nullptr_t{})".to_string()),
        Expression::BooleanLiteral(b) => Ok(format!("eppx_variant({})", b)),
        Expression::Identifier(name) => {
            if function_table.get_function(name).is_some() {
                 let func_sig = function_table.get_function(name).unwrap();
                 let params_count = func_sig.param_types.len();
                 let mut arg_placeholders = Vec::new();
                 let mut direct_args = Vec::new();
                 for i in 0..params_count {
                     arg_placeholders.push(format!("const eppx_variant& arg{}", i));
                     direct_args.push(format!("arg{}",i));
                 }
                 return Ok(format!("eppx_variant(eppx_callable_variant_t([=]({}) -> eppx_variant {{ return {}({}); }}))", arg_placeholders.join(", "), name, direct_args.join(", ")));
            }
            Ok(name.clone())
        },
        Expression::ListLiteral(elements) => {
            let mut elements_cpp = Vec::new();
            for el in elements { elements_cpp.push(emit_expression_cpp(el, symbol_table, function_table, type_map)?); }
            Ok(format!("eppx_variant(eppx_list_variant_t{{{}}})", elements_cpp.join(", ")))
        }
        Expression::TupleLiteral(elements) => {
            let mut elements_cpp = Vec::new();
            for el in elements { elements_cpp.push(emit_expression_cpp(el, symbol_table, function_table, type_map)?); }
            Ok(format!("eppx_variant(eppx_tuple{{eppx_list_variant_t{{{}}}}})", elements_cpp.join(", ")))
        }
        Expression::DictLiteral(entries) => {
            let mut entries_cpp = Vec::new();
            for (k, v) in entries { // k,v are Box<Expression>
                let k_expr_cpp = emit_expression_cpp(k, symbol_table, function_table, type_map)?;
                // Key must be a string, so we'll try to std::get<std::string> from it.
                // This assumes the key expression evaluates to an eppx_variant holding a string.
                let k_str_extraction = format!("std::get<std::string>({})", k_expr_cpp);
                let v_cpp = emit_expression_cpp(v, symbol_table, function_table, type_map)?;
                entries_cpp.push(format!("{{{}(), {}}}", k_str_extraction, v_cpp));
            }
            Ok(format!("eppx_variant(eppx_dict_variant_t{{{}}})", entries_cpp.join(", ")))
        }
        Expression::Call { callee, args } => {
            let mut positional_args_cpp = Vec::new();
            let mut keyword_args_for_dict = Vec::new(); // Stores (String name, String compiled_value_cpp)
            let mut is_dict_kwargs_call = false;

            if let Expression::Identifier(callee_name) = &**callee {
                if callee_name == "dict" {
                    let mut has_positional = false;
                    let mut has_keywords = false;
                    for arg_expr_node in args { // args is Vec<Expression>
                        // This is where the AST limitation hits. We can't easily distinguish
                        // `dict(a=1)` from `dict(some_var)` if some_var happens to be an assignment expression.
                        // The parser should create `Argument::Keyword` for true keyword args.
                        // For now, assume if `dict` is called, and if an arg is BinOp Assign, it's a kwarg.
                        // This is a heuristic and needs proper AST support (Call with Vec<Argument>).
                        if let Expression::BinaryOperation {left, op: BinOp::Assign, right} = arg_expr_node {
                             if let Expression::Identifier(kw_name) = &**left {
                                has_keywords = true;
                                let kw_val_cpp = emit_expression_cpp(right, symbol_table, function_table, type_map)?;
                                keyword_args_for_dict.push(format!("{{\"{}\", {}}}", kw_name, kw_val_cpp));
                             } else { // Not a simple ident=val kwarg, treat as positional
                                positional_args_cpp.push(emit_expression_cpp(arg_expr_node, symbol_table, function_table, type_map)?);
                                has_positional = true;
                             }
                        } else {
                            positional_args_cpp.push(emit_expression_cpp(arg_expr_node, symbol_table, function_table, type_map)?);
                            has_positional = true;
                        }
                    }
                    if has_keywords && has_positional {
                        return Err("dict() with mixed positional and keyword arguments is not yet fully supported in this simplified codegen pass. Use dict(iterable) or dict(key=value, ...).".to_string());
                    }
                    is_dict_kwargs_call = has_keywords && !has_positional;
                }
            }

            if !is_dict_kwargs_call { // Standard argument processing for non-dict-kwargs calls
                positional_args_cpp.clear(); // Clear if filled by dict kwarg attempt
                for arg_expr_node in args {
                    positional_args_cpp.push(emit_expression_cpp(arg_expr_node, symbol_table, function_table, type_map)?);
                }
            }

            if let Expression::Identifier(name) = &**callee {
                 match name.as_str() {
                    "hasattr" if args.len() == 2 => {
                        let obj_expr = &args[0]; // This is an Expression node
                        let attr_name_expr = &args[1]; // This is an Expression node
                        // We need to ensure obj_expr is an Identifier for static hasattr
                        if let Expression::Identifier(obj_name_str) = obj_expr {
                             if let Expression::StringLiteral(attr_name) = attr_name_expr {
                                return Ok(format!("eppx_variant(eppx_has_static_attr(\"{}\", \"{}\"))", obj_name_str, attr_name.replace("\"", "\\\"")));
                             } else { return Err("hasattr's second argument must be a string literal for attribute name".to_string());}
                        } else { // If obj_expr is not an identifier, assume it's an instance (not supported yet for instance hasattr)
                            let obj_cpp = emit_expression_cpp(obj_expr, symbol_table, function_table, type_map)?;
                            let attr_name_cpp = emit_expression_cpp(attr_name_expr, symbol_table, function_table, type_map)?; // attr_name could be variable
                            return Ok(format!("eppx_variant(eppx_hasattr_instance({}, {}))", obj_cpp, attr_name_cpp)); // Placeholder for instance version
                        }
                    }
                    "getattr" => { /* Similar logic to hasattr, distinguishing static vs instance */ return Ok(String::from("eppx_variant()"))} // Placeholder
                    "setattr" => { /* ... */ return Ok(String::from("eppx_variant(nullptr)"))}
                    "delattr" => { /* ... */ return Ok(String::from("eppx_variant(nullptr)"))}
                    "dir" => { /* ... */ return Ok(String::from("eppx_variant()"))}
                    "vars" => { /* ... */ return Ok(String::from("eppx_variant()"))}
                    _ => {}
                }
            }
            let callee_cpp = emit_expression_cpp(callee, symbol_table, function_table, type_map)?;
            let args_cpp: Result<Vec<String>, String> = args.iter().map(|arg| emit_expression_cpp(arg, symbol_table, function_table, type_map)).collect();
            Ok(format!("{}({})", callee_cpp, args_cpp?.join(", ")))
        }
        Expression::Lambda { params, body } => {
            let mut lambda_symbol_table = symbol_table.fork();
            lambda_symbol_table.enter_scope();
            let params_cpp = params.iter().map(|p_name| { lambda_symbol_table.add_variable(p_name, "eppx_variant"); format!("const eppx_variant& {}", p_name) }).collect::<Vec<String>>().join(", ");
            let body_cpp = emit_expression_cpp(body, &mut lambda_symbol_table, function_table, type_map)?;
            lambda_symbol_table.exit_scope();
            Ok(format!("eppx_variant(eppx_callable_variant_t([=]({}) -> eppx_variant {{ return {}; }}))", params_cpp, body_cpp))
        }
         Expression::BinaryOperation { left, op, right } => {
            let l = emit_expression_cpp(left, symbol_table, function_table, type_map)?;
            let r = emit_expression_cpp(right, symbol_table, function_table, type_map)?;
            let op_str = match op {
                BinOp::Add => "add",
                BinOp::Sub => "sub",
                BinOp::Mul => "mul",
                BinOp::Div => "div",
                BinOp::FloorDiv => "floordiv",
                BinOp::Mod => "mod",
                BinOp::Pow => "pow",
                BinOp::Eq => "eq",
                BinOp::NotEq => "ne",
                BinOp::Lt => "lt",
                BinOp::Gt => "gt",
                BinOp::LtEq => "le",
                BinOp::GtEq => "ge",
                BinOp::And => "and",
                BinOp::Or => "or",
                BinOp::BitAnd => "bitand",
                BinOp::BitOr => "bitor",
                BinOp::BitXor => "bitxor",
                BinOp::LShift => "lshift",
                BinOp::RShift => "rshift",
                BinOp::Is => "is",
                BinOp::IsNot => "is_not",
                BinOp::In => "in",
                BinOp::NotIn => "not_in",
                BinOp::Assign => return Err("Assign should be handled in Statement::Assignment, not as a binary operation".to_string()),
            };
            Ok(format!("eppx_binary_op(\"{}\", {}, {})", op_str, l, r))
        }
        Expression::UnaryOperation { op, operand } => {
            let o = emit_expression_cpp(operand, symbol_table, function_table, type_map)?;
            let op_str = match op { UnaryOp::Not => "not", UnaryOp::Negate => "-", UnaryOp::BitNot => "~" };
            Ok(format!("eppx_unary_op(\"{}\", {})", op_str, o))
        }
        _ => Err(String::from("Unsupported expression type for C++ codegen"))
    }
}

fn generate_decorator_wrappers(decorators: &[Decorator]) -> Result<String, String> {
    let mut wrapper_code = String::new();
    for decorator in decorators {
        match decorator {
            Decorator::Simple(name) => wrapper_code.push_str(&format!("// @{}\n", name)),
            Decorator::WithArgs(name, _args) => wrapper_code.push_str(&format!("// @{}(...)\n", name)),
        }
    }
    Ok(wrapper_code)
}
