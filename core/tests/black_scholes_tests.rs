extern crate core;

use core::models::black_scholes::BlackScholesModel;
use core::models::OptionPricingModel;

#[test]
fn test_black_scholes_call() {
    let model = BlackScholesModel;
    let s = 100.0;
    let k = 100.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let call_price = model.call_price(s, k, r, sigma, t);
    assert!((call_price - 10.45).abs() < 0.1);
}

#[test]
fn test_black_scholes_put() {
    let model = BlackScholesModel;
    let s = 100.0;
    let k = 100.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let put_price = model.put_price(s, k, r, sigma, t);
    assert!((put_price - 5.57).abs() < 0.1);
}
