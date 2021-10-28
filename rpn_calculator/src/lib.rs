#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

struct MyStack {
    vec: Vec<i32>,
}

impl MyStack {
    pub fn new() -> Self {
        MyStack { vec: vec![] }
    }
    fn calc(&mut self, input: &CalculatorInput) -> Option<i32> {
        match input {
            &CalculatorInput::Value(val) => Some(val),
            _ => {
                if self.vec.len() < 2 {
                    return None;
                }
                let right = self.vec.pop().unwrap();
                let left = self.vec.pop().unwrap();
                let new_val = match input {
                    CalculatorInput::Add => left + right,
                    CalculatorInput::Subtract => left - right,
                    CalculatorInput::Multiply => left * right,
                    CalculatorInput::Divide => left / right,
                    _ => panic!("val_op was Value"),
                };
                return Some(new_val);
            }
        }
    }

    fn push(&mut self, num: i32) {
        self.vec.push(num);
    }

    fn pop(&mut self) -> Option<i32> {
        self.vec.pop()
    }

    fn len(&self) -> usize {
        self.vec.len()
    }
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut s = MyStack::new();

    for val_op in inputs {
        if let Some(new_val) = s.calc(val_op) {
            s.push(new_val);
        } else {
            return None;
        }
    }
    if s.len() != 1 {
        None
    } else {
        s.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn simple_expression_add() {
        let input = &[
            CalculatorInput::Value(1),
            CalculatorInput::Value(2),
            CalculatorInput::Add,
        ];
        let result = evaluate(input).unwrap();
        assert_eq!(result, 1 + 2);
    }

    #[test]
    fn simple_expression_sub() {
        let input = &[
            CalculatorInput::Value(3),
            CalculatorInput::Value(1),
            CalculatorInput::Subtract,
        ];
        let result = evaluate(input).unwrap();
        assert_eq!(result, 3 - 1);
    }

    #[test]
    fn simple_expression_mul() {
        let input = &[
            CalculatorInput::Value(2),
            CalculatorInput::Value(4),
            CalculatorInput::Multiply,
        ];
        let result = evaluate(input).unwrap();
        assert_eq!(result, 2 * 4);
    }

    #[test]
    fn simple_expression_div() {
        let input = &[
            CalculatorInput::Value(6),
            CalculatorInput::Value(3),
            CalculatorInput::Divide,
        ];
        let result = evaluate(input).unwrap();
        assert_eq!(result, 6 / 3);
    }

    #[test]
    fn expression1() {
        let input = &[
            // 4 8 + 7 5 -
            CalculatorInput::Value(4),
            CalculatorInput::Value(8),
            CalculatorInput::Add,
            CalculatorInput::Value(7),
            CalculatorInput::Value(5),
            CalculatorInput::Subtract,
            CalculatorInput::Divide,
        ];
        let result = evaluate(input).unwrap();
        assert_eq!(result, (4 + 8) / (7 - 5));
    }

    #[test]
    fn too_many_operators_return_none() {
        let input = &[
            CalculatorInput::Value(6),
            CalculatorInput::Value(3),
            CalculatorInput::Divide,
            CalculatorInput::Subtract,
        ];
        let result = evaluate(input);
        assert_eq!(result, None);
    }

    #[test]
    fn too_few_operators_return_none() {
        let input = &[CalculatorInput::Value(6), CalculatorInput::Value(3)];
        let result = evaluate(input);
        assert_eq!(result, None);
    }
}
