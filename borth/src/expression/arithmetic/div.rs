use crate::{context::*, errors::*};

/// Divide the second top value on the stack by the top value.
pub fn call(ctx: &mut BorthContext) -> BorthResult<()> {
    let value1 = ctx.pop_value()?;
    let value2 = ctx.pop_value()?;
    if value1 == 0 {
        return Err(BorthError::DivisionByZero);
    }
    ctx.push_value(value2 / value1)
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
    fn test1_div() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[12, 3]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[4], "");
    }

    #[test]
    fn test2_div() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[8, 3]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[2], "");
    }

    #[test]
    fn test3_div_with_three() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[1, 12, 3]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[1, 4], "");
    }

    #[test]
    fn test4_stack_underflow_empty() {
        let mut ctx = create_context();
        assert_eq!(call(&mut ctx), Err(BorthError::StackUnderflow));
    }

    #[test]
    fn test5_stack_underflow_with_one_item() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[1]);
        assert_eq!(call(&mut ctx), Err(BorthError::StackUnderflow));
    }

    #[test]
    fn test6_div_by_zero() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[8, 0]);
        assert_eq!(call(&mut ctx), Err(BorthError::DivisionByZero));
        ctx.test(&[], "");
    }
}
