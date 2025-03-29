use crate::{errors::*, stack::*};
use std::{fmt::Display, io::Write};

pub struct BorthInterpreter<Output: Write> {
    stack: BorthStack,
    output: Output,
}

impl<Output: Write> BorthInterpreter<Output> {
    pub fn with_stack_size(size: usize, output: Output) -> Self {
        Self {
            stack: BorthStack::with_capacity(size),
            output,
        }
    }

    pub fn export_stack_to<File: Write>(&self, file: &mut File) -> BorthResult<()> {
        let items = self.stack.items();
        let len = items.len();
        for i in 0..len {
            let mut buf = items[i].to_string();
            if i < len - 1 {
                buf.push(' ');
            }
            if file.write(buf.as_bytes()).is_err() {
                return Err(BorthError::CanNotWriteFile);
            }
        }
        Ok(())
    }

    pub fn run_code(&mut self, code: &str) -> BorthResult<()> {
        let mut op_stack = Vec::<String>::new();
        let mut whitespaces = code.match_indices(char::is_whitespace);
        let mut offset = 0;
        while offset < code.len() {
            let (stop, whitespace) = match whitespaces.next() {
                Some(result) => result,
                None => (code.len(), ""),
            };
            let token = match code.get(offset..stop) {
                Some(token) => token,
                None => code,
            };
            self.detect_operation(token, whitespace, &mut op_stack)?;
            offset = stop + 1;
        }
        Ok(())
    }

    fn detect_operation(
        &mut self,
        token: &str,
        whitespace: &str,
        op_stack: &mut Vec<String>,
    ) -> BorthResult<()> {
        if let Some(last_op) = op_stack.last() {
            match last_op.as_str() {
                ".\"" => {
                    if token.ends_with("\"") {
                        op_stack.pop();
                        self.print(token.trim_end_matches("\""))?;
                    } else {
                        self.print(token)?;
                        self.print(whitespace)?;
                    }
                    return Ok(());
                }
                "IF" => match token.to_uppercase().as_str() {
                    "THEN" => {
                        self.stack.pop()?;
                        op_stack.pop();
                        return Ok(());
                    }
                    "ELSE" => {
                        let item = self.stack.pop()?;
                        op_stack.push("ELSE".into());
                        return self.stack.push(item);
                    }
                    _ => {
                        let item = self.stack.pop()?;
                        if item != 0 {
                            self.detect_word(token, op_stack)?;
                        }
                        return self.stack.push(item);
                    }
                },
                "ELSE" => match token.to_uppercase().as_str() {
                    "THEN" => {
                        self.stack.pop()?;
                        op_stack.pop();
                        op_stack.pop();
                        return Ok(());
                    }
                    _ => {
                        let item = self.stack.pop()?;
                        if item == 0 {
                            self.detect_word(token, op_stack)?;
                        }
                        return self.stack.push(item);
                    }
                },
                _ => {}
            }
        }
        self.detect_word(token, op_stack)
    }

    fn detect_word(&mut self, token: &str, op_stack: &mut Vec<String>) -> BorthResult<()> {
        match token.trim().to_uppercase().as_str() {
            "" => Ok(()),
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
            "." => self.dot(),
            "EMIT" => self.emit(),
            "CR" => self.print("\n"),
            ".\"" => self.pstring(token, op_stack),
            "IF" => {
                op_stack.push("IF".into());
                Ok(())
            }
            _ => self.push(token),
        }
    }

    fn print<T: Display>(&mut self, sth: T) -> BorthResult<()> {
        if write!(self.output, "{}", sth).is_err() {
            return Err(BorthError::CanNotWriteToOutput);
        }
        Ok(())
    }

    fn push(&mut self, token: &str) -> BorthResult<()> {
        match token.parse::<BorthItem>() {
            Ok(item) => self.stack.push(item),
            _ => Err(BorthError::UnknownWord),
        }
    }

    fn sum(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = item1 + item2;
        self.stack.push(result)
    }

    fn sub(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = item1 - item2;
        self.stack.push(result)
    }

    fn prod(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = item1 * item2;
        self.stack.push(result)
    }

