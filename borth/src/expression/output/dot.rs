use crate::{context::*, errors::*};

pub fn call(ctx: &mut BorthContext) -> BorthResult<()> {
    let item1 = ctx.pop_value()?;
    ctx.print(&item1.to_string());
    Ok(())
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
    fn test1_dot_without_leftover() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[1, 2]);
        assert_eq!(call(&mut ctx), Ok(()));
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[], "2 1");
    }

    #[test]
    fn test2_dot_with_leftover() {
        let mut ctx = create_context();
        push_to_stack(&mut ctx, &[1, 2, 3, 4, 5]);
        assert_eq!(call(&mut ctx), Ok(()));
        assert_eq!(call(&mut ctx), Ok(()));
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[1, 2], "5 4 3");
    }

    #[test]
    fn test3_stack_underflow() {
        let mut ctx = create_context();
        assert_eq!(call(&mut ctx), Err(BorthError::StackUnderflow));
    }
}
