pub mod evaluator;
pub mod graph;
pub mod rule;

use serde_json::Value;

pub async fn eval(expr: &str, data: Value) -> Value {
    evaluator::eval(expr, data).await
}


