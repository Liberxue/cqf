extern crate flow;
use serde_json::json;
use flow::eval;

 #[tokio::test]
 async fn test_eval() {
     let result = eval("5 + 3", json!({})).await;
     assert_eq!(result, json!(8.0));

     let result = eval("$x * 2", json!({"x": 5})).await;
     assert_eq!(result, json!(10.0));

     let result = eval("invalid expression", json!({})).await;
     assert!(result.is_string());
 }
