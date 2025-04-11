use super::BorthExpression;
use crate::{context::BorthContext, dict::BorthDict, errors::*, parser::*, stack::BorthItem};
use std::rc::Rc;

pub fn create(iterator: &mut BorthIterator, dict: &mut BorthDict) -> Rc<BorthExpression> {
    match iterator.next() {
        None => Rc::new(BorthExpression::InvalidWord),
        Some((word, _)) => {
            if word.parse::<BorthItem>().is_ok() {
                return Rc::new(BorthExpression::InvalidWord);
            }

            let mut body = vec![];
            while let Some(exp) = dict.detect_next(iterator) {
                match exp.as_ref() {
                    BorthExpression::UnknownWord(word) => {
                        if word == ";" {
                            break;
                        }
                        body.push(exp);
                    }
                    _ => body.push(exp),
                }
            }
            if body.is_empty() {
                return Rc::new(BorthExpression::InvalidWord);
            }
            dict.add_word(word, body)
        }
    }
}

pub fn call(ctx: &mut BorthContext, body: &Vec<Rc<BorthExpression>>) -> BorthResult<()> {
    for exp in body {
        exp.eval(ctx)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::expression::{BorthExpression, arithmetic::mul, stack::dup};
    use std::rc::Rc;

    fn create_dict() -> BorthDict {
        BorthDict::new()
    }

    fn assert_create(code: &str, dict: &mut BorthDict, expected: &BorthExpression) {
        let tokens = parse_tokens(code);
        assert_eq!(create(&mut tokens.iter(), dict).as_ref(), expected);
    }

    fn assert_create_word(
        word: &str,
        code: &str,
        dict: &mut BorthDict,
        expected: &BorthExpression,
    ) {
        assert_create(code, dict, &BorthExpression::WordCreated);
        assert_eq!(dict.detect(word).as_ref(), expected);
    }

    #[test]
    fn test1_word_def() {
        let mut dict = create_dict();
        assert_create_word(
            "foo",
            "foo 1 ;",
            &mut dict,
            &BorthExpression::Word(vec![Rc::new(BorthExpression::Number(1))]),
        );
    }

    #[test]
    fn test2_invalid_word() {
        let mut dict = create_dict();
        assert_create("foo ;", &mut dict, &BorthExpression::InvalidWord);
        assert_eq!(
            dict.detect("foo").as_ref(),
            &BorthExpression::UnknownWord("foo".into())
        );
    }

    #[test]
    fn test3_invalid_word() {
        let mut dict = create_dict();
        assert_create("1 1 ;", &mut dict, &BorthExpression::InvalidWord);
    }

    #[test]
    fn test4_word_def() {
        let mut dict = create_dict();
        assert_create_word(
            "dup-twice",
            "dup-twice dup dup ;",
            &mut dict,
            &BorthExpression::Word(vec![
                Rc::new(BorthExpression::Operation(dup::call)),
                Rc::new(BorthExpression::Operation(dup::call)),
            ]),
        );
    }

    #[test]
    fn test5_word_def() {
        let mut dict = create_dict();
        assert_create_word(
            "countup",
            "countup 1 2 3 ;",
            &mut dict,
            &BorthExpression::Word(vec![
                Rc::new(BorthExpression::Number(1)),
                Rc::new(BorthExpression::Number(2)),
                Rc::new(BorthExpression::Number(3)),
            ]),
        );
    }

    #[test]
    fn test6_word_redefinition() {
        let mut dict = create_dict();
        assert_create_word(
            "foo",
            "foo dup ;",
            &mut dict,
            &BorthExpression::Word(vec![Rc::new(BorthExpression::Operation(dup::call))]),
        );
        assert_create_word(
            "foo",
            "foo dup dup ;",
            &mut dict,
            &BorthExpression::Word(vec![
                Rc::new(BorthExpression::Operation(dup::call)),
                Rc::new(BorthExpression::Operation(dup::call)),
            ]),
        );
    }

    #[test]
    fn test7_shadowing() {
        let mut dict = create_dict();
        assert_create_word(
            "swap",
            "swap dup ;",
            &mut dict,
            &BorthExpression::Word(vec![Rc::new(BorthExpression::Operation(dup::call))]),
        );
    }

    #[test]
    fn test8_shadowing_symbol() {
        let mut dict = create_dict();
        assert_create_word(
            "+",
            "+ * ;",
            &mut dict,
            &BorthExpression::Word(vec![Rc::new(BorthExpression::Operation(mul::call))]),
        );
    }

    #[test]
    fn test_hello_world() {
        let mut dict = create_dict();
        assert_create_word(
            "hello",
            "hello .\" hello world\" ;",
            &mut dict,
            &BorthExpression::Word(vec![Rc::new(BorthExpression::DotQuote(
                "hello world".into(),
            ))]),
        );
    }
}