    fn div(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = item1 / item2;
        self.stack.push(result)
    }

    fn dup(&mut self) -> BorthResult<()> {
        let item = self.stack.pop()?;
        self.stack.push(item)?;
        self.stack.push(item)
    }

    fn drop(&mut self) -> BorthResult<()> {
        self.stack.pop()?;
        Ok(())
    }

    fn swap(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        self.stack.push(item1)?;
        self.stack.push(item2)
    }

    fn over(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        self.stack.push(item2)?;
        self.stack.push(item1)?;
        self.stack.push(item2)
    }

    fn rot(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let item3 = self.stack.pop()?;
        self.stack.push(item2)?;
        self.stack.push(item1)?;
        self.stack.push(item3)
    }

    fn eq(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = if item1 == item2 { -1 } else { 0 };
        self.stack.push(result)
    }

    fn lt(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = if item1 < item2 { -1 } else { 0 };
        self.stack.push(result)
    }

    fn gt(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = if item1 > item2 { -1 } else { 0 };
        self.stack.push(result)
    }

    fn and(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = item1 & item2;
        self.stack.push(result)
    }

    fn or(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let item2 = self.stack.pop()?;
        let result = item1 | item2;
        self.stack.push(result)
    }

    fn not(&mut self) -> BorthResult<()> {
        let item1 = self.stack.pop()?;
        let result = !item1;
        self.stack.push(result)
    }

    fn dot(&mut self) -> BorthResult<()> {
        let item = self.stack.pop()?;
        self.print(item)
    }

    fn emit(&mut self) -> BorthResult<()> {
        let item = self.stack.pop()?;
        let ascii = char::from_u32(item as u32).ok_or(BorthError::RuntimeError)?;
        self.print(ascii)
    }

