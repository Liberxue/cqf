extern crate core;

use core::models::monte_carlo::MonteCarloModel;
use core::models::OptionPricingModel;

#[test]
fn test_monte_carlo_european_call() {
    let model = MonteCarloModel { simulations: 10000 };
    let s = 100.0;
    let k = 100.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let call_price = model.call_price(s, k, r, sigma, t);
    assert!((call_price - 10.45).abs() < 0.5);
}

#[test]
fn test_monte_carlo_european_put() {
    let model = MonteCarloModel { simulations: 10000 };
    let s = 100.0;
    let k = 100.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let put_price = model.put_price(s, k, r, sigma, t);
    assert!((put_price - 5.57).abs() < 0.5);
}
