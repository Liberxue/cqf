extern crate flow;
use flow::eval;
use flow::ExpressionEvaluator;
use serde_json::json;
use std::f64::consts;

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
    assert_eq!(evaluator.eval("1 / 0"), "");
}

#[test]
fn test_expression_evaluator() {
    let context = json!({
        "person": {
            "name": "Alice",
            "age": 30
        },
        "constants": {
            "pi": consts::PI
        }
    });

    let mut evaluator = ExpressionEvaluator::new(&context);

    assert_eq!(evaluator.eval("person.name"), json!("Alice"));
    assert_eq!(evaluator.eval("person.age + 5"), json!(35));
    assert_eq!(evaluator.eval("constants.pi * 2"), json!(consts::TAU));
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

#[test]
fn test_datatime_operations() {
    let context = json!({});

    let mut evaluator = ExpressionEvaluator::new(&context);

    assert_eq!(evaluator.eval("date('2024-01-01')"), json!(1704067200));

    assert_eq!(evaluator.eval("weekOfYear('2024-09-18')"), json!(38));

    assert_eq!(
        evaluator.eval("date('2024-04-04T21:48:30Z')"),
        json!(1712267310)
    );

    assert_eq!(evaluator.eval("year('2024-09-18')"), json!(2024));
    //
    assert_eq!(evaluator.eval("time('21:49')"), json!(78540));

    assert_eq!(evaluator.eval("time('21:48:20')"), json!(78500));

    assert_eq!(evaluator.eval("duration('30m')"), json!(1800));
    assert_eq!(evaluator.eval("duration('72h')"), json!(259200));

    assert_eq!(evaluator.eval("dayOfMonth(date('2022-11-09'))"), json!(9));

    assert_eq!(evaluator.eval("dayOfYear(date('2022-11-10'))"), json!(314));

    assert_eq!(
        evaluator.eval("weekdayString(date('2022-11-14'))"),
        json!("Mon")
    );

    // https://github.com/gorules/zen/pull/91/files#diff-0351bd90a8ac5af79cbd69797085dae5d37782fb4359797dd72a801485f133a2R207
    assert_eq!(
        evaluator.eval("dateString(startOf('2024-01-01 00:00:00', 'day'))"),
        json!("2024-01-01 00:00:00")
    );
}

#[test]
fn test_array_operations() {
    let context = json!({
        "code": 0,
        "data": {
            "array": [
                1,
                8,
                9,
                10,
                30
            ],
            "map": {
                "subMap": {
                    "status": 1
                }
            }
        }
    });
    //https://github.com/gorules/zen/blob/17b468f50755f448a298c60ff2af8eb1856fd0bb/core/expression/tests/data/standard.csv
    let mut eveluator = ExpressionEvaluator::new(&context);
    assert_eq!(eveluator.eval("len([1, 2, 3, 4, 5])"), json!(5));
    assert_eq!(eveluator.eval("sum([1, 2, 3, 4, 5])"), json!(15));
    assert_eq!(
        eveluator.eval("map([1, 2, 3, 4, 5], # ^ 2)"),
        json!([1, 4, 9, 16, 25])
    );

    assert_eq!(
        eveluator.eval("filter([0.13,1, 2, 3, 4, 5], # > 3)"),
        json!([4, 5])
    );

    assert_eq!(eval("filter(data.array,# > 9)", &context), json!([10, 30]));
    assert_eq!(eval("data.map.subMap.status > 0", &context), json!(true));

    let context = json!([
        {
            "id": "1",
            "price": [
                {
                    "type": 0,
                    "val": 420_222_111
                },
                {
                    "type": 1,
                    "val": 2300
                },
                {
                    "type": 2,
                    "val": 1300
                },
                {
                    "type": 3,
                    "val": 4211
                },
            ],
            "type": 1
        },
        {
            "id": "2",
            "price": [
                {
                    "type": 1,
                    "val": 2300
                },
                {
                    "type": 2,
                    "val": 1300
                },
                {
                    "type": 3,
                    "val": 4211
                }
            ],
            "type": 2
        }
    ]);
    let expr = r#"
            map(
                filter($root, #.type == 1 or #.type == 2),
                {
                    id: #.id,
                    type1_val: filter(#.price, #.type == 1)[0].val,
                    type2_val: filter(#.price, #.type == 2)[0].val
                }
            )
        "#;

    let expected = json!([
        {
            "id": "1",
            "type1_val": 2300,
            "type2_val": 1300
        },
        {
            "id": "2",
            "type1_val": 2300,
            "type2_val": 1300
        }
    ]);

    assert_eq!(eval(expr, &context), expected);
}
