use super::errors::*;

/// Each stack item takes 2 bytes
pub type BorthItem = i16;

/// Handle the stack and output of an interpreter execution
pub struct BorthContext {
    capacity: usize,
    items: Vec<BorthItem>,
    output: String,
}

impl BorthContext {
    /// Create a new BorthContext instance with the given stack size in bytes
    pub fn with_stack_size(stack_size: usize) -> Self {
        let capacity = stack_size / size_of::<BorthItem>();
        Self {
            capacity,
            items: Vec::with_capacity(capacity),
            output: String::new(),
        }
    }

    // data stack

    /// Pop the last item from the stack or return an error if the stack is empty
    pub fn pop_value(&mut self) -> BorthResult<BorthItem> {
        self.items.pop().ok_or(BorthError::StackUnderflow)
    }

    /// Push a new item to the stack or return an error if the stack is full
    pub fn push_value(&mut self, value: BorthItem) -> BorthResult<()> {
        if self.capacity == self.items.len() {
            return Err(BorthError::StackOverflow);
        }
        self.items.push(value);
        Ok(())
    }

    /// Returns the items from the stack as as slice
    pub fn stack_items(&self) -> &[BorthItem] {
        self.items.as_slice()
    }

    // output

    /// Push a string to the output buffer
    pub fn print(&mut self, str: &str) {
        if !self.output.is_empty() && !self.output.ends_with(char::is_whitespace) {
            self.print_char(' ');
        }
        self.output.push_str(str);
    }

    /// Push a character to the output buffer
    pub fn print_char(&mut self, char: char) {
        self.output.push(char);
    }

    /// Return the output buffer as a str slice
    pub fn output(&self) -> &str {
        &self.output
    }

    // testing

    #[allow(dead_code)]
    /// Wrap test assertions to avoid code duplication
    pub fn test(&self, stack: &[BorthItem], output: &str) {
        assert_eq!(self.stack_items(), stack);
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
    fn test05_push_when_full() {
        let mut stack = BorthContext::with_stack_size(2);
        assert!(stack.push_value(0).is_ok());
        assert_eq!(stack.push_value(0), Err(BorthError::StackOverflow));
    }

    #[test]
    fn test06_print_once() {
        let mut ctx = create_context();
        ctx.print(&"hello");
        ctx.test(&[], "hello");
    }

    #[test]
    fn test07_print_many() {
        let mut ctx = create_context();
        ctx.print(&"hello");
        ctx.print(&"world");
        ctx.test(&[], "hello world");
    }

    #[test]
    fn test08_print_many_with_new_line() {
        let mut ctx = create_context();
        ctx.print(&"hello");
        ctx.print_char('\n');
        ctx.print(&"world");
        ctx.test(&[], "hello\nworld");
    }

    #[test]
    fn test09_output_slice() {
        let mut ctx = create_context();
        ctx.print(&"hello world");
        assert_eq!(ctx.output(), "hello world");
    }

    #[test]
    fn test10_stack_items() {
        let mut ctx = create_context();
        for i in 1..5 {
            let _ = ctx.push_value(i);
        }
        assert_eq!(ctx.stack_items(), &[1, 2, 3, 4]);
    }
}
