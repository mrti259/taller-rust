use crate::{context::*, errors::*};

/// Return -1 if the top value is falsy, otherwise returns 0.
pub fn call(ctx: &mut BorthContext) -> BorthResult<()> {
    let value1 = ctx.pop_value()?;
    ctx.push_value(if value1 == 0 { -1 } else { 0 })
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
    fn test1_not_true() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[-1]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[0], "");
    }

    #[test]
    fn test2_not_false() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[0]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[-1], "");
    }

    #[test]
    fn test3_not_not() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[10]);
        assert_eq!(call(&mut ctx), Ok(()));
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[-1], "");
    }

    #[test]
    fn test4_stack_underflow_empty() {
        let mut ctx = create_context();
        assert_eq!(call(&mut ctx), Err(BorthError::StackUnderflow));
    }
}
