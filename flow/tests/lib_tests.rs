extern crate flow;
use flow::eval;
use flow::ExpressionEvaluator;
use serde_json::json;

#[test]
fn test_eval_function() {
    let context = json!({
        "x": 10,
        "y": 5
    });

    assert_eq!(eval("x + y", &context), json!(15));
    assert_eq!(eval("x * y", &context), json!(50));
    assert_eq!(eval("x - y", &context), json!(5));
    assert_eq!(eval("x / y", &context), json!(2));
}

#[test]
fn test_expression_evaluator_error_handling() {
    let context = json!({});
    let mut evaluator = ExpressionEvaluator::new(&context);

    let result = evaluator.eval("nonexistent.field");
    assert!(
        result.is_null() || result.is_string(),
        "Expected null or error string for nonexistent field"
    );

    let result = evaluator.eval("1 +");
    assert!(
        result.is_null() || result.is_string(),
        "Expected null or error string for invalid syntax"
    );
}

#[test]
#[should_panic(expected = "Division by zero")]
fn test_expression_evaluator_division_by_zero() {
    let context = json!({});
    let mut evaluator = ExpressionEvaluator::new(&context);
    // panic
    evaluator.eval("1 / 0");
}

#[test]
fn test_expression_evaluator() {
    let context = json!({
        "person": {
            "name": "Alice",
            "age": 30
        },
        "constants": {
            "pi": 3.14159
        }
    });

    let mut evaluator = ExpressionEvaluator::new(&context);

    assert_eq!(evaluator.eval("person.name"), json!("Alice"));
    assert_eq!(evaluator.eval("person.age + 5"), json!(35));
    assert_eq!(evaluator.eval("constants.pi * 2"), json!(6.28318));
}

#[test]
fn test_number_functions() {
    let context = json!({});
    let mut evaluator = ExpressionEvaluator::new(&context);

    assert_eq!(evaluator.eval("abs(-1.23)"), json!(1.23));
    assert!(evaluator.eval("rand(100)").as_u64().unwrap() <= 100);
    assert_eq!(evaluator.eval("floor(5.9)"), json!(5));
    assert_eq!(evaluator.eval("round(5.5)"), json!(6));
    assert_eq!(evaluator.eval("ceil(5.1)"), json!(6));
    assert_eq!(evaluator.eval("number('20')"), json!(20));
    assert_eq!(evaluator.eval("isNumeric('20')"), json!(true));
    assert_eq!(evaluator.eval("isNumeric('test')"), json!(false));
}

#[test]
fn test_string_functions() {
    let context = json!({});
    let mut evaluator = ExpressionEvaluator::new(&context);

    assert_eq!(evaluator.eval("len('string')"), json!(6));
    assert_eq!(evaluator.eval("upper('string')"), json!("STRING"));
    assert_eq!(evaluator.eval("lower('StrInG')"), json!("string"));
    assert_eq!(
        evaluator.eval("startsWith('Saturday night plans', 'Sat')"),
        json!(true)
    );
    assert_eq!(
        evaluator.eval("endsWith('Saturday night plans', 'plans')"),
        json!(true)
    );
    assert_eq!(
        evaluator.eval("contains('Saturday night plans', 'night')"),
        json!(true)
    );
    assert_eq!(evaluator.eval("matches('12345', '^\\d+$')"), json!(true));
    assert_eq!(
        evaluator.eval("extract('2022-02-01', '(\\d{4})-(\\d{2})-(\\d{2})')"),
        json!(["2022-02-01", "2022", "02", "01"])
    );
}

