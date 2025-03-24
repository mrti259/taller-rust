use crate::errors::*;
use crate::operator::*;
use std::{fs::File, io::Write};

pub struct Interpreter {
    stack: Vec<i16>,
    operators: Vec<Operator>,
}

impl Interpreter {
    pub fn with_stack_size(size: usize) -> Self {
        Self {
            stack: Vec::with_capacity(size),
            operators: vec![Operator::new("+".into())],
        }
    }

    pub fn run_code(&mut self, code: &str) -> Result<(), BorthError> {
        let tokens = code.split(" ");
        for token in tokens {
            if let Some(operator) = self.detect_operation(token) {
                operator.operate(&mut self.stack);
            } else if let Ok(value) = token.parse::<i16>() {
                self.stack.push(value);
            } else {
                return Err(BorthError::RuntimeError);
            }
        }
        Ok(())
    }

    pub fn export_stack_to(&self, path_to_file: &str) -> Result<(), BorthError> {
        if let Ok(mut file) = File::create(path_to_file) {
            for i in 0..self.stack.len() {
                let mut value = self.stack[i].to_string();
                if i < self.stack.len() - 1 {
                    value.push(' ');
                }
                if file.write(value.as_bytes()).is_err() {
                    return Err(BorthError::ExportError);
                }
            }
        }
        Ok(())
    }

    fn detect_operation(&self, token: &str) -> Option<Operator> {
        for operator in &self.operators {
            if operator.accept(token) {
                return Some(operator.to_owned());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs::{File, remove_file},
        io::Read,
        path::Path,
    };

    fn create_interpreter() -> Interpreter {
        Interpreter::with_stack_size(10)
    }

    fn run_code(code: &str) -> Vec<i16> {
        let mut interpreter = create_interpreter();
        interpreter.run_code(code).expect("Valid code.");
        interpreter.stack
    }

    fn delete_file_if_exists(path_to_file: &str) {
        if Path::new(path_to_file).exists() {
            let _ = remove_file(path_to_file);
        }
    }

    #[test]
    fn push_to_stack() {
        let stack = run_code("1 2 3");
        assert_eq!(stack, [1, 2, 3]);
    }

    #[test]
    fn perform_operation() {
        let stack = run_code("1 2 3 +");
        assert_eq!(stack, [6]);
    }

    #[test]
    fn export_stack() {
        let path_to_file = "stack.fth";
        delete_file_if_exists(path_to_file);

        let mut interpreter = create_interpreter();
        assert!(interpreter.run_code("1 2 3 + 1 2").is_ok());
        assert_eq!(interpreter.stack, [6, 1, 2]);

        assert!(interpreter.export_stack_to(path_to_file).is_ok());

        let mut file = File::open(path_to_file).expect("File exists.");
        let mut content = String::new();
        assert!(file.read_to_string(&mut content).is_ok());
        let stack_read = content
            .split(" ")
            .filter_map(|element| element.parse::<i16>().ok())
            .collect::<Vec<i16>>();
        assert_eq!(stack_read, interpreter.stack);
    }

    #[test]
    fn stop_at_error() {
        let mut forth = create_interpreter();
        assert!(forth.run_code("1 2 3 UNKNOWN + 4 5 6 + ").is_err());
        assert_eq!(forth.stack, [1, 2, 3]);
    }
}
