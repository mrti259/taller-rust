use crate::{context::*, errors::*};

pub fn call(ctx: &mut BorthContext) -> BorthResult<()> {
    let value1 = ctx.pop_value()?;
    let value2 = ctx.pop_value()?;
    ctx.push_value(if value2 < value1 { -1 } else { 0 })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stack::BorthItem;

    fn create_context() -> BorthContext {
        BorthContext::with_stack_size(10)
    }

    fn push_to_stack(ctx: &mut BorthContext, items: &[BorthItem]) {
        for item in items {
            let _ = ctx.push_value(*item);
        }
    }

    #[test]
    fn test1_less_true() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[1, 2]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[-1], "", &[]);
    }

    #[test]
    fn test2_less_false() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[2, 1]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[0], "", &[]);
    }

    #[test]
    fn test2_less_equals() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[2, 2]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[0], "", &[]);
    }

    #[test]
    fn test4_stack_underflow() {
        let mut ctx = create_context();
        assert_eq!(call(&mut ctx), Err(BorthError::StackUnderflow));
    }

    #[test]
    fn test5_stack_underflow() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[-1]);
        assert_eq!(call(&mut ctx), Err(BorthError::StackUnderflow));
    }
}
