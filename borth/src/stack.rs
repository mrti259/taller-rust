use crate::errors::*;

pub struct BorthStack<Item> {
    capacity: usize,
    items: Vec<Item>,
}

impl<Item> BorthStack<Item> {
    pub fn with_capacity(capacity: usize) -> Self {
        BorthStack {
            capacity,
            items: Vec::with_capacity(capacity),
        }
    }

    pub fn pop(&mut self) -> BorthResult<Item> {
        self.items.pop().ok_or(BorthError::StackUnderflow)
    }

    pub fn push(&mut self, item: Item) -> BorthResult<()> {
        if self.capacity == self.items.len() {
            return Err(BorthError::StackOverflow);
        }
        self.items.push(item);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn reverse(&mut self) {
        self.items.reverse();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    type Item = i16;

    fn create_stack(capacity: usize) -> BorthStack<Item> {
        BorthStack::with_capacity(capacity)
    }

    fn assert_pop_items(stack: &mut BorthStack<Item>, items: &[Item]) {
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

    #[test]
    fn test4_len() {
        let mut stack = create_stack(5);
        assert_eq!(stack.len(), 0);
        assert!(stack.push(0).is_ok());
        assert_eq!(stack.len(), 1);
        assert!(stack.push(0).is_ok());
        assert_eq!(stack.len(), 2);
    }

    #[test]
    fn test5_reverse_stack() {
        let mut stack = create_stack(3);
        let _ = stack.push(0);
        let _ = stack.push(1);
        let _ = stack.push(2);
        stack.reverse();
        assert_pop_items(&mut stack, &[0, 1, 2]);
    }
}
