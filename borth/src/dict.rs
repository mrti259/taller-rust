use crate::{
    expression::{arithmetic::*, booleans::*, output::*, specials::*, stack::*, *},
    parser::BorthIterator,
    stack::*,
};
use std::{collections::HashMap, rc::Rc};

pub struct BorthDict {
    words: HashMap<String, Rc<BorthExpression>>,
    words_created: Rc<BorthExpression>,
}

impl BorthDict {
    pub fn new() -> Self {
        let mut this = Self {
            words: HashMap::new(),
            words_created: Rc::new(BorthExpression::WordCreated),
        };
        this.init_words();
        this
    }

    // initialization

    fn init_words(&mut self) {
        self.add("+", BorthExpression::Operation(add::call));
        self.add("-", BorthExpression::Operation(sub::call));
        self.add("*", BorthExpression::Operation(mul::call));
        self.add("/", BorthExpression::Operation(div::call));
        self.add("dup", BorthExpression::Operation(dup::call));
        self.add("drop", BorthExpression::Operation(drop::call));
        self.add("swap", BorthExpression::Operation(swap::call));
        self.add("over", BorthExpression::Operation(over::call));
        self.add("rot", BorthExpression::Operation(rot::call));
        self.add("=", BorthExpression::Operation(eq::call));
        self.add("<", BorthExpression::Operation(lt::call));
        self.add(">", BorthExpression::Operation(gt::call));
        self.add("and", BorthExpression::Operation(and::call));
        self.add("or", BorthExpression::Operation(or::call));
        self.add("not", BorthExpression::Operation(not::call));
        self.add(".", BorthExpression::Operation(dot::call));
        self.add("emit", BorthExpression::Operation(emit::call));
        self.add("cr", BorthExpression::Operation(cr::call));
    }

    // word definition

    fn add(&mut self, token: &str, exp: BorthExpression) {
        self.words.insert(token.to_lowercase(), Rc::new(exp));
    }

    pub fn add_word(&mut self, token: &str, body: Vec<Rc<BorthExpression>>) -> Rc<BorthExpression> {
        self.add(token, BorthExpression::Word(body));
        Rc::clone(&self.words_created)
    }

    // evaluation

    pub fn detect_next(&mut self, iterator: &mut BorthIterator) -> Option<Rc<BorthExpression>> {
        while let Some((word, _)) = iterator.next() {
            if word.is_empty() {
                continue;
            }

            let expression = self.try_detect(word);
            if expression.is_some() {
                return expression;
            }

            return match word.to_lowercase().as_str() {
                ".\"" => Some(Rc::new(dot_quote::create(iterator))),
                "if" => Some(Rc::new(if_else_then::create(iterator, self))),
                ":" => Some(word_def::create(iterator, self)),
                _ => Some(Rc::new(BorthExpression::UnknownWord(word.to_string()))),
            };
        }
        None
    }

    pub fn try_detect(&self, token: &str) -> Option<Rc<BorthExpression>> {
        if let Some(word) = self.words.get(&token.to_lowercase()) {
            return Some(Rc::clone(word));
        }
        if let Ok(value) = token.parse::<BorthItem>() {
            let expression = BorthExpression::Number(value);
            return Some(Rc::new(expression));
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser;

    fn create_dict() -> BorthDict {
        BorthDict::new()
    }

    fn assert_detect(code: &str, expected: &BorthExpression) {
        let mut dict = create_dict();
        let tokens = parser::parse_tokens(code);
        let result = dict.detect_next(&mut tokens.iter());
        assert!(match result {
            Some(actual) => actual.as_ref() == expected,
            _ => false,
        });
    }

    fn assert_unknown_word(token: &str) {
        let dict = create_dict();
        assert!(dict.try_detect(token).is_none());
    }

    //arithmetic

    #[test]
    fn test_add() {
        assert_detect("+", &BorthExpression::Operation(add::call));
    }

    #[test]
    fn test_sub() {
        assert_detect("-", &BorthExpression::Operation(sub::call));
    }

    #[test]
    fn test_mul() {
        assert_detect("*", &BorthExpression::Operation(mul::call));
    }

    #[test]
    fn test_div() {
        assert_detect("/", &BorthExpression::Operation(div::call));
    }

    //stack manipulation

    #[test]
    fn test_dup() {
        assert_detect("dup", &BorthExpression::Operation(dup::call));
    }

    #[test]
    fn test_drop() {
        assert_detect("drop", &BorthExpression::Operation(drop::call));
    }

    #[test]
    fn test_swap() {
        assert_detect("swap", &BorthExpression::Operation(swap::call));
    }

    #[test]
    fn test_over() {
        assert_detect("over", &BorthExpression::Operation(over::call));
    }

    #[test]
    fn test_rot() {
        assert_detect("rot", &BorthExpression::Operation(rot::call));
    }

    //booleans

    #[test]
    fn test_eq() {
        assert_detect("=", &BorthExpression::Operation(eq::call));
    }

    #[test]
    fn test_lt() {
        assert_detect("<", &BorthExpression::Operation(lt::call));
    }

    #[test]
    fn test_gt() {
        assert_detect(">", &BorthExpression::Operation(gt::call));
    }

    #[test]
    fn test_and() {
        assert_detect("and", &BorthExpression::Operation(and::call));
    }

    #[test]
    fn test_or() {
        assert_detect("or", &BorthExpression::Operation(or::call));
    }

    #[test]
    fn test_not() {
        assert_detect("not", &BorthExpression::Operation(not::call));
    }

    //output

    #[test]
    fn test_dot() {
        assert_detect(".", &BorthExpression::Operation(dot::call));
    }

    #[test]
    fn test_emit() {
        assert_detect("emit", &BorthExpression::Operation(emit::call));
    }

    #[test]
    fn test_cr() {
        assert_detect("cr", &BorthExpression::Operation(cr::call));
    }

    #[test]
    fn test_dot_quote() {
        assert_detect(
            ".\" Hello World!\"",
            &BorthExpression::DotQuote("Hello World!".into()),
        );
    }

    // conditional

    #[test]
    fn test_if_else_then() {
        assert_unknown_word("then");
        assert_unknown_word("else");
        assert_detect("if then", &BorthExpression::IfElseThen(vec![], vec![]));
        assert_detect("if else then", &BorthExpression::IfElseThen(vec![], vec![]));
        assert_detect(
            "if 1 then",
            &BorthExpression::IfElseThen(vec![Rc::new(BorthExpression::Number(1))], vec![]),
        );
        assert_detect(
            "if else 1 then",
            &BorthExpression::IfElseThen(vec![], vec![Rc::new(BorthExpression::Number(1))]),
        );
    }

    // word definition

    #[test]
    fn test_add_word() {
        assert_unknown_word("foo");
        assert_detect(": foo 1 9 + 5 ;", &BorthExpression::WordCreated);
    }

    #[test]
    fn test_case_insensitive() {
        let body = vec![
            Rc::new(BorthExpression::Number(1)),
            Rc::new(BorthExpression::Number(9)),
            Rc::new(BorthExpression::Operation(add::call)),
            Rc::new(BorthExpression::Number(5)),
        ];
        let mut dict = create_dict();
        dict.add_word("foo", body.clone());
        assert!(match dict.try_detect("FoO") {
            Some(exp) => exp.as_ref() == &BorthExpression::Word(body),
            _ => false,
        });
    }
}
