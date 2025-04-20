use crate::{errors::*, stack::*};

#[derive(Debug)]
pub struct BorthContext {
    stack: BorthStack,
    output: String,
}

impl BorthContext {
    pub fn with_stack_size(stack_size: usize) -> Self {
        Self {
            stack: BorthStack::with_size(stack_size),
            output: String::new(),
        }
    }

    // data stack

    pub fn pop_value(&mut self) -> BorthResult<BorthItem> {
        self.stack.pop()
    }

    pub fn push_value(&mut self, value: BorthItem) -> BorthResult<()> {
        self.stack.push(value)
    }

    pub fn stack_items(&self) -> &[BorthItem] {
        self.stack.items()
    }

    // output

    pub fn print(&mut self, sth: &str) {
        if !self.output.is_empty() && !self.output.ends_with(char::is_whitespace) {
            self.print_char(' ');
        }
        self.output.push_str(sth);
    }

    pub fn print_char(&mut self, ch: char) {
        self.output.push(ch);
    }

    pub fn output(&self) -> &str {
        &self.output
    }

    // testing

    #[allow(dead_code)]
    pub fn test(&self, stack: &[BorthItem], output: &str) {
        assert_eq!(self.stack.items(), stack);
        assert_eq!(self.output, output);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_context() -> BorthContext {
        BorthContext::with_stack_size(10)
    }

    #[test]
    fn test01_new() {
        let ctx = create_context();
        ctx.test(&[], "");
    }

    #[test]
    fn test02_pop_empty() {
        let mut ctx = create_context();
        assert_eq!(ctx.pop_value(), Err(BorthError::StackUnderflow));
        ctx.test(&[], "");
    }

    #[test]
    fn test03_push_to_stack() {
        let mut ctx = create_context();
        assert_eq!(ctx.push_value(1), Ok(()));
        ctx.test(&[1], "");
        assert_eq!(ctx.pop_value(), Ok(1));
        ctx.test(&[], "");
    }

    #[test]
    fn test04_push_many_to_stack() {
        let mut ctx = create_context();
        assert_eq!(ctx.push_value(1), Ok(()));
        assert_eq!(ctx.push_value(2), Ok(()));
        ctx.test(&[1, 2], "");
        assert_eq!(ctx.pop_value(), Ok(2));
        assert_eq!(ctx.pop_value(), Ok(1));
        assert_eq!(ctx.pop_value(), Err(BorthError::StackUnderflow));
        ctx.test(&[], "");
    }

    #[test]
    fn test05_print_once() {
        let mut ctx = create_context();
        ctx.print(&"hello");
        ctx.test(&[], "hello");
    }

    #[test]
    fn test06_print_many() {
        let mut ctx = create_context();
        ctx.print(&"hello");
        ctx.print(&"world");
        ctx.test(&[], "hello world");
    }

    #[test]
    fn test07_print_many_with_new_line() {
        let mut ctx = create_context();
        ctx.print(&"hello");
        ctx.print_char('\n');
        ctx.print(&"world");
        ctx.test(&[], "hello\nworld");
    }

    #[test]
    fn test08_output_slice() {
        let mut ctx = create_context();
        ctx.print(&"hello world");
        assert_eq!(ctx.output(), "hello world");
    }

    #[test]
    fn test09_stack_items() {
        let mut ctx = create_context();
        for i in 1..5 {
            let _ = ctx.push_value(i);
        }
        assert_eq!(ctx.stack_items(), &[1, 2, 3, 4]);
    }
}
