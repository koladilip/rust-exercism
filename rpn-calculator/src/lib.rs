#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;
    let mut stack = Vec::new();
    for input in inputs {
        if let Value(i) = input {
            stack.push(*i);
            continue;
        }

        match (stack.pop(), stack.pop(), input) {
            (None, _, _) => return None,
            (_, None, _) => return None,
            // x is top of stack and y is below x so we need to compute "y op x"
            (Some(x), Some(y), Add) => stack.push(x + y),
            (Some(x), Some(y), Subtract) => stack.push(y - x),
            (Some(x), Some(y), Multiply) => stack.push(x * y),
            (Some(x), Some(y), Divide) => stack.push(y / x),
            _ => return None,
        };
    }
    return if stack.len() == 1 { stack.pop() } else { None };
}
