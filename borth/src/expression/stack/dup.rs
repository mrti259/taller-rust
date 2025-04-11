use crate::{context::*, errors::*};

pub fn call(ctx: &mut BorthContext) -> BorthResult<()> {
    let value1 = ctx.pop_value()?;
    ctx.push_value(value1)?;
    ctx.push_value(value1)
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
    fn test1_dup() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[1]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[1, 1], "");
    }

    #[test]
    fn test2_dup() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[1, 2]);
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[1, 2, 2], "");
    }

    #[test]
    fn test3_stack_underflow() {
        let mut ctx = create_context();
        assert_eq!(call(&mut ctx), Err(BorthError::StackUnderflow));
    }
}
