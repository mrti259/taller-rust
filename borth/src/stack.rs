use crate::errors::*;

pub type BorthItem = i16;

#[derive(Debug)]
pub struct BorthStack {
    capacity: usize,
    items: Vec<BorthItem>,
}

impl BorthStack {
    pub fn with_size(size: usize) -> Self {
        let capacity = size / size_of::<BorthItem>();
        BorthStack {
            capacity,
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn pop(&mut self) -> BorthResult<BorthItem> {
        self.items.pop().ok_or(BorthError::StackUnderflow)
    }

    pub fn push(&mut self, value: BorthItem) -> BorthResult<()> {
        if self.capacity == self.items.len() {
            return Err(BorthError::StackOverflow);
        }
        self.items.push(value);
        Ok(())
    }

    pub fn items(&self) -> &[BorthItem] {
        self.items.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_stack(capacity: usize) -> BorthStack {
        BorthStack::with_size(capacity * size_of::<BorthItem>())
    }

    fn assert_pop_items(stack: &mut BorthStack, items: &[BorthItem]) {
        for item in items {
            if let Ok(value) = stack.pop() {
                assert_eq!(value, *item);
            } else {
                assert!(false)
            }
        }
        assert!(match stack.pop() {
            Err(BorthError::StackUnderflow) => true,
            _ => false,
        })
    }

    #[test]
    fn test1_pop_when_empty() {
        let mut stack = create_stack(1);
        assert_pop_items(&mut stack, &[]);
    }

    #[test]
    fn test2_push_item() {
        let mut stack = create_stack(1);
        assert!(stack.push(0).is_ok());
        assert_pop_items(&mut stack, &[0]);
    }

    #[test]
    fn test3_push_many_items() {
        let mut stack = create_stack(3);
        assert!(stack.push(0).is_ok());
        assert!(stack.push(1).is_ok());
        assert!(stack.push(2).is_ok());
        assert_pop_items(&mut stack, &[2, 1, 0]);
    }

    #[test]
    fn test4_push_when_full() {
        let mut stack = create_stack(1);
        assert!(stack.push(0).is_ok());
        assert!(match stack.push(0) {
            Err(BorthError::StackOverflow) => true,
            _ => false,
        })
    }
}
