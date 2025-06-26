use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom, BufRead, BufReader};
use std::path::Path;

use super::FileObject;

// File I/O operations for EPlusPlus runtime
#[allow(dead_code)]
pub struct FileIO;

#[allow(dead_code)]
impl FileIO {
    /// Open a file and return a file handle
    pub fn open_file(
        filepath: &str, 
        mode: &str, 
        _buffering: Option<i32>,
        encoding: Option<&str>,
        _errors: Option<&str>,
        _newline: Option<&str>,
        _closefd: Option<bool>,
        _opener: Option<&str>
    ) -> Result<FileObject, String> {
        let path = Path::new(filepath);
        
        // Validate mode
        let mode = mode.to_lowercase();
        if !Self::is_valid_mode(&mode) {
            return Err(format!("Invalid mode: '{}'", mode));
        }

        // Create file object
        let mut file_obj = FileObject::new(
            filepath.to_string(),
            mode.clone(),
            encoding.map(|s| s.to_string()),
        );

        // Check if file exists for read modes
        if mode.contains('r') && !path.exists() {
            return Err(format!("No such file or directory: '{}'", filepath));
        }

        file_obj.is_open = true;
        Ok(file_obj)
    }

    /// Read from a file
    pub fn read_file(file_obj: &mut FileObject, size: Option<usize>) -> Result<String, String> {
        if !file_obj.is_open {
            return Err("I/O operation on closed file".to_string());
        }

        if !file_obj.mode.contains('r') {
            return Err("File not open for reading".to_string());
        }

        let mut file = File::open(&file_obj.filepath)
            .map_err(|e| format!("Failed to open file: {}", e))?;

        // Seek to current position
        file.seek(SeekFrom::Start(file_obj.position as u64))
            .map_err(|e| format!("Failed to seek: {}", e))?;

        let mut contents = String::new();
        
        match size {
            Some(n) => {
                let mut buffer = vec![0; n];
                let bytes_read = file.read(&mut buffer)
                    .map_err(|e| format!("Failed to read file: {}", e))?;
                buffer.truncate(bytes_read);
                contents = String::from_utf8_lossy(&buffer).to_string();
                file_obj.position += bytes_read;
            }
            None => {
                file.read_to_string(&mut contents)
                    .map_err(|e| format!("Failed to read file: {}", e))?;
                file_obj.position = contents.len();
            }
        }

        Ok(contents)
    }

    /// Read a single line from a file
    pub fn readline(file_obj: &mut FileObject, size: Option<usize>) -> Result<String, String> {
        if !file_obj.is_open {
            return Err("I/O operation on closed file".to_string());
        }

        if !file_obj.mode.contains('r') {
            return Err("File not open for reading".to_string());
        }

        let file = File::open(&file_obj.filepath)
            .map_err(|e| format!("Failed to open file: {}", e))?;

        let mut reader = BufReader::new(file);
        
        // Skip to current position
        let mut skipped = 0;
        while skipped < file_obj.position {
            let mut buf = vec![0; std::cmp::min(8192, file_obj.position - skipped)];
            let bytes_read = reader.read(&mut buf)
                .map_err(|e| format!("Failed to read file: {}", e))?;
            if bytes_read == 0 {
                break;
            }
            skipped += bytes_read;
        }

        let mut line = String::new();
        let bytes_read = reader.read_line(&mut line)
            .map_err(|e| format!("Failed to read line: {}", e))?;

        // Apply size limit if specified
        if let Some(limit) = size {
            if line.len() > limit {
                line.truncate(limit);
            }
        }

        file_obj.position += bytes_read;
        Ok(line)
    }

    /// Read all lines from a file
    pub fn readlines(file_obj: &mut FileObject, hint: Option<usize>) -> Result<Vec<String>, String> {
        if !file_obj.is_open {
            return Err("I/O operation on closed file".to_string());
        }

        if !file_obj.mode.contains('r') {
            return Err("File not open for reading".to_string());
        }

        let file = File::open(&file_obj.filepath)
            .map_err(|e| format!("Failed to open file: {}", e))?;

        let reader = BufReader::new(file);
        let mut lines = Vec::new();
        let mut total_size = 0;

        for line_result in reader.lines() {
            let line = line_result.map_err(|e| format!("Failed to read line: {}", e))?;
            let line_with_newline = format!("{}\n", line);
            
            if let Some(limit) = hint {
                if total_size + line_with_newline.len() > limit {
                    break;
                }
            }
            
            total_size += line_with_newline.len();
            lines.push(line_with_newline);
        }

        file_obj.position += total_size;
        Ok(lines)
    }

