extern crate cqf;

use cqf::core::models::black_scholes::BlackScholesModel;
use cqf::core::strategies::single_leg::SingleLegOption;
use cqf::core::strategies::OptionStrategy;

#[test]
fn test_single_leg_call() {
    let model = BlackScholesModel;
    let s = 100.0;
    let k = 100.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let single_leg_call = SingleLegOption::new(&model, s, k, r, sigma, t, true);
    let price = single_leg_call.price();
    assert!((price - 10.45).abs() < 0.1);
}

#[test]
fn test_single_leg_put() {
    let model = BlackScholesModel;
    let s = 100.0;
    let k = 100.0;
    let r = 0.05;
    let sigma = 0.2;
    let t = 1.0;
    let single_leg_put = SingleLegOption::new(&model, s, k, r, sigma, t, false);
    let price = single_leg_put.price();
    assert!((price - 5.57).abs() < 0.1);
}
