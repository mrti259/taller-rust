use crate::{context::*, errors::BorthResult};

pub fn call(ctx: &mut BorthContext) -> BorthResult<()> {
    ctx.print_char('\n');
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_context() -> BorthContext {
        BorthContext::with_stack_size(10)
    }

    #[test]
    fn test1_cr() {
        let mut ctx = create_context();
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[], "\n", &[]);
    }

    #[test]
    fn test2_cr() {
        let mut ctx = create_context();
        assert_eq!(call(&mut ctx), Ok(()));
        assert_eq!(call(&mut ctx), Ok(()));
        ctx.test(&[], "\n\n", &[]);
    }
}
