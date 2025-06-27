use std::collections::HashMap;
use crate::ast::AstNode;

pub mod file;

// Runtime value types for file objects
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum RuntimeValue {
    None,
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    File(FileObject),
    Generator(GeneratorObject),
    List(Vec<RuntimeValue>),
    // Add other types as needed
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct FileObject {
    pub filepath: String,
    pub mode: String,
    pub encoding: Option<String>,
    pub is_open: bool,
    pub position: usize,
}

#[allow(dead_code)]
impl FileObject {
    pub fn new(filepath: String, mode: String, encoding: Option<String>) -> Self {
        Self {
            filepath,
            mode,
            encoding,
            is_open: false,
            position: 0,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct GeneratorObject {
    pub function_body: Vec<AstNode>,
    pub local_variables: HashMap<String, RuntimeValue>,
    pub execution_state: GeneratorState,
    pub current_position: usize,
}

#[allow(dead_code)]
impl GeneratorObject {
    pub fn new(function_body: Vec<AstNode>) -> Self {
        Self {
            function_body,
            local_variables: HashMap::new(),
            execution_state: GeneratorState::Created,
            current_position: 0,
        }
    }
    
    pub fn next_value(&mut self) -> Result<Option<RuntimeValue>, String> {
        // This would contain the logic to execute until the next yield
        // For now, return None to indicate the generator is exhausted
        Ok(None)
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum GeneratorState {
    Created,
    Running,
    Suspended,
    Completed,
}

// Runtime context for managing open files
#[allow(dead_code)]
pub struct RuntimeContext {
    pub open_files: HashMap<String, FileObject>,
    pub generators: Vec<GeneratorObject>,
}

#[allow(dead_code)]
impl RuntimeContext {
    pub fn new() -> Self {
        Self {
            open_files: HashMap::new(),
            generators: Vec::new(),
        }
    }
}
