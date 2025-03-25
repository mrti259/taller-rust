use crate::errors::*;
use std::{fs::File, io::Write};

type StackElementType = i32;
type Stack = Vec<StackElementType>;

pub struct Interpreter {
    stack: Stack,
}

impl Interpreter {
    pub fn with_stack_size(size: usize) -> Self {
        Self {
            stack: Vec::with_capacity(size),
        }
    }

    pub fn export_stack_to(&self, path_to_file: &str) -> BorthResult<()> {
        if let Ok(mut file) = File::create(path_to_file) {
            for i in 0..self.stack.len() {
                let mut value = self.stack[i].to_string();
                if i < self.stack.len() - 1 {
                    value.push(' ');
                }
                if file.write(value.as_bytes()).is_err() {
                    return Err(BorthError::CanNotWriteFile);
                }
            }
            Ok(())
        } else {
            Err(BorthError::CanNotWriteFile)
        }
    }

    pub fn run_code(&mut self, code: &str) -> BorthResult<()> {
        let code = code.replace(" ", "\n");
        for token in code.lines() {
            match token.trim().to_uppercase().as_str() {
                "" => continue,
                "+" => self.sum(),
                "-" => self.sub(),
                "*" => self.prod(),
                "/" => self.div(),
                "DUP" => self.dup(),
                "DROP" => self.drop(),
                "SWAP" => self.swap(),
                "OVER" => self.over(),
                "ROT" => self.rot(),
                "=" => self.eq(),
                "<" => self.lt(),
                ">" => self.gt(),
                "AND" => self.and(),
                "OR" => self.or(),
                "NOT" => self.not(),
                "." => todo!(),
                "EMIT" => todo!(),
                "CR" => todo!(),
                ".\"" => todo!(),
                "\"" => todo!(),
                "IF" => todo!(),
                "ELSE" => todo!(),
                "THEN" => todo!(),
                _ => self.push(token),
            }?;
        }
        Ok(())
    }

    fn push(&mut self, token: &str) -> BorthResult<()> {
        match token.parse::<StackElementType>() {
            Ok(element) => {
                self.stack.push(element);
                Ok(())
            }
            _ => Err(BorthError::UnknownWord),
        }
    }

    fn pop(&mut self) -> BorthResult<StackElementType> {
        self.stack.pop().ok_or(BorthError::StackUnderflow)
    }

    fn sum(&mut self) -> BorthResult<()> {
        let mut sum = self.pop()?;
        while let Ok(element) = self.pop() {
            sum += element;
        }
        self.stack.push(sum);
        Ok(())
    }

    fn sub(&mut self) -> BorthResult<()> {
        let mut sub = self.pop()?;
        while let Ok(element) = self.pop() {
            sub -= element;
        }
        self.stack.push(sub);
        Ok(())
    }

    fn prod(&mut self) -> BorthResult<()> {
        let mut prod = self.pop()?;
        while let Ok(element) = self.pop() {
            prod *= element;
        }
        self.stack.push(prod);
        Ok(())
    }

    fn div(&mut self) -> BorthResult<()> {
        let mut div = self.pop()?;
        while let Ok(element) = self.pop() {
            div /= element;
        }
        self.stack.push(div);
        Ok(())
    }

    fn dup(&mut self) -> BorthResult<()> {
        let el = self.pop()?;
        self.stack.push(el);
        self.stack.push(el);
        Ok(())
    }

    fn drop(&mut self) -> BorthResult<()> {
        self.pop()?;
        Ok(())
    }

    fn swap(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        self.stack.push(element1);
        self.stack.push(element2);
        Ok(())
    }

    fn over(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        self.stack.push(element2);
        self.stack.push(element1);
        self.stack.push(element2);
        Ok(())
    }

    fn rot(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let element3 = self.pop()?;
        self.stack.push(element2);
        self.stack.push(element1);
        self.stack.push(element3);
        Ok(())
    }

    fn eq(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = if element1 == element2 { -1 } else { 0 };
        self.stack.push(result);
        Ok(())
    }

    fn lt(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = if element1 < element2 { -1 } else { 0 };
        self.stack.push(result);
        Ok(())
    }

    fn gt(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = if element1 > element2 { -1 } else { 0 };
        self.stack.push(result);
        Ok(())
    }

    fn and(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = element1 & element2;
        self.stack.push(result);
        Ok(())
    }

    fn or(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let element2 = self.pop()?;
        let result = element1 | element2;
        self.stack.push(result);
        Ok(())
    }

