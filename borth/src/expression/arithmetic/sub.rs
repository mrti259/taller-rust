use crate::{context::*, errors::*};

/// Subtract the top value on the stack from the second top value.
pub fn call(ctx: &mut BorthContext) -> BorthResult<()> {
    let value1 = ctx.pop_value()?;
    let value2 = ctx.pop_value()?;
    ctx.push_value(value2 - value1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::BorthItem;

    fn create_context() -> BorthContext {
        BorthContext::with_stack_size(10)
    }

    fn push_to_stack(ctx: &mut BorthContext, items: &[BorthItem]) {
        for item in items {
            let _ = ctx.push_value(*item);
        }
    }

    #[test]
    fn test1_sub() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[3, 4]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[-1], "");
    }

    #[test]
    fn test2_sub_with_three_items() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[1, 12, 3]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[1, 9], "");
    }

    #[test]
    fn test3_stack_underflow_empty() {
        let mut ctx = create_context();
        assert_eq!(call(&mut ctx), Err(BorthError::StackUnderflow));
    }

    #[test]
    fn test4_stack_underflow_with_one_item() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[1]);
        assert_eq!(call(&mut ctx), Err(BorthError::StackUnderflow));
    }
}
