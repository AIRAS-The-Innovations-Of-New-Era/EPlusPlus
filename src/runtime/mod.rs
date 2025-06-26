use std::collections::HashMap;

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

// Runtime context for managing open files
#[allow(dead_code)]
pub struct RuntimeContext {
    pub open_files: HashMap<String, FileObject>,
}

#[allow(dead_code)]
impl RuntimeContext {
    pub fn new() -> Self {
        Self {
            open_files: HashMap::new(),
        }
    }
}