    fn pstring(&self, token: &str, op_stack: &mut Vec<String>) -> BorthResult<()> {
        op_stack.push(token.to_string());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{Cursor, Read};

    type Output = Cursor<Vec<u8>>;

    fn create_interpreter() -> BorthInterpreter<Output> {
        BorthInterpreter::with_stack_size(10, Cursor::new(Vec::new()))
    }

    fn assert_stack_equals(interpreter: &BorthInterpreter<Output>, items: &[BorthItem]) {
        assert_eq!(interpreter.stack.items(), items);
    }

    fn run_code_and_assert_stack_equals(code: &str, stack: &[BorthItem]) {
        let mut interpreter = create_interpreter();
        interpreter.run_code(code).expect("Valid testing code");
        assert_stack_equals(&interpreter, stack);
    }

    fn run_code_and_assert_output_equals(code: &str, output: &str) {
        let mut interpreter = create_interpreter();
        interpreter.run_code(code).expect("Valid testing code");
        let mut buf = Default::default();
        interpreter.output.set_position(0);
        assert!(interpreter.output.read_to_string(&mut buf).is_ok());
        assert_eq!(buf, output);
    }

    #[test]
    fn test01_push_to_stack() {
        run_code_and_assert_stack_equals("1 2 3", &[1, 2, 3]);
    }

    #[test]
    fn test02_perform_operation() {
        run_code_and_assert_stack_equals("1 2 +", &[3]);
    }

    #[test]
    fn test03_export_stack() {
        let mut interpreter = create_interpreter();
        assert!(interpreter.run_code("1 2 3 + 1 2").is_ok());

        let mut file = Cursor::new(Vec::new());
        assert!(interpreter.export_stack_to(&mut file).is_ok());

        let mut buf = Default::default();
        let _ = file.set_position(0);
        let _ = file.read_to_string(&mut buf);
        assert_eq!(buf, "1 5 1 2");
    }

    #[test]
    fn test04_stop_at_error() {
        let mut interpreter = create_interpreter();
        assert!(interpreter.run_code("1 2 3 UNKNOWN + 4 5 6 + ").is_err());
        assert_stack_equals(&interpreter, &[1, 2, 3]);
    }

    #[test]
    fn test05_ignore_whitespaces() {
        run_code_and_assert_stack_equals("1 2\n\n 3\n \n4\n 5            6", &[1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test06_sum() {
        run_code_and_assert_stack_equals("4 5 +", &[9]);
    }

    #[test]
    fn test07_sub() {
        run_code_and_assert_stack_equals("4 5 -", &[1]);
    }

    #[test]
    fn test08_prod() {
        run_code_and_assert_stack_equals("4 5 *", &[20]);
    }

    #[test]
    fn test09_div() {
        run_code_and_assert_stack_equals("4 9 /", &[2]);
    }

    #[test]
    fn test10_dup() {
        run_code_and_assert_stack_equals("5 DUP", &[5, 5]);
    }

    #[test]
    fn test11_drop() {
        run_code_and_assert_stack_equals("1 2 3 DROP", &[1, 2]);
    }

    #[test]
    fn test12_swap() {
        run_code_and_assert_stack_equals("1 2 3 SWAP", &[1, 3, 2]);
    }

    #[test]
    fn test13_over() {
        run_code_and_assert_stack_equals("1 2 3 OVER", &[1, 2, 3, 2]);
    }

    #[test]
    fn test14_rot() {
        run_code_and_assert_stack_equals("1 2 3 ROT", &[2, 3, 1]);
    }

    #[test]
    fn test15_eq() {
        run_code_and_assert_stack_equals("1 2 =", &[0]);
        run_code_and_assert_stack_equals("2 2 =", &[-1]);
    }

    #[test]
    fn test16_lt() {
        run_code_and_assert_stack_equals("1 2 <", &[0]);
        run_code_and_assert_stack_equals("2 1 <", &[-1]);
    }

    #[test]
    fn test17_gt() {
        run_code_and_assert_stack_equals("2 1 >", &[0]);
        run_code_and_assert_stack_equals("1 2 >", &[-1]);
    }

    #[test]
    fn test18_and() {
        run_code_and_assert_stack_equals("0 0 AND", &[0]);
        run_code_and_assert_stack_equals("0 -1 AND", &[0]);
        run_code_and_assert_stack_equals("-1 0 AND", &[0]);
        run_code_and_assert_stack_equals("-1 -1 AND", &[-1]);
    }

    #[test]
    fn test19_or() {
        run_code_and_assert_stack_equals("0 0 OR", &[0]);
        run_code_and_assert_stack_equals("0 -1 OR", &[-1]);
        run_code_and_assert_stack_equals("-1 0 OR", &[-1]);
        run_code_and_assert_stack_equals("-1 -1 OR", &[-1]);
    }

    #[test]
    fn test19_not() {
        run_code_and_assert_stack_equals("0 NOT", &[-1]);
        run_code_and_assert_stack_equals("-1 NOT", &[0]);
    }

    #[test]
    fn test20_dot() {
        let code = "0 .";
        run_code_and_assert_stack_equals(code, &[]);
        run_code_and_assert_output_equals(code, "0");
    }

    #[test]
    fn test21_emit() {
        let code = "33 119 111 87 emit emit emit emit";
        run_code_and_assert_stack_equals(code, &[]);
        run_code_and_assert_output_equals(code, "Wow!");
    }

    #[test]
    fn test22_emit() {
        run_code_and_assert_output_equals("CR", "\n");
    }

    #[test]
    fn test23_output_string() {
        run_code_and_assert_output_equals(".\" Hello World!\"", "Hello World!");
    }

    #[test]
    fn test24_if_then() {
        run_code_and_assert_stack_equals("0 IF 1 THEN", &[]);
        run_code_and_assert_stack_equals("0 IF 1 2 + THEN", &[]);
        run_code_and_assert_stack_equals("-1 IF 2 THEN", &[2]);
        run_code_and_assert_stack_equals("-1 IF 1 2 + THEN", &[3]);
    }

    #[test]
    fn test25_if_else_then() {
        run_code_and_assert_stack_equals("0 IF 1 ELSE 2 THEN", &[2]);
        run_code_and_assert_stack_equals("0 IF 1 ELSE 1 2 + THEN", &[3]);
        run_code_and_assert_stack_equals("-1 IF ELSE 3 THEN", &[]);
        run_code_and_assert_stack_equals("-1 IF ELSE 1 2 3 + THEN", &[]);
    }
}
