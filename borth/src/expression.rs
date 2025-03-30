pub mod function;
pub mod operation;

use crate::{context::*, dict::BorthDict, errors::*, stack::*};
use std::rc::Rc;

#[derive(Debug, PartialEq)]
pub enum BorthExpression {
    Operation(fn(&mut BorthContext) -> BorthResult<()>),
    FunctionWithWhiteSpace(fn(&mut BorthContext, &str, &str) -> BorthResult<()>),
    FunctionWithDict(fn(&mut BorthContext, &BorthDict, &str) -> BorthResult<()>),
    FunctionWithMutDict(fn(&mut BorthContext, &mut BorthDict, &str) -> BorthResult<()>),
    Number(BorthItem),
    Word(Vec<Rc<Self>>),
}