    /// Write to a file
    pub fn write_file(file_obj: &mut FileObject, data: &str) -> Result<usize, String> {
        if !file_obj.is_open {
            return Err("I/O operation on closed file".to_string());
        }

        if !file_obj.mode.contains('w') && !file_obj.mode.contains('a') && !file_obj.mode.contains('+') {
            return Err("File not open for writing".to_string());
        }

        let mut file = if file_obj.mode.contains('a') {
            OpenOptions::new()
                .append(true)
                .open(&file_obj.filepath)
                .map_err(|e| format!("Failed to open file for append: {}", e))?
        } else {
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(file_obj.mode.contains('w') && !file_obj.mode.contains('+'))
                .open(&file_obj.filepath)
                .map_err(|e| format!("Failed to open file for write: {}", e))?
        };

        let bytes_written = file.write(data.as_bytes())
            .map_err(|e| format!("Failed to write to file: {}", e))?;

        file.flush()
            .map_err(|e| format!("Failed to flush file: {}", e))?;

        file_obj.position += bytes_written;
        Ok(bytes_written)
    }

    /// Write multiple lines to a file
    pub fn writelines(file_obj: &mut FileObject, lines: &[String]) -> Result<(), String> {
        for line in lines {
            Self::write_file(file_obj, line)?;
        }
        Ok(())
    }

    /// Close a file
    pub fn close_file(file_obj: &mut FileObject) -> Result<(), String> {
        if !file_obj.is_open {
            return Ok(()); // Already closed
        }
        
        file_obj.is_open = false;
        file_obj.position = 0;
        Ok(())
    }

    /// Flush file buffers
    pub fn flush_file(file_obj: &FileObject) -> Result<(), String> {
        if !file_obj.is_open {
            return Err("I/O operation on closed file".to_string());
        }

        // For read-only files, flush is a no-op
        if file_obj.mode.contains('r') && !file_obj.mode.contains('+') {
            return Ok(());
        }

        // Note: In a real implementation, we'd maintain file handles
        // For now, we'll assume the OS handles flushing
        Ok(())
    }

    /// Seek to a position in the file
    pub fn seek_file(file_obj: &mut FileObject, offset: i64, whence: i32) -> Result<usize, String> {
        if !file_obj.is_open {
            return Err("I/O operation on closed file".to_string());
        }

        let file = File::open(&file_obj.filepath)
            .map_err(|e| format!("Failed to open file: {}", e))?;

        let seek_from = match whence {
            0 => SeekFrom::Start(offset as u64), // SEEK_SET
            1 => SeekFrom::Current(offset),      // SEEK_CUR  
            2 => SeekFrom::End(offset),          // SEEK_END
            _ => return Err(format!("Invalid whence value: {}", whence)),
        };

        let mut file_handle = file;
        let new_pos = file_handle.seek(seek_from)
            .map_err(|e| format!("Failed to seek: {}", e))?;

        file_obj.position = new_pos as usize;
        Ok(file_obj.position)
    }

    /// Tell current position in file
    pub fn tell_file(file_obj: &FileObject) -> Result<usize, String> {
        if !file_obj.is_open {
            return Err("I/O operation on closed file".to_string());
        }
        Ok(file_obj.position)
    }

    /// Check if file is readable
    pub fn is_readable(file_obj: &FileObject) -> bool {
        file_obj.is_open && file_obj.mode.contains('r')
    }

    /// Check if file is writable  
    pub fn is_writable(file_obj: &FileObject) -> bool {
        file_obj.is_open && (file_obj.mode.contains('w') || file_obj.mode.contains('a') || file_obj.mode.contains('+'))
    }

    /// Check if file is seekable
    pub fn is_seekable(file_obj: &FileObject) -> bool {
        file_obj.is_open && !file_obj.mode.contains("pipe") // Simplified check
    }

    /// Validate file mode string
    fn is_valid_mode(mode: &str) -> bool {
        let valid_chars = ['r', 'w', 'a', 'x', 'b', 't', '+'];
        let mut has_main_mode = false;
        let mut has_binary = false;
        let mut has_text = false;

        for ch in mode.chars() {
            if !valid_chars.contains(&ch) {
                return false;
            }
            
            match ch {
                'r' | 'w' | 'a' | 'x' => {
                    if has_main_mode {
                        return false; // Only one main mode allowed
                    }
                    has_main_mode = true;
                }
                'b' => {
                    if has_binary || has_text {
                        return false; // Can't have both binary and text
                    }
                    has_binary = true;
                }
                't' => {
                    if has_binary || has_text {
                        return false; // Can't have both binary and text  
                    }
                    has_text = true;
                }
                '+' => {} // Can always have plus
                _ => return false,
            }
        }

        has_main_mode
    }
}

// Context manager for with statements
#[allow(dead_code)]
pub struct FileContextManager {
    file_obj: FileObject,
}

#[allow(dead_code)]
impl FileContextManager {
    pub fn new(file_obj: FileObject) -> Self {
        Self { file_obj }
    }

    pub fn enter(&mut self) -> &mut FileObject {
        &mut self.file_obj
    }

    pub fn exit(&mut self, _exc_type: Option<&str>, _exc_val: Option<&str>, _exc_tb: Option<&str>) -> Result<bool, String> {
        FileIO::close_file(&mut self.file_obj)?;
        Ok(false) // Don't suppress exceptions
    }
}
