pub mod graph;
pub mod rule;

use zen_expression::{evaluate_expression, Isolate};
use serde_json::Value;

pub fn eval(expr: &str, data: &Value) -> Value {
    match evaluate_expression(expr, data) {
        Ok(result) => result,
        Err(error) => Value::String(error.to_string()),
    }
}

pub struct ExpressionEvaluator<'a> {
    isolate: Isolate<'a>,
}

impl<'a> ExpressionEvaluator<'a> {
    pub fn new(context: &'a Value) -> Self {
        Self {
            isolate: Isolate::with_environment(context),
        }
    }

    pub fn eval(&mut self, expr: &'a str) -> Value {
        match self.isolate.run_standard(expr) {
            Ok(result) => result,
            Err(error) => Value::String(error.to_string()),
        }
    }
}
