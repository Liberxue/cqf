use serde_json::Value;
use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Variable(String),
    Operator(String),
    LeftParen,
    RightParen,
}

impl FromStr for Token {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "(" => Ok(Token::LeftParen),
            ")" => Ok(Token::RightParen),
            _ => s.parse::<f64>()
                .map(Token::Number)
                .or_else(|_| {
                    if s.starts_with('$') {
                        Ok(Token::Variable(s[1..].to_string()))
                    } else {
                        Ok(Token::Operator(s.to_string()))
                    }
                })
        }
    }
}

trait Operator: Send + Sync {
    fn apply(&self, left: f64, right: Option<f64>) -> f64;
    fn precedence(&self) -> u8;
    fn is_unary(&self) -> bool {
        false
    }
}

macro_rules! impl_operator {
    ($name:ident, $op:expr, $precedence:expr) => {
        struct $name;
        impl Operator for $name {
            fn apply(&self, left: f64, right: Option<f64>) -> f64 {
                $op(left, right.unwrap_or(0.0))
            }
            fn precedence(&self) -> u8 {
                $precedence
            }
        }
    };
}

impl_operator!(AddOperator, |a, b| a + b, 1);
impl_operator!(SubtractOperator, |a, b| a - b, 1);
impl_operator!(MultiplyOperator, |a, b| a * b, 2);
impl_operator!(ModuloOperator, |a, b| a % b, 2);

struct AbsOperator;
impl Operator for AbsOperator {
    fn apply(&self, left: f64, _right: Option<f64>) -> f64 {
        left.abs()
    }
    fn precedence(&self) -> u8 {
        3
    }
    fn is_unary(&self) -> bool {
        true
    }
}

struct OperatorFactory {
    operators: HashMap<String, Box<dyn Operator>>,
}

impl OperatorFactory {
    fn new() -> Self {
        let mut operators = HashMap::new();
        operators.insert("+".to_string(), Box::new(AddOperator) as Box<dyn Operator>);
        operators.insert("-".to_string(), Box::new(SubtractOperator) as Box<dyn Operator>);
        operators.insert("*".to_string(), Box::new(MultiplyOperator) as Box<dyn Operator>);
        operators.insert("%".to_string(), Box::new(ModuloOperator) as Box<dyn Operator>);
        operators.insert("abs".to_string(), Box::new(AbsOperator) as Box<dyn Operator>);
        Self { operators }
    }

    fn get_operator(&self, op: &str) -> Option<&Box<dyn Operator>> {
        self.operators.get(op)
    }
}

struct Evaluator {
    operator_factory: OperatorFactory,
}

impl Evaluator {
    fn new() -> Self {
        Self {
            operator_factory: OperatorFactory::new(),
        }
    }

    fn evaluate(&self, expr: &str, data: &Value) -> Result<f64, String> {
        if expr.trim().is_empty() {
            return Err("Invalid expression".to_string());
        }
        let tokens = self.tokenize(expr)?;
        let postfix = self.shunting_yard(tokens)?;
        self.evaluate_postfix(postfix, data)
    }

    fn tokenize(&self, expr: &str) -> Result<Vec<Token>, String> {
        expr.split_whitespace()
            .map(|s| match s {
                "(" => Ok(Token::LeftParen),
                ")" => Ok(Token::RightParen),
                _ => Token::from_str(s),
            })
            .collect()
    }

    fn shunting_yard(&self, tokens: Vec<Token>) -> Result<Vec<Token>, String> {
        let mut output_queue = Vec::new();
        let mut operator_stack = Vec::new();
    
        for token in tokens {
            match &token {
                Token::Number(_) | Token::Variable(_) => output_queue.push(token),
                Token::Operator(op) => {
                    while let Some(Token::Operator(top_op)) = operator_stack.last() {
                        if let (Some(top), Some(current)) = (self.operator_factory.get_operator(top_op), self.operator_factory.get_operator(op)) {
                            if top.precedence() >= current.precedence() {
                                output_queue.push(operator_stack.pop().unwrap());
                            } else {
                                break;
                            }
                        } else {
                            return Err(format!("Unknown operator: {}", op));
                        }
                    }
                    operator_stack.push(token);
                }
                Token::LeftParen => operator_stack.push(token),
                Token::RightParen => {
                    while let Some(top) = operator_stack.pop() {
                        if top == Token::LeftParen {
                            break;
                        }
                        output_queue.push(top);
                    }
                }
            }
        }
    
        while let Some(op) = operator_stack.pop() {
            if op == Token::LeftParen {
                return Err("Mismatched parentheses".to_string());
            }
            output_queue.push(op);
        }
    
        Ok(output_queue)
    }
    fn evaluate_postfix(&self, postfix: Vec<Token>, data: &Value) -> Result<f64, String> {
        let mut stack = Vec::new();

        for token in postfix {
            match token {
                Token::Number(n) => stack.push(n),
                Token::Variable(v) => {
                    let value = self.get_variable(data, &v)?;
                    stack.push(value);
                }
                Token::Operator(op) => {
                    let operator = self.operator_factory.get_operator(&op)
                        .ok_or_else(|| format!("Unknown operator: {}", op))?;
                    
                    if operator.is_unary() {
                        let operand = stack.pop().ok_or("Not enough operands for unary operator")?;
                        stack.push(operator.apply(operand, None));
                    } else {
                        let right = stack.pop().ok_or("Not enough operands for binary operator")?;
                        let left = stack.pop().ok_or("Not enough operands for binary operator")?;
                        stack.push(operator.apply(left, Some(right)));
                    }
                }
                _ => return Err("Unexpected token in postfix expression".to_string()),
            }
        }

        stack.pop().ok_or("Empty expression".to_string())
    }

    fn get_variable(&self, data: &Value, var: &str) -> Result<f64, String> {
        data.get(var)
            .and_then(Value::as_f64)
            .ok_or_else(|| format!("Invalid or missing variable: {}", var))
    }
}

pub async fn eval(expr: &str, data: Value) -> Value {
    let evaluator = Evaluator::new();
    match evaluator.evaluate(expr, &data) {
        Ok(result) => Value::Number(serde_json::Number::from_f64(result).unwrap_or(serde_json::Number::from(0))),
        Err(error) => Value::String(error),
    }
}
