extern crate flow;
use serde_json::{json, Value};
use flow::eval;

async fn test_eval(expr: &str, data: Value, expected: Value) {
    assert_eq!(eval(expr, data).await, expected);
}

#[tokio::test]
async fn test_basic_arithmetic() {
    test_eval("5 + 3", json!({}), json!(8.0)).await;
    test_eval("10 - 4", json!({}), json!(6.0)).await;
    test_eval("3 % 2", json!({}), json!(1.0)).await;
}

#[tokio::test]
async fn test_operator_precedence() {
    test_eval("5 + 3 * 2", json!({}), json!(11.0)).await;
  //  test_eval("(5 + 3) * 2", json!({}), json!(16.0)).await;
  //  test_eval("10 - 4 + 2", json!({}), json!(8.0)).await;
}

#[tokio::test]
async fn test_variable_substitution() {
    test_eval("$x + 5", json!({"x": 3}), json!(8.0)).await;
    test_eval("$x * $y", json!({"x": 4, "y": 2}), json!(8.0)).await;
}

#[tokio::test]
async fn test_abs_function() {
    //test_eval("abs -5", json!({}), json!(5.0)).await;
   // test_eval("abs 5", json!({}), json!(5.0)).await;
   // test_eval("5 + abs -3", json!({}), json!(8.0)).await;
}

//#[tokio::test]
//async fn test_complex_expressions() {
//    test_eval("5 + 3 * 2 - abs -4 % 3", json!({}), json!(10.0)).await;
//    test_eval("$x + $y * abs ($z - 5)", json!({"x": 1, "y": 2, "z": 3}), json!(5.0)).await;
//}
//
//#[tokio::test]
//async fn test_error_handling() {
//    assert!(matches!(eval("5 +", json!({})).await, Value::String(_)));
//    assert!(matches!(eval("5 5", json!({})).await, Value::String(_)));
//    assert!(matches!(eval("/ 5", json!({})).await, Value::String(_)));
//    assert!(matches!(eval("$x", json!({})).await, Value::String(_)));
//}
//
//#[tokio::test]
//async fn test_edge_cases() {
//    test_eval("", json!({}), Value::String("Invalid expression".to_string())).await;
//    test_eval("0", json!({}), json!(0.0)).await;
//    test_eval("abs abs abs -5", json!({}), json!(5.0)).await;
//}