#[test]
fn test_complex_expressions() {
    let context = json!({
             "x": 10,
            "y": 5,
            "name": "John",
            "price": 120
    });
    let mut evaluator = ExpressionEvaluator::new(&context);

    assert_eq!(evaluator.eval("100 + 100"), json!(200));
    assert_eq!(evaluator.eval("10 * 5"), json!(50));
    assert_eq!(evaluator.eval("10 ^ 2"), json!(100));
    assert_eq!(evaluator.eval("1 + 2 + 3"), json!(6));

    assert_eq!(evaluator.eval("x + y"), json!(15));
    assert_eq!(evaluator.eval("x * y"), json!(50));

    assert_eq!(
        evaluator.eval("\"hello\" + \" \" + \"world\""),
        json!("hello world")
    );
    assert_eq!(evaluator.eval("len(\"world\")"), json!(5));
    assert_eq!(evaluator.eval("upper(\"john\")"), json!("JOHN"));
    assert_eq!(evaluator.eval("lower(\"HELLO\")"), json!("hello"));

    assert_eq!(evaluator.eval("x > y"), json!(true));
    assert_eq!(evaluator.eval("x <= 10"), json!(true));

    assert_eq!(evaluator.eval("true and false"), json!(false));
    assert_eq!(evaluator.eval("true or false"), json!(true));
    assert_eq!(evaluator.eval("not false"), json!(true));

    assert_eq!(evaluator.eval("5 > 3"), json!(true));
    assert_eq!(evaluator.eval("10 <= 10"), json!(true));
    assert_eq!(evaluator.eval("\"abc\" == \"abc\""), json!(true));

    assert_eq!(evaluator.eval("true and false"), json!(false));
    assert_eq!(evaluator.eval("true or false"), json!(true));
    assert_eq!(evaluator.eval("not false"), json!(true));

    assert_eq!(evaluator.eval("abs(-5)"), json!(5));
    assert_eq!(evaluator.eval("round(3.7)"), json!(4));
    assert_eq!(evaluator.eval("floor(3.7)"), json!(3));
    assert_eq!(evaluator.eval("ceil(3.2)"), json!(4));

    // Ternary
    assert_eq!(
        evaluator.eval("price > 100 ? 'premium' : 'value'"),
        json!("premium")
    );
}

#[test]
fn test_unary_expressions() {
    let context = json!({
        "$": 5,
        "currency": "USD"
    });
    let mut evaluator = ExpressionEvaluator::new(&context);

    assert_eq!(evaluator.eval("$ == 5"), json!(true));
    assert_eq!(evaluator.eval("$ >= 1"), json!(true));
    assert_eq!(evaluator.eval("$ < 1"), json!(false));
    assert_eq!(evaluator.eval("$ >= 0 and $ <= 10"), json!(true));
    assert_eq!(evaluator.eval("$ > 0 and $ < 10"), json!(true));

    let context = json!({ "$": "USD" });
    let mut evaluator = ExpressionEvaluator::new(&context);

    assert_eq!(evaluator.eval("$ == 'USD'"), json!(true));
    assert_eq!(evaluator.eval("$ == 'EUR'"), json!(false));
    assert_eq!(evaluator.eval("startsWith($, \"US\")"), json!(true));
    assert_eq!(evaluator.eval("endsWith($, \"D\")"), json!(true));
    assert_eq!(evaluator.eval("lower($) == \"usd\""), json!(true));
}

#[test]
fn test_boolean_operations() {
    let context = json!({
        "a": true,
        "b": false,
        "x": 10,
        "y": 5
    });
    let mut evaluator = ExpressionEvaluator::new(&context);

    assert_eq!(evaluator.eval("a and b"), json!(false));
    assert_eq!(evaluator.eval("a or b"), json!(true));
    assert_eq!(evaluator.eval("!a"), json!(false));
    assert_eq!(evaluator.eval("not(b)"), json!(true));

    assert_eq!(evaluator.eval("a == true"), json!(true));
    assert_eq!(evaluator.eval("b != true"), json!(true));

    assert_eq!(evaluator.eval("(x > y) and (y < 10)"), json!(true));
    assert_eq!(evaluator.eval("(x < y) or (x == 10)"), json!(true));

    assert_eq!(evaluator.eval("bool('true')"), json!(true));
    assert_eq!(evaluator.eval("bool('false')"), json!(false));
}
