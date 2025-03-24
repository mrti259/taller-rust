#[derive(Clone)]
pub struct Operator {
    token: String,
}

impl Operator {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn accept(&self, token: &str) -> bool {
        token == self.token
    }

    pub fn operate(&self, stack: &mut Vec<i16>) {
        let mut sum = 0;
        while let Some(element) = stack.pop() {
            sum += element;
        }
        stack.push(sum);
    }
}

#[cfg(test)]
mod tests {
    use super::Operator;

    #[test]
    fn create_operator() {
        let sum_op = Operator::new("+".into());
        assert!(sum_op.accept("+"));
        assert!(!sum_op.accept("-"));

        let sub_op = Operator::new("-".into());
        assert!(sub_op.accept("-"));
        assert!(!sub_op.accept("+"));
    }

    #[test]
    fn sum() {
        let mut stack = [1, 2].to_vec();
        let op = Operator::new("+".into());
        op.operate(&mut stack);
        assert_eq!(stack, [3]);
    }
}