    fn not(&mut self) -> BorthResult<()> {
        let element1 = self.pop()?;
        let result = !element1;
        self.stack.push(result);
        Ok(())
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

    fn run_code(code: &str) -> Stack {
        let mut interpreter = create_interpreter();
        interpreter.run_code(code).expect("Valid testing code.");
        interpreter.stack
    }

    fn delete_file_if_exists(path_to_file: &str) {
        if Path::new(path_to_file).exists() {
            let _ = remove_file(path_to_file);
        }
    }

    #[test]
    fn test01_push_to_stack() {
        let stack = run_code("1 2 3");
        assert_eq!(stack, [1, 2, 3]);
    }

    #[test]
    fn test02_perform_operation() {
        let stack = run_code("1 2 3 +");
        assert_eq!(stack, [6]);
    }

    #[test]
    fn test03_export_stack() {
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
            .filter_map(|element| element.parse::<StackElementType>().ok())
            .collect::<Stack>();
        assert_eq!(stack_read, interpreter.stack);
    }

    #[test]
    fn test04_stop_at_error() {
        let mut forth = create_interpreter();
        assert!(forth.run_code("1 2 3 UNKNOWN + 4 5 6 + ").is_err());
        assert_eq!(forth.stack, [1, 2, 3]);
    }

    #[test]
    fn test05_ignore_whitespaces() {
        let stack = run_code("1 2\n\n 3\n \n4\n 5            6");
        assert_eq!(stack, [1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test06_sum() {
        let stack = run_code("4 5 +");
        assert_eq!(stack, [9]);
    }

    #[test]
    fn test07_sub() {
        let stack = run_code("4 5 -");
        assert_eq!(stack, [1]);
    }

    #[test]
    fn test08_prod() {
        let stack = run_code("4 5 *");
        assert_eq!(stack, [20]);
    }

    #[test]
    fn test09_div() {
        let stack = run_code("4 9 /");
        assert_eq!(stack, [2]);
    }

    #[test]
    fn test10_dup() {
        let stack = run_code("5 DUP");
        assert_eq!(stack, [5, 5]);
    }

    #[test]
    fn test11_drop() {
        let stack = run_code("1 2 3 DROP");
        assert_eq!(stack, [1, 2]);
    }

    #[test]
    fn test12_swap() {
        let stack = run_code("1 2 3 SWAP");
        assert_eq!(stack, [1, 3, 2]);
    }

    #[test]
    fn test13_over() {
        let stack = run_code("1 2 3 OVER");
        assert_eq!(stack, [1, 2, 3, 2]);
    }

    #[test]
    fn test14_rot() {
        let stack = run_code("1 2 3 ROT");
        assert_eq!(stack, [2, 3, 1]);
    }

    #[test]
    fn test15_eq() {
        let stack = run_code("1 2 =");
        assert_eq!(stack, [0]);
        let stack = run_code("2 2 =");
        assert_eq!(stack, [-1]);
    }

    #[test]
    fn test16_lt() {
        let stack = run_code("1 2 <");
        assert_eq!(stack, [0]);
        let stack = run_code("2 1 <");
        assert_eq!(stack, [-1]);
    }

    #[test]
    fn test17_gt() {
        let stack = run_code("2 1 >");
        assert_eq!(stack, [0]);
        let stack = run_code("1 2 >");
        assert_eq!(stack, [-1]);
    }

    #[test]
    fn test18_and() {
        let stack = run_code("0 0 AND");
        assert_eq!(stack, [0]);
        let stack = run_code("0 -1 AND");
        assert_eq!(stack, [0]);
        let stack = run_code("-1 0 AND");
        assert_eq!(stack, [0]);
        let stack = run_code("-1 -1 AND");
        assert_eq!(stack, [-1]);
    }

    #[test]
    fn test19_or() {
        let stack = run_code("0 0 OR");
        assert_eq!(stack, [0]);
        let stack = run_code("0 -1 OR");
        assert_eq!(stack, [-1]);
        let stack = run_code("-1 0 OR");
        assert_eq!(stack, [-1]);
        let stack = run_code("-1 -1 OR");
        assert_eq!(stack, [-1]);
    }

    #[test]
    fn test19_not() {
        let stack = run_code("0 NOT");
        assert_eq!(stack, [-1]);
        let stack = run_code("-1 NOT");
        assert_eq!(stack, [0]);
    }
}
