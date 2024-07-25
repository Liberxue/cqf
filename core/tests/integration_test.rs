extern crate core;

use core::models::OptionPricingModel;
use core::models::{BlackScholesModel, OptionParameters};

#[test]
fn test_black_scholes_call() {
    let model = BlackScholesModel;
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let call_price = model.call_price(&params);
    assert!((call_price - 10.45).abs() < 0.1);
}

#[test]
fn test_black_scholes_put() {
    let model = BlackScholesModel;
    let params = OptionParameters {
        s: 100.0,
        k: 100.0,
        r: 0.05,
        sigma: 0.2,
        t: 1.0,
    };
    let put_price = model.put_price(&params);
    assert!((put_price - 5.57).abs() < 0.1);
}
